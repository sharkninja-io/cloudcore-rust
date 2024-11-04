use std::collections::HashMap;
use std::env;
use std::error::Error;

use log::{debug, error, LevelFilter};
use mantle_utilities::MantleError;
use simplelog::{Config, SimpleLogger};

use cloudcore::CloudCore;
use cloudcore::examples::utils::get_cloudcore;
use cloudcore::notifications::notifications::NotificationService;
use cloudcore::properties::trigger::{ERROR_NOTIFICATION_PROPERTY_NAME, IoTTrigger, TriggerAppRequest, TriggerRequest};

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(
            "Need email and password arguments".into(),
        );
    }
    let email = &args[1];
    let pw = &args[2];
    cloudcore
        .login(
            Some(email.to_string()),
            None,
            pw.to_string(),
        )
        .await?;
    if args.len() < 6 {
        return Err(
            "Need a DSN".into(),
        );
    }
    let dsn = &args[3];
    let device_id = &args[4];
    let registration_id = &args[5];

    let error_prop = "GET_Error_Code";
    // let mut triggers = cloudcore.fetch_triggers(dsn.to_string(), error_prop.to_string()).await?;
    // let original_count = triggers.len();
    // println!("Fetched {} {} triggers: {:?}", original_count, error_prop, triggers);

    let res = cloudcore.create_trigger_and_app(
        dsn.clone(),
        error_prop.to_string(),
           TriggerRequest::new_error_request("Test device".to_string(), "32".to_string()),
            TriggerAppRequest::fcm_request(
                "testreg".to_string(),
                "test".to_string(),
                "test".to_string(),
                None,
                "test".to_string()
            )
    ).await;

    println!("Created trigger and app triggers: {:?}", res);

    //
    // println!("Fetched {} {} triggers: {:?}", original_count, error_prop, triggers);
    // let res = cloudcore.delete_all_triggers(dsn.to_string(), error_prop.to_string()).await;
    // println!("Delete ALL triggers for {}: {}", error_prop.to_string(), res.is_ok());
    // let property_type = "integer";
    // let compare_type = "==";
    // let trigger_type = "compare_absolute";
    // let value = "1";
    // let active = true;
    // let new_trigger = cloudcore.create_trigger(
    //     dsn.to_string(),
    //     error_prop.to_string(),
    //     TriggerRequest::new(
    //         "test".to_string(),
    //         "test".to_string(),
    //         compare_type.to_string(),
    //         trigger_type.to_string(),
    //         value.to_string(),
    //         active,
    //         property_type.to_string(),
    //     ),
    // ).await?;
    // println!("Successfully created a trigger"); // : {:?}", new_trigger);
    // let trigger_key = new_trigger.key;
    //
    // let tar = TriggerAppRequest::fcm_request(
    //     registration_id.to_string(),
    //     "test_app_id".to_string(),
    //     "Test push message".to_string(),
    //     None,
    //     "{android_device_id: \"" + device_id.to_string() + "\"}".to_string(),
    // );
    // let new_trigger_app = cloudcore.create_trigger_app(
    //     trigger_key, tar,
    // ).await?;
    // println!("Successfully created a trigger app."); // {:?}", new_trigger_app);
    // let updated_trigger_apps = cloudcore.update_all_trigger_apps_registration_id(
    //     dsn.to_string(),
    //     ERROR_NOTIFICATION_PROPERTY_NAME.to_string(),
    //     device_id.to_string(),
    //     registration_id.to_string(),
    // ).await;
    // println!("update_all_trigger_apps_registration_id - SUCCESS: {}", updated_trigger_apps.is_ok());
    // //
    // let trigger_app_key = new_trigger_app.key;
    // let update_tar = TriggerAppRequest::new(
    //     "email".to_string(),
    //     Some("Test Error Code - Email Trigger (Updated)".to_string()),
    //     None, // repeat interval in seconds
    //     Some("testemail@testemail.com".to_string()),
    //     None,
    //     Some("test".to_string()),
    //     None,
    //     None,
    //     None,
    //     None,
    //     None,
    //     Some("1234".to_string()),
    //     Some("Test subject (Updated)".to_string()),
    //     Some("Test email body".to_string()),
    //     Some(false),
    // );
    // let updated_trigger_app = cloudcore.update_trigger_app(
    //     trigger_app_key,
    //     update_tar,
    // ).await?;
    // println!("Successfully updated a trigger app."); // {:?}", updated_trigger_app);
    //
    // triggers = cloudcore.fetch_triggers(dsn.to_string(), error_prop.to_string()).await?;
    // //assert_eq!(triggers.len(), original_count + 1);
    // println!("Fetched {} {} triggers back, originally got {}", triggers.len(), error_prop, original_count);
    //
    // let trigger_app_key = new_trigger_app.key;
    // cloudcore.delete_trigger_app(trigger_app_key).await?;
    // println!("Successfully deleted trigger app: {}", trigger_app_key.to_string());
    //
    // let trigger_key = new_trigger.key;
    // cloudcore.delete_trigger(trigger_key).await?;
    // println!("Successfully deleted trigger: {}", trigger_key.to_string());
    //
    // triggers = cloudcore.fetch_triggers(dsn.to_string(), error_prop.to_string()).await?;
    // //assert_eq!(triggers.len(), original_count);
    // println!("Fetched {} {} triggers back, should be eq to {}", triggers.len(), error_prop, original_count);

    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Trigger tests passed!"),
        Err(err) => println!("Trigger tests failed: {}", err),
    }
}