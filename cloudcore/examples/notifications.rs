use std::env;
use std::error::Error;

use log::LevelFilter;
use simplelog::{Config, SimpleLogger};

use cloudcore::examples::utils::get_cloudcore;

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<String, Box<dyn Error>> {
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
    if args.len() < 4 {
        return Err(
            "Need a DSN".into(),
        );
    }
    let dsn = &args[3];

    let all_notifications = cloudcore.fetch_all_notifications(
        "05/12/22".to_string()
    ).await?;
    println!("Fetched ALL {} notifications: {:?}", all_notifications.len(), all_notifications);

    let notifications = cloudcore.fetch_notifications(
        dsn.to_string(),
        "05/12/22".to_string(),
    ).await?;
    println!("Fetched {} notifications: {:?}", notifications.len(), notifications);


    let read_ok = cloudcore.mark_notifications_as_read(dsn.to_string()).await;
    println!("Marked all Notifications as READ: {}", read_ok);


    let to = "2022-05-24T03:36:55Z";
    cloudcore.delete_all_notifications(to.to_string()).await?;
    println!("Mark notifications up to {} as DELETED", to);

    let notifications = cloudcore.fetch_notifications(
        dsn.to_string(),
        "05/12/22".to_string(),
    ).await?;
    // NOTE: These should have all hit the cache and have read=true and deleted=true
    println!("Re-fetched {} notifications: {:?}", notifications.len(), notifications);

    Ok("".to_string())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Notifications tests passed!"),
        Err(err) => println!("Notification tests failed: {}", err),
    }
}