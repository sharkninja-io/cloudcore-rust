use android_utilities::java_class_names::{CLASSNAMES, CLASSREFSMAP};
use android_utilities::java_signatures::{BOOL_SIG, INTEGER_SIG, STRING_SIG, VOID_SIG};
use android_utilities::JavaClass;
use android_utilities::jni_exts::option_traits::AndroidOption;
use android_utilities::jni_exts::string::AndroidString;
use crate::properties::value::{JavaIoTPropertyValue, PROPERTY_VALUE_SIG};
use cloudcore::properties::property::IoTProperty;
use ctor::ctor;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;
use crate::JAVA_PACKAGE;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaIoTProperty::full_name(None));
}

pub struct JavaIoTProperty(pub IoTProperty);
impl JavaClass<IoTProperty> for JavaIoTProperty {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("IoTProperty");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            BOOL_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            INTEGER_SIG,
            INTEGER_SIG,
            STRING_SIG,
            BOOL_SIG,
            STRING_SIG,
            BOOL_SIG,
            BOOL_SIG,
            BOOL_SIG,
            STRING_SIG,
            STRING_SIG,
            PROPERTY_VALUE_SIG,
            BOOL_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            ")",
            VOID_SIG,
        ]
        .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTProperty::signature(None);

        let r#type = AndroidString(self.0.r#type().to_owned()).to_jstring(jni_env);
        let name = AndroidString(self.0.name().to_owned()).to_jstring(jni_env);
        let base_type = AndroidString(self.0.base_type().to_owned()).to_jstring(jni_env);
        let read_only = JValue::Bool(self.0.read_only().into());
        let direction = AndroidString(self.0.direction().to_owned()).to_jstring(jni_env);
        let scope = AndroidString(self.0.scope().to_owned()).to_jstring(jni_env);
        let data_updated_at = self.0.data_updated_at().map(String::from).to_j_value(jni_env);//AndroidString(self.0.data_updated_at().to_owned()).to_jstring(jni_env);
        let key = self.0.key().to_owned().to_j_value(jni_env);
        let device_key = self.0.device_key().to_owned().to_j_value(jni_env);
        let product_name = AndroidString(self.0.product_name().to_owned()).to_jstring(jni_env);
        let track_only_changes = JValue::Bool(self.0.track_only_changes().into());
        let display_name = AndroidString(self.0.display_name().to_owned()).to_jstring(jni_env);
        let host_sw_version = JValue::Bool(self.0.host_sw_version().into());
        let time_series = JValue::Bool(self.0.time_series().into());
        let derived = JValue::Bool(self.0.derived().into());
        let app_type = self.0.app_type().map(String::from).to_j_value(jni_env);
        let recipe = self.0.recipe().map(String::from).to_j_value(jni_env);
        let value = match self.0.value() {
            None => JValue::from(JObject::null()),
            Some(value) => {
                let java_value = JavaIoTPropertyValue(value.clone());
                let j_class = CLASSREFSMAP::get_class(Some(&java_value));
                JValue::from(java_value.j_object(jni_env, j_class))
            }
        };
        let ack_enabled = JValue::Bool(self.0.ack_enabled().into());
        let ack_status = self.0.ack_status().map(String::from).to_j_value(jni_env);
        let ack_message = self.0.ack_message().map(String::from).to_j_value(jni_env);
        let acked_at = self.0.acked_at().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/Properties **
        let args = &[
            JValue::from(r#type.into_inner()),
            JValue::from(name.into_inner()),
            JValue::from(base_type.into_inner()),
            read_only,
            JValue::from(direction.into_inner()),
            JValue::from(scope.into_inner()),
            data_updated_at,
            key,
            device_key,
            JValue::from(product_name.into_inner()),
            track_only_changes,
            JValue::from(display_name.into_inner()),
            host_sw_version,
            time_series,
            derived,
            app_type,
            recipe,
            value,
            ack_enabled,
            ack_status,
            ack_message,
            acked_at,
        ];

        let prop_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating IoT Property for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *prop_object
    }

    fn new(rust_object: IoTProperty) -> Self {
        Self(rust_object)
    }
}
