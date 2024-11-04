use std::error::Error;
use std::env;
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
    let devices = cloudcore.fetch_all_devices().await?;
    if !devices.is_empty() {
        // Update a Schedule
        let dev = devices.first().unwrap();
        let device_id = Some(dev.id().unwrap());
        println!("Device device_id {:?}", device_id);

        // Fetch all device schedules
        let mut schedules = cloudcore.fetch_schedules(device_id).await?;
        println!("Device Schedules {:?}", schedules);

        let schedule = schedules.first_mut().unwrap();
        schedule.set_active(true);
        schedule.set_start_time_each_day("08:30:00".to_string());

        cloudcore.update_schedule(
            schedule.clone()
        ).await?;
    }
    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Schedule tests passed!"),
        Err(err) => println!("Schedule tests failed: {}", err),
    }
}