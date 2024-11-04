use std::env;
use std::error::Error;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::cache::CacheInteract;
use cloudcore::examples::utils::get_cloudcore;
use cloudcore::io::read_from_disk_to_string;

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    cloudcore.write_to_pairing_log("Hello 1".to_string())?;
    cloudcore.write_to_pairing_log("Hello 2".to_string())?;
    cloudcore.write_to_pairing_log("Hello 4".to_string())?;
    cloudcore.write_to_pairing_log("Hello 6".to_string())?;
    let contents = cloudcore.get_pairing_log()?;
    println!("pairing contents: {}", contents);
    let args: Vec<String> = env::args().collect();
    if cloudcore.logged_in() {
        println!("Have valid user session: {:#?}", &cloudcore.user_session.as_ref().unwrap());
    } else {
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
    }
    if args.len() < 4 {
        return Err(
            "Need a DSN".into(),
        );
    }

    let dsn = &args[3];
    let _name = cloudcore.cache.get_value(dsn.to_string(), "name".to_string())?;
    let _is_online = cloudcore.cache.get_value(dsn.to_string(), "is_online".to_string())?;

    let prop_name = "Mobile_App_Room_Definition";
    let datapoint_file = &cloudcore
        .get_file_property(
            dsn.to_owned(),
            prop_name.to_string(),
            "".to_string(),
        )
        .await.0?;
    println!("datapoint file for {}: {:#?}", prop_name, datapoint_file);
    if datapoint_file.local_file().is_none() {
        return Err(format!("local file path for {} is empty", prop_name).into())
    }
    match read_from_disk_to_string(datapoint_file.local_file().unwrap()) {
        Ok(contents) => {
            println!("contents of datapoint file: {:#?}", contents);
            if args.len() < 5 {
                return Err(
                    "Need another DSN".into(),
                );
            }
            let other_dsn = &args[4];
            cloudcore.cache.set_value(other_dsn.to_string(), "dsn".to_string(), dsn)?;
            let dsn = cloudcore.cache.get_value(other_dsn.to_string(), "dsn".to_string())?;

            println!("{:?}", dsn);

            Ok(())
        },
        Err(err) => {
            Err(err)
        }
    }
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Cache tests passed!"),
        Err(err) => println!("Cache tests failed: {}", err),
    }
}