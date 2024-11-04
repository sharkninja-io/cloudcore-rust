use std::error::Error;
use std::{env, io};
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::cache::CacheInteract;
use cloudcore::examples::utils::get_cloudcore;

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<(), Box<dyn Error>> {
    let cloudcore = unsafe { get_cloudcore() };
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(
            "Need email and phone number arguments".into(),
        );
    }
    let test_email = &args[1];
    let test_number = &args[2];
    let use_phone = &args[3] == "phone";
    let email = if !use_phone { Some(test_email.to_string()) } else { None };
    let phone_number = if use_phone { Some(["+1", test_number].concat()) } else { None };
    let pw = "@Password123";
    let new_pw = "@Password12345";
    cloudcore
        .create_account(
            pw.to_string(),
            email.clone(),
            phone_number.clone(),
            None,
            None,
            None,
        )
        .await?;
    cloudcore
        .send_confirmation_instructions(email.clone(),
                                        phone_number.clone(),
                                        None,
                                        None,
                                        None,
        )
        .await?;
    println!("Enter confirm account token");
    let mut confirm_token = String::new();
    io::stdin()
        .read_line(&mut confirm_token)
        .expect("error: unable to read confirm token");
    confirm_token = confirm_token.trim().to_string();
    cloudcore.confirm_account(confirm_token).await?;
    if use_phone {
        let _ = cloudcore.cache.set_value("user".to_string(), "countryRegionSelection".to_string(), "CN");
    }
    cloudcore
        .request_password_reset(email.clone(),
                                phone_number.clone(),
                                None,
                                None,
                                None,
        )
        .await?;
    println!("Enter reset password token");
    let mut reset_password_token = String::new();
    io::stdin()
        .read_line(&mut reset_password_token)
        .expect("error: unable to read reset password token");
    reset_password_token = reset_password_token.trim().to_string();
    cloudcore
        .reset_password(reset_password_token, new_pw.to_string(), new_pw.to_string())
        .await?;
    println!("reset password");
    cloudcore
        .login(email, phone_number.clone(), new_pw.to_string())
        .await?;
    println!("User session with new password: {:#?}", cloudcore.user_session.as_ref());
    if !use_phone {
        let new_email = "sntester123456789@gmail.com".to_string();
        cloudcore.update_email(new_email.to_owned()).await?;
        cloudcore
            .login(Some(new_email), None, new_pw.to_string())
            .await?;
        println!("User session with new email: {:#?}", cloudcore.user_session.as_ref());
    }
    cloudcore.delete_account().await?;
    Ok(())
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Account tests passed!"),
        Err(err) => println!("Account tests failed: {}", err),
    }
}
