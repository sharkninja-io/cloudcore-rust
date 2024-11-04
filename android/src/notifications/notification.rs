use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{BOOL_SIG, INT_SIG, STRING_SIG, VOID_SIG};
use android_utilities::JavaClass;
use android_utilities::jni_exts::string::AndroidString;
use ctor::ctor;
use jni::JNIEnv;
use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use log::error;

use cloudcore::notifications::notifications::Notification;

use crate::JAVA_PACKAGE;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaNotification::full_name(None));
}

pub struct JavaNotification(pub Notification);

impl JavaClass<Notification> for JavaNotification {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("Notification");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            BOOL_SIG,
            BOOL_SIG,
            INT_SIG,
            INT_SIG,
            ")",
            VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaNotification::signature(None);

        let user_uuid = AndroidString(self.0.user_uuid().to_owned()).to_jstring(jni_env);
        let id = AndroidString(self.0.id().to_owned()).to_jstring(jni_env);
        let dsn = AndroidString(self.0.dsn().to_owned()).to_jstring(jni_env);
        let created_at = AndroidString(self.0.created_at().to_owned()).to_jstring(jni_env);
        let datapoint_created_at = AndroidString(self.0.datapoint_created_at().to_owned()).to_jstring(jni_env);
        let read = JValue::Bool(self.0.read().into());
        let deleted = JValue::Bool(self.0.deleted().into());
        let notification_type = JValue::Int(self.0.notification_type() as i32);
        let notification_subtype = JValue::Int(self.0.notification_subtype() as i32);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/DataPoint **
        let args = &[
            JValue::from(user_uuid.into_inner()),
            JValue::from(id.into_inner()),
            JValue::from(dsn.into_inner()),
            JValue::from(created_at.into_inner()),
            JValue::from(datapoint_created_at.into_inner()),
            read,
            deleted,
            notification_type,
            notification_subtype,
        ];

        let notification = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating Notification for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *notification
    }

    fn new(rust_object: Notification) -> Self {
        Self(rust_object)
    }
}