#[cfg(feature = "library")]
pub static AYLA_USER_JSON: &str = "/users.json";
#[cfg(feature = "library")]
pub static AYLA_DEVICE_JSON: &str = "/apiv1/devices.json";

#[cfg(feature = "library")]
pub static AYLA_SIGN_IN_JSON: &str = "/users/sign_in.json";
#[cfg(feature = "library")]
pub static AYLA_SIGN_OUT_JSON: &str = "/users/sign_out.json";
#[cfg(feature = "library")]
pub static AYLA_CONFIRMATION_JSON: &str = "/users/confirmation.json";
#[cfg(feature = "library")]
pub static AYLA_PASSWORD_JSON: &str = "/users/password.json";
#[cfg(feature = "library")]
pub static AYLA_REFRESH_TOKEN_JSON: &str = "/users/refresh_token.json";
#[cfg(feature = "library")]
pub static AYLA_UPDATE_EMAIL_JSON: &str = "/users/update_email.json";

#[cfg(feature = "library")]
pub static AUTHORIZATION_HEADER: &str = "Authorization";
#[cfg(feature = "library")]
pub static AUTHORIZATION_BEARER: &str = "auth_token";

#[cfg(feature = "library")]
pub static AYLA_PROPS_QUERY_PARAMS_KEY: &str = "names[]";
#[cfg(feature = "library")]
pub static AYLA_PROPS_JSON: &str = "/apiv1/dsns/<dsn>/properties.json";
#[cfg(feature = "library")]
pub static AYLA_PROPS_DATAPOINTS_JSON: &str =
    "/apiv1/dsns/<dsn>/properties/<prop_name>/datapoints.json";
#[cfg(feature = "library")]
pub static AYLA_PROPS_MSG_DATAPOINTS_JSON: &str =
    "/apiv1/dsns/<dsn>/properties/<prop_name>/message_datapoints.json";
#[cfg(feature = "library")]
pub static AYLA_PROP_DATAPOINT_ID_JSON: &str =
    "/apiv1/dsns/<dsn>/properties/<prop_name>/datapoints/<datapoint_id>.json";
#[cfg(feature = "library")]
pub static AYLA_DATAPOINTS_FILTER_SINCE_DATE_KEY: &str = "filter[created_at_since_date]";
#[cfg(feature = "library")]
pub static AYLA_DATAPOINTS_FILTER_END_DATE_KEY: &str = "filter[created_at_end_date]";
#[cfg(feature = "library")]
pub static AYLA_DATAPOINTS_LIMIT_KEY: &str = "limit";

#[cfg(feature = "library")]
pub static AYLA_USER_PROFILE_JSON: &str = "/users/get_user_profile.json";

#[cfg(feature = "library")]
pub static CRATE_WORKSPACE: &str = env!("CARGO_PKG_NAME");

#[cfg(feature = "library")]
pub static AYLA_DEVICE_SCHEDULE_JSON: &str = "/apiv1/devices/<dsn>/schedules.json";
#[cfg(feature = "library")]
pub static AYLA_DEVICE_USER_SCHEDULES_JSON: &str = "/apiv1/schedules/all/by_user.json";
#[cfg(feature = "library")]
pub static AYLA_DEVICE_UPDATE_SCHEDULE_JSON: &str = "/apiv1/devices/<dsn>/schedules/<prop_name>.json";
#[cfg(feature = "library")]
pub static AYLA_DEVICE_ID_SCHEDULE_JSON: &str = "/apiv1/devices/<dsn>/schedules.json";


#[cfg(feature = "library")]
pub static PROPS_PATH_PARAMS_TRIGGER_KEY: &str = "<trigger_key>";
#[cfg(feature = "library")]
pub static PROPS_PATH_PARAMS_TRIGGER_APP_KEY: &str = "<trigger_app_key>";
#[cfg(feature = "library")]
pub static AYLA_PROPS_TRIGGERS_JSON: &str =
    "/apiv1/dsns/<dsn>/properties/<prop_name>/triggers.json";
#[cfg(feature = "library")]
pub static AYLA_TRIGGER_APPS_JSON: &str =
    "/apiv1/triggers/<trigger_key>/trigger_apps.json";
#[cfg(feature = "library")]
pub static AYLA_TRIGGER_JSON: &str =
    "/apiv1/triggers/<trigger_key>.json";
#[cfg(feature = "library")]
pub static AYLA_TRIGGER_APP: &str =
    "/apiv1/trigger_apps/<trigger_app_key>.json";