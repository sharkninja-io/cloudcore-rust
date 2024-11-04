use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::JavaClass;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jobject;
use cloudcore::WifiPairingState;
use ctor::ctor;
use log::error;
use cloudcore::WifiPairingState::{Done, Connected, DeviceScanningWifi, EndingAccessPointsScanning, FetchingDSN, GettingWifiNetworks, HandshakingWithAyla, Idle, PollingDeviceOnAyla, PollingUserInternetConnection, SendingWiFiCredentialsToDevice};
use crate::JAVA_PACKAGE;

pub static WIFI_PAIRING_STATE_SIG: &str = "Lcom/sharkninja/cloudcore/WifiPairingState;";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaWifiPairingState::full_name(None))
}

pub struct JavaWifiPairingState(pub WifiPairingState);
impl JavaClass<WifiPairingState> for JavaWifiPairingState {
    fn full_name(_instance: Option<&Self>) -> String {
        [
            JAVA_PACKAGE,
            "WifiPairingState",
        ]
            .concat()
    }

    fn signature(_instance: Option<&Self>) -> String {
        WIFI_PAIRING_STATE_SIG.to_string()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaWifiPairingState::signature(None);
        let state_object = jni_env.get_static_field(j_class, field_name(&self.0), signature).unwrap_or_else(|err| {
            error!(
                    "Error creating Wifi Pairing State {:#?} for JNI: {:?}",
                    self.0, err
                );
            jni_env.exception_describe().unwrap();
            panic!();
        });
        *state_object.l().unwrap()
    }

    fn new(rust_object: WifiPairingState) -> Self {
        Self(rust_object)
    }
}

fn field_name(state: &WifiPairingState) -> String {
    match state {
        Idle => "Idle",
        FetchingDSN => "FetchingDSN",
        DeviceScanningWifi => "DeviceScanningWifi",
        GettingWifiNetworks => "GettingWifiNetworks",
        SendingWiFiCredentialsToDevice=> "SendingWiFiCredentialsToDevice",
        EndingAccessPointsScanning => "EndingAccessPointsScanning",
        PollingUserInternetConnection => "PollingUserInternetConnection",
        HandshakingWithAyla=> "HandshakingWithAyla",
        PollingDeviceOnAyla => "PollingDeviceOnAyla",
        Connected => "Connected",
        Done => "Done",
    }.to_string()
}