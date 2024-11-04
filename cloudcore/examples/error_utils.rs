use std::error::Error;
use std::env;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::CloudCore;
use cloudcore::examples::utils::get_cloudcore;

#[tokio::main]
async fn run_sad_fetch_devices() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let devices = cloudcore.fetch_device_with_dsn("".to_string()).await;
    match devices {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            let error_value: &str = "ServerError";
            println!("Returned Error: {}", err);
            if error_value.eq(&err.to_string()) {
                return Ok(());
            } else {
                return Err("MantleError not parsed".into());
            }
        }
    }
}

#[tokio::main]
async fn run_sad_login_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let login = cloudcore.login(None, None, "".to_string()).await;
    match login {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            let error_value: &str = "EmailOrPhoneNumberMissing";
            println!("Returned Error: {}", err);
            if error_value.eq(&err.to_string()) {
                return Ok(());
            } else {
                return Err("MantleError not parsed".into());
            }
        }
    }
}

#[tokio::main]
async fn run_sad_request_password_reset_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let reset = cloudcore.request_password_reset(None, None, None, None, None).await;

    match reset {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            let error_value: &str = "EmailOrPhoneNumberMissing";
            println!("Returned Error: {}", err);
            if error_value.eq(&err.to_string()) {
                return Ok(());
            } else {
                return Err("MantleError not parsed".into());
            }
        }
    }
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_sad_login_tests() {
        Ok(_) => {
            match run_sad_request_password_reset_tests() {
                Ok(_) => {
                    match run_sad_fetch_devices() {
                        Ok(_) => println!("Error tests passed!"),
                        Err(err) => println!("Error tests failed: {}", err),
                    }
                }
                Err(err) => println!("Error tests failed: {}", err),
            }
        }
        Err(err) => println!("Error tests failed: {}", err),
    }
    
}