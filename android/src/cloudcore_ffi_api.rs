use std::collections::HashMap;
use std::os::raw::{c_char, c_int};

use dlopen::wrapper::{Container, WrapperApi};
use lazy_static::lazy_static;
use log::error;
use mantle_utilities::MantleError;

use cloudcore::{CloudCore, WifiNetwork, WifiPairing, WifiPairingState};
use cloudcore::authentication::UserSession;
use cloudcore::cache::CacheDataValue;
use cloudcore::cloudcore::{ApplicationInfo, AylaRegionEnvironment};
use cloudcore::devices::IoTDevice;
use cloudcore::notifications::notifications::Notification;
use cloudcore::properties::datapoint::{IoTDatapoint, IoTDatapointFile, IoTDatapointMessage};
use cloudcore::properties::property::IoTProperty;
use cloudcore::properties::trigger::IoTTrigger;
use cloudcore::properties::value::IoTPropertyValue;
use cloudcore::schedules::Schedule;

// Think of this as the Header file for the CloudCore FFI library

#[derive(WrapperApi)]
pub struct CloudCoreAPI {
    // Create
    cloudcore_create: fn(
        os_dir: *const c_char,
        hash_map: *mut HashMap<AylaRegionEnvironment, ApplicationInfo>,
    ) -> *const CloudCore,
    // Destroy
    cloudcore_destroy: fn(
        ptr_cloudcore: *mut CloudCore,
    ),
    // Account
    cloudcore_create_account: fn(
        ptr_cloudcore: *mut CloudCore,
        password: *const c_char,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_confirm_account: fn(
        ptr_cloudcore: *mut CloudCore,
        token: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_send_confirmation_instructions: fn(
        ptr_cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_delete_account: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_request_password_reset: fn(
        ptr_cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        email_template_id: *const c_char,
        email_subject: *const c_char,
        email_body_html: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_reset_password: fn(
        ptr_cloudcore: *mut CloudCore,
        token: *const c_char,
        password: *const c_char,
        password_confirmation: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_reset_password_for_current_user: fn(
        ptr_cloudcore: *mut CloudCore,
        current_password: *const c_char,
        new_password: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_update_email: fn(
        ptr_cloudcore: *mut CloudCore,
        new_email: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    // Authentication
    cloudcore_login: fn(
        cloudcore: *mut CloudCore,
        email: *const c_char,
        phone_number: *const c_char,
        password: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_get_user_session: fn(
        ptr_cloudcore: *mut CloudCore,
    ) -> Result<UserSession, Box<MantleError>>,
    cloudcore_set_user_session: fn(
        ptr_cloudcore: *mut CloudCore,
        user_session: *const UserSession,
    ),
    cloudcore_refresh_session: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_logged_in: fn(ptr_cloudcore: *mut CloudCore) -> bool,
    cloudcore_logout: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    // Devices
    cloudcore_devices: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<Vec<IoTDevice>, Box<MantleError>>),
    ),
    cloudcore_device: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        callback: fn(result: Result<IoTDevice, Box<MantleError>>),
    ),
    cloudcore_rename_device: fn(
        ptr_cloudcore: *const CloudCore,
        dsn: *const c_char,
        new_name: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_factory_reset_device: fn(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        dsn: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_delete_device: fn(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        dsn: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_delete_device_map: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        re_explore: *const bool,
        partial_delete: *const bool,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    // Cache
    cloudcore_set_value: fn(
        ptr_cloudcore: *mut CloudCore,
        path: *const c_char,
        key: *const c_char,
        cache_value: *mut CacheDataValue,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_get_value: fn(
        ptr_cloudcore: *mut CloudCore,
        path: *const c_char,
        key: *const c_char,
        callback: fn(result: Result<CacheDataValue, Box<MantleError>>),
    ),
    // Properties
    cloudcore_get_property: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
    ),
    cloudcore_get_properties: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTProperty>, Box<MantleError>>, String)),
    ),
    cloudcore_get_data_points: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapoint>, Box<MantleError>>, String)),
    ),
    cloudcore_get_file_property: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
    ),
    cloudcore_get_file_properties: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
    ),
    cloudcore_get_message_property: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
    ),
    cloudcore_get_message_properties: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        names: *mut Vec<String>,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
    ),
    cloudcore_get_datapoint_with_id: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        datapoint_id: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointMessage, Box<MantleError>>, String)),
    ),
    cloudcore_get_datapoint_with_file_url: fn(
        ptr_cloudcore: *mut CloudCore,
        url: *const c_char,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<IoTDatapointFile, Box<MantleError>>, String)),
    ),
    cloudcore_set_property_value: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        value: *mut IoTPropertyValue,
        callback_id: *const c_char,
        callback: fn(result: (Result<(), Box<MantleError>>, String)),
    ),
    cloudcore_get_file_property_as_files: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointFile>, Box<MantleError>>, String)),
    ),
    cloudcore_get_message_property_as_files: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        count: *const c_int,
        from: *const c_char,
        to: *const c_char,
        callback_id: *const c_char,
        callback: fn(result: (Result<Vec<IoTDatapointMessage>, Box<MantleError>>, String)),
    ),
    cloudcore_save_file: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        path: *const c_char,
        is_message: *const bool,
        callback_id: *const c_char,
        callback: fn(result: (Result<(), Box<MantleError>>, String)),
    ),
    // Pairing
    cloudcore_create_pairing_manager: fn(
        ptr_cloudcore: *const CloudCore,
        get_state_callback: fn(
            state: WifiPairingState,
        ),
        wifi_networks_callback: fn(
            wifi_networks: Vec<WifiNetwork>,
        ),
        result_callback: fn(
            result: Result<String, Box<MantleError>>
        ),
    ) -> *mut WifiPairing,
    cloudcore_start_pairing: fn(
        ptr_wifi_manager: *mut WifiPairing,
        ip_address: *const c_char,
    ),
    cloudcore_continue_pairing: fn(
        ptr: *mut WifiPairing
    ),
    cloudcore_done_pairing: fn(
        ptr: *mut WifiPairing
    ),
    cloudcore_handle_selected_network: fn(
        ptr_wifi_manager: *mut WifiPairing,
        selected_network: *mut WifiNetwork,
    ),
    cloudcore_write_to_pairing_log: fn(
        ptr_cloudcore: *const CloudCore,
        c_content: *const c_char,
    ) -> Result<(), Box<MantleError>>,
    cloudcore_get_pairing_log: fn(
        ptr_cloudcore: *const CloudCore,
    ) -> Result<String, Box<MantleError>>,
    // Schedules
    cloudcore_create_device_schedule: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        name: *const c_char,
        start_date: *const c_char,
        start_time_each_day: *const c_char,
        action_name: *const c_char,
        action_base_type: *const c_char,
        callback: fn(result: Result<Schedule, Box<MantleError>>),
    ),
    cloudcore_fetch_schedules: fn(
        ptr_cloudcore: *mut CloudCore,
        device_id: *const u32,
        callback: fn(result: Result<Vec<Schedule>, Box<MantleError>>),
    ),
    cloudcore_update_device_schedule: fn(
        ptr_cloudcore: *mut CloudCore,
        schedule: *const Schedule,
        callback: fn(result: Result<Schedule, Box<MantleError>>),
    ),
    // Notifications
    cloudcore_fetch_all_notifications: fn(
        ptr_cloudcore: *mut CloudCore,
        from: *const c_char,
        callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
    ),
    cloudcore_get_cached_notifications: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<Vec<Notification>, Box<MantleError>>),
    ),
    cloudcore_delete_all_notifications: fn(
        ptr_cloudcore: *mut CloudCore,
        to: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_delete_notification: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        id: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_mark_all_notifications_as_read: fn(
        ptr_cloudcore: *mut CloudCore,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_fetch_triggers: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback: fn(result: Result<Vec<IoTTrigger>, Box<MantleError>>),
    ),
    cloudcore_create_error_push_triggers: fn(
        ptr_cloudcore: *mut CloudCore,
        robot_nicknames: *mut HashMap<String, String>,
        device_id: *const c_char,
        application_id: *const c_char,
        channel_id: *const c_char,
        registration_id: *const c_char,
        service: *const c_char,
        errors: *mut HashMap<String, HashMap<u32, String>>,
        callback: fn(result: Result<Vec<IoTTrigger>, Box<MantleError>>),
    ),
    cloudcore_delete_all_triggers: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_update_all_trigger_apps_registration_id: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        device_id: *const c_char,
        registration_id: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
    cloudcore_delete_all_trigger_apps_by_device_id: fn(
        ptr_cloudcore: *mut CloudCore,
        dsn: *const c_char,
        prop_name: *const c_char,
        device_id: *const c_char,
        callback: fn(result: Result<(), Box<MantleError>>),
    ),
}

lazy_static! {
    pub static ref CLOUDCORE_API: Container<CloudCoreAPI> = {
        let lib = "libcloudcore_ffi.so";
        let cc_api: Container<CloudCoreAPI> = unsafe { Container::load(lib) }.unwrap_or_else(|err| {
            error!("Could not open {} or load symbols: {}", lib, err);
            panic!()
        });
        cc_api
    };
}