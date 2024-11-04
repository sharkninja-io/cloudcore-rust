
#[repr(C)]
#[derive(PartialEq, Debug, Clone)]
pub enum WifiPairingState {
    Idle,
    FetchingDSN,
    DeviceScanningWifi,
    GettingWifiNetworks,
    SendingWiFiCredentialsToDevice,
    EndingAccessPointsScanning,
    PollingUserInternetConnection,
    HandshakingWithAyla,
    PollingDeviceOnAyla,
    Connected,
    Done,
}