use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{BOOLEAN_SIG, INTEGER_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::option_traits::AndroidOption;
use cloudcore::devices::IoTDevice;
use ctor::ctor;
use jni::objects::JClass;
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;
use crate::JAVA_PACKAGE;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaIoTDevice::full_name(None));
}

pub struct JavaIoTDevice(pub IoTDevice);
impl JavaClass<IoTDevice> for JavaIoTDevice {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("IoTDevice");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(", INTEGER_SIG, STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG,
            STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG, STRING_SIG,
            BOOLEAN_SIG, BOOLEAN_SIG, INTEGER_SIG, ")", VOID_SIG,
        ]
        .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaIoTDevice::signature(None);
        let id = self.0.id().to_j_value(jni_env);
        let product_name = self.0.product_name().map(String::from).to_j_value(jni_env);
        let model = self.0.model().map(String::from).to_j_value(jni_env);
        let dsn = self.0.dsn().map(String::from).to_j_value(jni_env);
        let oem_model = self.0.oem_model().map(String::from).to_j_value(jni_env);
        let sw_version = self.0.sw_version().map(String::from).to_j_value(jni_env);
        let template_id = self.0.template_id().to_j_value(jni_env);
        let mac = self.0.mac().map(String::from).to_j_value(jni_env);
        let lan_ip = self.0.lan_ip().map(String::from).to_j_value(jni_env);
        let connected_at = self.0.connected_at().map(String::from).to_j_value(jni_env);
        let lan_enabled = self.0.lan_enabled().to_j_value(jni_env);
        let has_properties = self.0.has_properties().to_j_value(jni_env);
        let connection_status = self.0.connection_status().map(String::from).to_j_value(jni_env);
        let lat = self.0.lat().map(String::from).to_j_value(jni_env);
        let lng = self.0.lng().map(String::from).to_j_value(jni_env);
        let device_type = self.0.device_type().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/Devices **
        let args = &[
            id,
            product_name,
            model,
            dsn,
            oem_model,
            sw_version,
            connection_status,
            mac,
            lan_ip,
            connected_at,
            lat,
            lng,
            device_type,
            has_properties,
            lan_enabled,
            template_id,
        ];
        let dev_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating IoT device for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *dev_object
    }

    fn new(rust_object: IoTDevice) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<IoTDevice> for JavaIoTDevice {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<IoTDevice> {
        if !j_object.0.is_null()
            && env
                .is_instance_of(j_object.0, JavaIoTDevice::full_name(None))
                .unwrap()
        {
            let id = j_object.to_optional_unsigned_int_field(env, "id");
            let product_name = j_object.to_optional_string_field(env, "productName");
            let model = j_object.to_optional_string_field(env, "model");
            let dsn = j_object.to_optional_string_field(env, "dsn");
            let oem_model = j_object.to_optional_string_field(env, "oemModel");
            let sw_version = j_object.to_optional_string_field(env, "swVersion");
            let connection_status = j_object.to_optional_string_field(env, "connectionStatus");
            let mac = j_object.to_optional_string_field(env, "mac");
            let lan_ip = j_object.to_optional_string_field(env, "lanIp");
            let connected_at = j_object.to_optional_string_field(env, "connectedAt");
            let lat = j_object.to_optional_string_field(env, "lat");
            let lng = j_object.to_optional_string_field(env, "lng");
            let device_type = j_object.to_optional_string_field(env, "deviceType");
            let has_properties = j_object.to_optional_bool_field(env, "hasProperties");
            let lan_enabled = j_object.to_optional_bool_field(env, "lanEnabled");
            let template_id = j_object.to_optional_unsigned_int_field(env, "templateId");
            let device = IoTDevice::new(
                id,
                product_name,
                model,
                dsn,
                oem_model,
                sw_version,
                template_id,
                mac,
                lan_ip,
                connected_at,
                lan_enabled,
                has_properties,
                connection_status,
                lat,
                lng,
                device_type,
            );
            Some(device)
        } else {
            None
        }
    }
}
