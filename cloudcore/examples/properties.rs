use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use cloudcore::properties::{value::IoTPropertyValue, datapoint};
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::examples::utils::get_cloudcore;

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
    println!("User session: {:#?}", &cloudcore.user_session);
    if args.len() < 4 {
        return Err(
            "Need a DSN".into(),
        );
    }
    let dsn = &args[3];

    /*let from = "2022-03-21";
    let to = "2022-04-21";
    let datapoints = cloudcore.get_file_property_as_files(
        dsn.to_owned(), "GET_Visual_Floor_1".to_string(),
        Some(10),
        Some(from.to_string()),
        Some(to.to_string()),
    ).await?;
    println!("Got {} file datapoints between {} and {} back: {:?}", datapoints.len(), from, to, datapoints);*/


    // Since we have the DSN -> Device Serial Number, we can
    // pass it in through our network request and perform
    // our CRUD operations.
    let properties = cloudcore
        .get_properties(
            dsn.to_owned(),
            vec![
                "GET_Operating_Mode".to_string(),
                "GET_Battery_Capacity".to_string(),
                "GET_Charging_Status".to_string(),
                "GET_Default_Power_Mode".to_string(),
                "OTA_FW_VERSION".to_string(),
                "GET_Device_Model_Number".to_string(),
            ],
            "".to_string()
        )
        .await.0?;

    // We iterate through the properties that were
    // fetched.
    // It will contain all necessary info about the property.
    // We can use decoding style as getting the value
    // by casting it or just from the enum directly
    properties.iter().for_each(|property| {
        if let Some(property_value) = property.value() {
            match property_value {
                IoTPropertyValue::Int(it) => {
                    println!(
                        "This is an i32 -> {:?} {:?}",
                        it,
                        property_value.int_value()
                    );
                }
                IoTPropertyValue::Str(it) => {
                    println!(
                        "This is an String -> {:?} {:?}",
                        it,
                        property_value.string_value()
                    );
                }
                IoTPropertyValue::Bool(it) => {
                    println!(
                        "This is an bool -> {:?} {:?}",
                        it,
                        property_value.bool_value()
                    );
                }
            }
        }
    });

    let file = cloudcore.get_file_property(dsn.to_owned(), "Mobile_App_Room_Definition".to_string(),"".to_string()).await.0?;
    println!("Got file: {:#?}", file);

    let _ = cloudcore.save_file(
        dsn.to_owned(),
        "Mobile_App_Room_Definition".to_string(),
        //"cloudcore/cloudcore/test-mard.json".to_string(),
        file.local_file().unwrap().to_string(),
        false,
        "".to_string(),
    ).await.0?;
    println!("Created cloud file");

    /*let message = cloudcore.get_message_property(dsn.to_owned(), "GET_Cleaning_Statistics".to_string(), "".to_string()).await.0?;
    println!("Got message: {:#?}", message);*/

    cloudcore
        .set_property_value(dsn.to_owned(), "SET_Operating_Mode".to_string(), IoTPropertyValue::Int(3), "".to_string())
        .await.0?;
    println!("Operating Mode set");

    let value = "{\"clean_count\":1,\"areas_to_clean\":[\"UltraClean:Room 2\"]}";

    cloudcore
        .set_property_value(dsn.to_owned(), "SET_Areas_To_Clean".to_string(), IoTPropertyValue::Str(value.to_owned()), "".to_string())
        .await.0?;
    println!("Areas To Clean set");

    let datapoints = cloudcore.get_datapoints(
        dsn.to_owned(), "SET_Operating_Mode".to_string(),
        None,
        None,
        None,
        "".to_string()
    ).await.0?;
    println!("Got {} datapoints back: {:?}", datapoints.len(), datapoints);

    let limit = 10;
    let datapoints = cloudcore.get_datapoints(
        dsn.to_owned(), "SET_Operating_Mode".to_string(),
        Some(10),
        None,
        None,
        "".to_string()
    ).await.0?;
    println!("Got {}=={} datapoints back: {:?}", limit, datapoints.len(), datapoints);

    let from = "2021-12-01";
    let to = "2022-01-01";
    let datapoints = cloudcore.get_datapoints(
        dsn.to_owned(), "SET_Operating_Mode".to_string(),
        Some(10),
        Some(from.to_string()),
        Some(to.to_string()),
        "".to_string()
    ).await.0?;
    println!("Got {} datapoints between {} and {} back: {:?}", datapoints.len(), from, to, datapoints);

    let from = "2022-07-23";
    let to = "2022-08-22";
    let _ = cloudcore.get_file_property_as_files_callback(
        dsn.to_owned(), "GET_Cleaning_Statistics".to_string(),
        Some(10),
        Some(from.to_string()),
        Some(to.to_string()),
        "".to_string(),
        |result| {
            if let Some(datapoints) = result.0.ok() {
                println!("Got {} files for message property back: {:?}", datapoints.len(), datapoints);
            }
        }
    ).await;
    sleep(Duration::from_secs(30));
    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Properties tests passed!"),
        Err(err) => println!("Properties tests failed: {}", err),
    }
}