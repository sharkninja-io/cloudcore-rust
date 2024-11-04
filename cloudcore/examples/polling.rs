use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::examples::utils::get_cloudcore;
use cloudcore::polling::PollConfig;
use cloudcore::properties::property::IoTProperty;

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
    let poll_1_props = vec![
        "GET_Operating_Mode".to_string(),
        "SET_Areas_To_Clean".to_string(),
        "GET_ExploreStatus".to_string(),
    ];
    let poll_1 = PollConfig::new(
        Some(poll_1_props),
        None,
        None,
        Some(dsn.to_string()),
        Some(handle_poll),
    );
    let poll_2_props = vec![
        "GET_Charging_Status".to_string(),
    ];
    let poll_2 = PollConfig::new(
        Some(poll_2_props),
        None,
        Some(8000),
        Some(dsn.to_string()),
        Some(handle_poll),
    );
    cloudcore.start_polling_manager();
    sleep(Duration::from_secs(3)); // Sleep for 3 seconds before adding a poll
    cloudcore.add_poll(poll_1);
    sleep(Duration::from_secs(2)); // Sleep for 2 seconds before adding another poll
    cloudcore.add_poll(poll_2);
    sleep(Duration::from_secs(20)); // Sleep for 20 seconds to poll
    println!("About to stop polling");
    cloudcore.stop_polling_manager();
    sleep(Duration::from_secs(20)); // Sleep for 20 seconds to poll
    println!("About to start polling");
    cloudcore.start_polling_manager();
    sleep(Duration::from_secs(20)); // Sleep for 20 seconds to poll
    Ok(())
}

fn handle_poll(dsn: String, props: Vec<IoTProperty>) {
    println!("Polled {} and got back: {:?}", dsn, props);
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Polling tests passed!"),
        Err(err) => println!("Polling tests failed: {}", err),
    }
}