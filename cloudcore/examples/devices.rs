use std::error::Error;
use std::env;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::CloudCore;
use cloudcore::devices::IoTDevice;
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
    let mut dsn = "".to_string();
    if args.len() > 3 {
        dsn = args[3].clone();
    }
    if dsn.is_empty() {
        let devices = cloudcore.fetch_all_devices().await?;
        if !devices.is_empty() {
            dsn = devices.first().unwrap().dsn().unwrap().to_string();
        }
    }
    let device = cloudcore.fetch_device_with_dsn(dsn.clone()).await?;
    println!("Device with dsn {}: {:#?}", dsn, device);
    let _ = cloudcore.set_device_time_zone(dsn).await?;
    //rename_tests(cloudcore, &device).await?;
    //let _ = cloudcore.delete_device_map(dsn.clone(), false, true).await?;
    if let Some(key) = device.id() {
        //let _ = cloudcore.delete_device(key, dsn.clone()).await?;
       //let _ = cloudcore.set_device_time_zone(key).await?;
    }
    Ok(())
}

async fn rename_tests(cloudcore: &mut CloudCore, device: &IoTDevice) -> Result<(), Box<dyn Error>> {
    let og_name = device.product_name().unwrap();
    let dsn = device.dsn();
    let new_name = "Robot Guy";
    let dsn = dsn.unwrap();
    let _ = cloudcore.rename_device_with_dsn(dsn.to_string(), new_name.to_string()).await?;
    println!("Renamed device {} with dsn {} to {}", og_name, dsn, new_name);
    let device = cloudcore.fetch_device_with_dsn(dsn.to_string()).await?;
    println!("Device with dsn {}: {:#?}", dsn, device);
    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Devices tests passed!"),
        Err(err) => println!("Devices tests failed: {}", err),
    }
}
