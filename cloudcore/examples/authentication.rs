use std::error::Error;
use std::env;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::{examples::utils::get_cloudcore, cache::CacheInteract};
use cloudcore::authentication::CACHE_USER_DIR;
use cloudcore::cache::CacheDataValue;
use cloudcore::cloudcore::SELECTED_REGION_CACHE_KEY;

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(
            "Need email and password arguments".into(),
        );
    }
    if args.len() > 3 {
        cloudcore.cache.set_value(CACHE_USER_DIR.to_string(), "countryRegionSelection".to_string(), &args[3])?;
    }
    let email = &args[1];
    let pw = &args[2];
    if false {
        println!("Have valid user session: {:#?}", &cloudcore.user_session.as_ref().unwrap());
    } else {
        cloudcore
            .login(
                Some(email.to_string()),
                None,
                pw.to_string(),
            )
            .await?;
        println!("User session: {:#?}", &cloudcore.user_session);
        if !&cloudcore.logged_in() {
            return Err("not logged in after just logging in!?".into());
        }
    }
    cloudcore.logout().await?;
    cloudcore.refresh_session().await?;
    println!("New user session: {:#?}", &cloudcore.user_session);
    if !&cloudcore.logged_in() {
        return Err("not logged in after just refreshing session!?".into());
    }
    let profile = cloudcore.get_user_profile().await?;
    println!("User profile: {:#?}", profile);
    // Just to help populate cache directory
    cloudcore.fetch_all_devices().await?;
    cloudcore.logout().await?;
    if cloudcore.logged_in() {
        return Err("still logged in after just logging out?".into());
    }
    if args.len() > 3 {
        let country_region_selection = cloudcore.cache.get_value(CACHE_USER_DIR.to_string(), SELECTED_REGION_CACHE_KEY.to_string())?;
        match country_region_selection {
            CacheDataValue::StringValue(string) => {
                if string.as_str() !=  &args[3] {
                    return Err("current Selected Country Region Selection did not match initial one".into())
                }
            }
            _ => return Err("Selected Country Region Selection did not persist".into())
        }
    }
    cloudcore
            .login(
                Some(email.to_string()),
                None,
                pw.to_string(),
            )
            .await?;
    println!("User session: {:#?}", &cloudcore.user_session);
    if !&cloudcore.logged_in() {
        return Err("not logged in after just logging in!?".into());
    }
    // Just to test populating cache directory
    cloudcore.fetch_all_devices().await?;
    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Authentication tests passed!"),
        Err(err) => println!("Authentication tests failed: {}", err),
    }
}
