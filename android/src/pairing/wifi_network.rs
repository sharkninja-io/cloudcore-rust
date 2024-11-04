use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{INTEGER_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::option_traits::AndroidOption;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jobject;
use cloudcore::pairing::wifi_network::WifiNetwork;
use ctor::ctor;
use log::error;
use crate::JAVA_PACKAGE;

pub static WIFI_NETWORK_SIG: &str = "Lcom/sharkninja/cloudcore/WifiNetwork;";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaWifiNetwork::full_name(None));
}

pub struct JavaWifiNetwork(pub WifiNetwork);
impl JavaClass<WifiNetwork> for JavaWifiNetwork {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("WifiNetwork");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(", INTEGER_SIG, STRING_SIG, INTEGER_SIG, STRING_SIG, INTEGER_SIG, STRING_SIG,
            STRING_SIG, STRING_SIG, ")", VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaWifiNetwork::signature(None);

        let bars = self.0.bars().to_owned().to_j_value(jni_env);
        let bssid = self.0.bssid().map(String::from).to_j_value(jni_env);
        let chan = self.0.chan().to_owned().to_j_value(jni_env);
        let security = self.0.security().map(String::from).to_j_value(jni_env);
        let signal = self.0.signal().to_owned().to_j_value(jni_env);
        let ssid = self.0.ssid().map(String::from).to_j_value(jni_env);
        let r#type = self.0.r#type().map(String::from).to_j_value(jni_env);
        let password = self.0.password().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/Pairing **
        let args = &[
            bars,
            bssid,
            chan,
            security,
            signal,
            ssid,
            r#type,
            password,
        ];

        let network_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating WifiNetwork for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *network_object
    }

    fn new(rust_object: WifiNetwork) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<WifiNetwork> for JavaWifiNetwork {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<WifiNetwork> {
        if !j_object.0.is_null()
            && env
            .is_instance_of(j_object.0, JavaWifiNetwork::full_name(None))
            .unwrap()
        {
            let bars = j_object.to_optional_unsigned_int_field(env, "bars");
            let bssid = j_object.to_optional_string_field(env, "bssid");
            let chan = j_object.to_optional_unsigned_int_field(env, "chan");
            let security = j_object.to_optional_string_field(env, "security");
            let signal = j_object.to_optional_signed_int_field(env, "signal");
            let ssid = j_object.to_optional_string_field(env, "ssid");
            let r#type = j_object.to_optional_string_field(env, "type");
            let password = j_object.to_optional_string_field(env, "password");

            let network = WifiNetwork::new(
                bars,
                bssid,
                chan,
                security,
                signal,
                ssid,
                r#type,
                password
            );
            Some(network)
        } else {
            None
        }
    }
}