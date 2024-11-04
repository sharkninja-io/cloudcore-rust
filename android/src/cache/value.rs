use android_utilities::java_class_names::{CLASSNAMES, CLASSREFSMAP};
use android_utilities::java_signatures::{BOOL_SIG, DOUBLE_SIG, INT_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::string::AndroidString;
use android_utilities::jni_exts::unit::{AndroidUnit, KOTLIN_UNIT_SIG};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jobject;
use cloudcore::cache::CacheDataValue;
use ctor::ctor;
use log::error;
use serde_json::Value;
use cloudcore::cache::CacheDataValue::{BooleanValue, DoubleValue, IntegerValue, ObjectValue, StringValue};
use crate::JAVA_PACKAGE;

#[allow(dead_code)]
pub static PROPERTY_VALUE_SIG: &str = "Lcom/sharkninja/cloudcore/CacheDataValue;";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaCacheDataValue::full_name(Some(&default_str_value())));
    names.push(JavaCacheDataValue::full_name(Some(&default_int_value())));
    names.push(JavaCacheDataValue::full_name(Some(&default_double_value())));
    names.push(JavaCacheDataValue::full_name(Some(&default_bool_value())));
    names.push(JavaCacheDataValue::full_name(Some(&default_object_value())));
    names.push(JavaCacheDataValue::full_name(Some(&JavaCacheDataValue(CacheDataValue::NullValue))));
}


fn default_str_value() -> JavaCacheDataValue {
    JavaCacheDataValue(StringValue(String::new()))
}
fn default_int_value() -> JavaCacheDataValue {
    JavaCacheDataValue(IntegerValue(0))
}
fn default_double_value() -> JavaCacheDataValue {
    JavaCacheDataValue(DoubleValue(0.0))
}
fn default_bool_value() -> JavaCacheDataValue {
    JavaCacheDataValue(BooleanValue(false))
}
fn default_object_value() -> JavaCacheDataValue {
    JavaCacheDataValue(ObjectValue(Value::Null))
}

pub struct JavaCacheDataValue(pub CacheDataValue);
impl JavaClass<CacheDataValue> for JavaCacheDataValue {
    fn full_name(instance: Option<&Self>) -> String {
        [
            JAVA_PACKAGE,
            "CacheDataValue$",
            match instance.unwrap().0 {
                StringValue(_) => "String",
                IntegerValue(_) => "Int",
                DoubleValue(_) => "Double",
                BooleanValue(_) => "Boolean",
                ObjectValue(_) => "Object",
                CacheDataValue::NullValue => "Null"
            },
        ]
            .concat()
    }

    fn signature(instance: Option<&Self>) -> String {
        match instance.unwrap().0 {
            StringValue(_) => ["(", STRING_SIG, ")", VOID_SIG],
            IntegerValue(_) => ["(", INT_SIG, ")", VOID_SIG],
            DoubleValue(_) => ["(", DOUBLE_SIG, ")", VOID_SIG],
            BooleanValue(_) => ["(", BOOL_SIG, ")", VOID_SIG],
            ObjectValue(_) => ["(", STRING_SIG, ")", VOID_SIG],
            CacheDataValue::NullValue => ["(", KOTLIN_UNIT_SIG, ")", VOID_SIG],
        }
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaCacheDataValue::signature(Some(self));

        let value: JValue = match &self.0 {
            StringValue(string) => JValue::from(
                AndroidString(string.to_owned())
                    .to_jstring(jni_env)
                    .into_inner()
            ),
            IntegerValue(int) => JValue::Int(*int as i32),
            DoubleValue(double) => JValue::Double(*double as f64),
            BooleanValue(bool) => JValue::Bool((*bool).into()),
            ObjectValue(object) => {
                let json = serde_json::to_string(object).unwrap_or("{}".to_string());
                JValue::from(
                    AndroidString(json.to_owned())
                        .to_jstring(jni_env)
                        .into_inner()
                )
            }
            CacheDataValue::NullValue => {
                let null_object = AndroidUnit::j_object(&(), jni_env, CLASSREFSMAP::get_class(Some(&())));
                JValue::from(JObject::from(null_object))
            },
        };

        let args = &[value];
        let value_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!(
                    "Error creating Cache Data Value {:#?} for JNI: {:?}",
                    self.0, err
                );
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *value_object
    }

    fn new(rust_object: CacheDataValue) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<CacheDataValue> for JavaCacheDataValue {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<CacheDataValue> {
        let mut cache_value = None;
        if !j_object.0.is_null() {
            if env
                .is_instance_of(
                    j_object.0,
                    JavaCacheDataValue::full_name(Some(&default_str_value())),
                )
                .unwrap()
            {
                let value = j_object.to_string_field(env, "value");
                cache_value = Some(StringValue(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaCacheDataValue::full_name(Some(&default_int_value())),
                )
                .unwrap()
            {
                let value = j_object.to_signed_int_field(env, "value");
                cache_value = Some(IntegerValue(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaCacheDataValue::full_name(Some(&default_double_value())),
                )
                .unwrap()
            {
                let value = j_object.to_double_field(env, "value");
                cache_value = Some(DoubleValue(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaCacheDataValue::full_name(Some(&default_bool_value())),
                )
                .unwrap()
            {
                let value = j_object.to_bool_field(env, "value");
                cache_value = Some(BooleanValue(value))
            } else if env
                .is_instance_of(
                    j_object.0,
                    JavaCacheDataValue::full_name(Some(&default_object_value())),
                )
                .unwrap()
            {
                let json = j_object.to_string_field(env, "json");
                let value = serde_json::from_str(json.as_str()).unwrap_or(Value::Null);
                cache_value = Some(ObjectValue(value))
            } else {
                cache_value = Some(CacheDataValue::NullValue)
            }
        }
        cache_value
    }
}