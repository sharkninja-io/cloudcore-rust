use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{BOOL_SIG, INT_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::string::AndroidString;
use cloudcore::authentication::UserSession;
use ctor::ctor;
use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;
use crate::JAVA_PACKAGE;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaUserSession::full_name(None));
}

pub struct JavaUserSession(pub UserSession);
impl JavaClass<UserSession> for JavaUserSession {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("UserSession");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(", STRING_SIG, STRING_SIG, INT_SIG, STRING_SIG, STRING_SIG, BOOL_SIG, ")", VOID_SIG,
        ]
        .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let us_signature = JavaUserSession::signature(None);

        let access_token = AndroidString(self.0.access_token().to_owned()).to_jstring(jni_env);
        let refresh_token = AndroidString(self.0.refresh_token().to_owned()).to_jstring(jni_env);
        let expiration_date = JValue::Int(self.0.auth_expiration_date() as i32);
        let auth_username = AndroidString(self.0.auth_username().to_owned()).to_jstring(jni_env);
        let user_uuid = AndroidString(self.0.user_uuid().unwrap().to_owned()).to_jstring(jni_env);
        let use_dev = JValue::Bool(self.0.use_dev().into());

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/UserSession **
        let us_args = &[
            JValue::from(access_token.into_inner()),
            JValue::from(refresh_token.into_inner()),
            expiration_date,
            JValue::from(auth_username.into_inner()),
            JValue::from(user_uuid.into_inner()),
            use_dev,
        ];
        let us_object = jni_env
            .new_object(j_class, us_signature, us_args)
            .unwrap_or_else(|err| {
                error!("Error creating user session for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *us_object
    }

    fn new(rust_object: UserSession) -> Self {
        Self(rust_object)
    }
}

// This is needed so the first time an app switches to using CC for auth it can pass in it's saved auth params
impl JObjectRustBridge<UserSession> for JavaUserSession {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<UserSession> {
        if !j_object.0.is_null()
            && env
            .is_instance_of(j_object.0, JavaUserSession::full_name(None))
            .unwrap()
        {
            let access_token = j_object.to_string_field(env, "accessToken");
            let refresh_token = j_object.to_string_field(env, "refreshToken");
            let auth_expiration_date = j_object.to_unsigned_int_field(env, "authExpirationDate");
            let auth_username = j_object.to_string_field(env, "authUsername");
            let user_uuid = Some(j_object.to_string_field(env, "userUUID"));
            let use_dev = j_object.to_bool_field(env, "useDev");
            let user_session = UserSession::new(
                access_token,
                refresh_token,
                auth_expiration_date as u64,
                auth_username,
                user_uuid,
                use_dev,
            );
            Some(user_session)
        } else {
            None
        }
    }
}