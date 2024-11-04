use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{BOOL_SIG, INT_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::string::AndroidString;
use cloudcore::properties::value::IoTPropertyValue;
use cloudcore::properties::value::IoTPropertyValue::{Bool, Int, Str};
use ctor::ctor;
use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;
use crate::JAVA_PACKAGE;

pub static PROPERTY_VALUE_SIG: &str = "Lcom/sharkninja/cloudcore/IoTPropertyValue;";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaIoTPropertyValue::full_name(Some(&default_int_value())));
    names.push(JavaIoTPropertyValue::full_name(Some(&default_str_value())));
    names.push(JavaIoTPropertyValue::full_name(Some(&default_bool_value())));
}

fn default_int_value() -> JavaIoTPropertyValue {
    JavaIoTPropertyValue(IoTPropertyValue::Int(0))
}
fn default_str_value() -> JavaIoTPropertyValue {
    JavaIoTPropertyValue(IoTPropertyValue::Str(String::new()))
}
fn default_bool_value() -> JavaIoTPropertyValue {
    JavaIoTPropertyValue(IoTPropertyValue::Bool(false))
}

pub struct JavaIoTPropertyValue(pub IoTPropertyValue);
impl JavaClass<IoTPropertyValue> for JavaIoTPropertyValue {
    fn full_name(instance: Option<&Self>) -> String {
        [
            JAVA_PACKAGE,
            "IoTPropertyValue$",
            match instance.unwrap().0 {
                Int(_) => "Int",
                Str(_) => "String",
                Bool(_) => "Boolean",
            },
        ]
        .concat()
    }

    fn signature(instance: Option<&Self>) -> String {
        match instance.unwrap().0 {
            Int(_) => ["(", INT_SIG, ")", VOID_SIG],
            Str(_) => ["(", STRING_SIG, ")", VOID_SIG],
            Bool(_) => ["(", BOOL_SIG, ")", VOID_SIG],
        }
        .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTPropertyValue::signature(Some(self));

        let value: JValue = match &self.0 {
            Int(int) => JValue::Int(*int as i32),
            Str(string) => JValue::from(
                AndroidString(string.to_owned())
                    .to_jstring(jni_env)
                    .into_inner(),
            ),
            Bool(bool) => JValue::Bool((*bool).into()),
        };

        let args = &[value];
        let value_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!(
                    "Error creating IoT Property Value {:#?} for JNI: {:?}",
                    self.0, err
                );
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *value_object
    }

    fn new(rust_object: IoTPropertyValue) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<IoTPropertyValue> for JavaIoTPropertyValue {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<IoTPropertyValue> {
        let mut property_value = None;
        if !j_object.0.is_null() {
            if env
                .is_instance_of(
                    j_object.0,
                    JavaIoTPropertyValue::full_name(Some(&default_int_value())),
                )
                .unwrap()
            {
                let value = j_object.to_signed_int_field(env, "value");
                property_value = Some(IoTPropertyValue::Int(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaIoTPropertyValue::full_name(Some(&default_str_value())),
                )
                .unwrap()
            {
                let value = j_object.to_string_field(env, "value");
                property_value = Some(IoTPropertyValue::Str(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaIoTPropertyValue::full_name(Some(&default_bool_value())),
                )
                .unwrap()
            {
                let value = j_object.to_bool_field(env, "value");
                property_value = Some(IoTPropertyValue::Bool(value))
            }
        }
        property_value
    }
}
