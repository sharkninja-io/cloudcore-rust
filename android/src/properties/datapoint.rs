use android_utilities::java_class_names::{CLASSNAMES, CLASSREFSMAP};
use android_utilities::java_signatures::{BOOL_SIG, BOOLEAN_SIG, STRING_SIG, VOID_SIG};
use android_utilities::JavaClass;
use android_utilities::jni_exts::option_traits::AndroidOption;
use android_utilities::jni_exts::string::AndroidString;
use ctor::ctor;
use jni::JNIEnv;
use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use log::error;
use cloudcore::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMessage};
use crate::JAVA_PACKAGE;
use crate::properties::value::{JavaIoTPropertyValue, PROPERTY_VALUE_SIG};

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaIoTDatapointFile::full_name(None));
    names.push(JavaIoTDatapoint::full_name(None));
    names.push(JavaIoTDatapointMessage::full_name(None));
}

pub struct JavaIoTDatapoint(pub IoTDatapoint);
impl JavaClass<IoTDatapoint> for JavaIoTDatapoint {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("IoTDataPoint");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            PROPERTY_VALUE_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            BOOLEAN_SIG,
            ")",
            VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTDatapoint::signature(None);

        let updated_at = self.0.updated_at().map(String::from).to_j_value(jni_env);
        let created_at = self.0.created_at().map(String::from).to_j_value(jni_env);
        let echo = self.0.echo().to_owned().to_j_value(jni_env);
        let java_value = JavaIoTPropertyValue(self.0.value().clone());
        let j_value_class = CLASSREFSMAP::get_class(Some(&java_value));
        let value = JValue::from(java_value.j_object(jni_env, j_value_class));
        let user_uuid = self.0.metadata().user_uuid().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/DataPoint **
        let args = &[
            value,
            user_uuid,
            updated_at,
            created_at,
            echo,
        ];

        let datapoint = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating IoT DataPoint for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *datapoint
    }

    fn new(rust_object: IoTDatapoint) -> Self {
        Self(rust_object)
    }
}

pub struct JavaIoTDatapointFile(pub IoTDatapointFile);
impl JavaClass<IoTDatapointFile> for JavaIoTDatapointFile {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("IoTDataPointFile");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            STRING_SIG,
            STRING_SIG,
            BOOL_SIG,
            BOOL_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            ")",
            VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTDatapointFile::signature(None);

        let updated_at = AndroidString(self.0.updated_at().to_owned()).to_jstring(jni_env);
        let created_at = AndroidString(self.0.created_at().to_owned()).to_jstring(jni_env);
        let echo = JValue::Bool(self.0.echo().into());
        let closed = JValue::Bool(self.0.closed().into());
        let generated_at = self.0.generated_at().map(String::from).to_j_value(jni_env);
        let generated_from = self.0.generated_from().map(String::from).to_j_value(jni_env);
        let value = AndroidString(self.0.value().to_owned()).to_jstring(jni_env);
        let created_at_from_device = self.0.created_at_from_device().map(String::from).to_j_value(jni_env);
        let file = AndroidString(self.0.file().to_owned()).to_jstring(jni_env);
        let local_file = self.0.local_file().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/DataPoint **
        let args = &[
            JValue::from(updated_at.into_inner()),
            JValue::from(created_at.into_inner()),
            echo,
            closed,
            generated_at,
            generated_from,
            JValue::from(value.into_inner()),
            created_at_from_device,
            JValue::from(file.into_inner()),
            local_file,
        ];

        let file_datapoint = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating IoT DataPoint File for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *file_datapoint
    }
    fn new(rust_object: IoTDatapointFile) -> Self {
        Self(rust_object)
    }
}

pub struct JavaIoTDatapointMessage(pub IoTDatapointMessage);
impl JavaClass<IoTDatapointMessage> for JavaIoTDatapointMessage {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("IoTDataPointMessage");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            BOOLEAN_SIG,
            STRING_SIG,
            ")",
            VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTDatapointMessage::signature(None);

        let user_uuid = self.0.metadata.user_uuid().map(String::from).to_j_value(jni_env);
        let updated_at = self.0.updated_at.to_owned().to_j_value(jni_env);
        let created_at = self.0.created_at.to_owned().to_j_value(jni_env);
        let echo = self.0.echo.to_owned().to_j_value(jni_env);
        let local_file = AndroidString(self.0.local_file.to_owned()).to_jstring(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/DataPoint **
        let args = &[
            user_uuid,
            updated_at,
            created_at,
            echo,
            JValue::from(local_file.into_inner()),
        ];

        let msg_datapoint = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating IoT DataPoint Message for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *msg_datapoint
    }
    fn new(rust_object: IoTDatapointMessage) -> Self {
        Self(rust_object)
    }
}