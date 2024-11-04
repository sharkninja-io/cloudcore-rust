use cloudcore::pairing::wifi_network::WifiNetwork;
use cloudcore::WifiPairing;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;
use std::process::Command;
use std::sync::Mutex;
use std::{str, thread, time::Duration};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Instant;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use tokio::time::sleep;
use cloudcore::examples::utils::get_cloudcore;
use cloudcore::wifi_manager::{handle_wifi_network};

static DEBUG_GATEWAY_IP: &str = "192.168.0.42";

static SSID_PATTERN: &str = "Shark_RV";

static SUPPORTED_PLATFORMS: &[&str] = &["macos"];

lazy_static! {
    static ref DONE: Mutex<bool> = Mutex::new(false);
    static ref PAIRED_DSN: Mutex<String> = Mutex::new("".to_string());
    static ref ERROR_MSG: Mutex<String> = Mutex::new("".to_string());
    static ref MANAGER_ADDR: Mutex<usize> = Mutex::new(0);
}

#[allow(dead_code)]
enum ConnectProcess {
    Full,
    Half,
    Done,
}

static CONNECT_PROCESS: ConnectProcess = ConnectProcess::Full;

#[tokio::main]
async fn run_happy_path_real_tests() -> Result<String, Box<dyn Error>> {
    let start = Instant::now();
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
    let user_session = cloudcore.user_session.as_ref();
    println!("User session: {:#?}", user_session);
    let token = user_session.unwrap().access_token().to_string();
    let manager = WifiPairing::new(cloudcore.session_params().device_url.to_owned());
    let boxed = Box::into_raw(Box::new(manager));
    *MANAGER_ADDR.lock().unwrap() = boxed as usize;
    let wifi_manager = unsafe { &mut *boxed };
    wifi_manager.configure(
        Box::new(|state| {
            println!("Pairing is now at: {:?}", state)
        }),
        Box::new(|networks| {
            let m = *MANAGER_ADDR.lock().unwrap();
            get_wifi_network(AtomicUsize::new(m), networks);
        }),
        Box::new(|result| {
            completed(result)
        }),
        Some(token),
    );
    get_device_ip_address(AtomicUsize::new(boxed as usize));
    let mut done = false;
    while !done {
        println!("Pairing...");
        // Poll every second
        sleep(Duration::from_secs(1)).await;
        done = *DONE.lock().unwrap();
    }
    let duration = start.elapsed();
    println!("Pairing took {} seconds", duration.as_secs());
    let error = ERROR_MSG.lock().unwrap();
    if error.is_empty() {
        Ok(PAIRED_DSN.lock().unwrap().to_owned())
    } else {
        Err((*error).clone().into())
    }
}

fn connect_to_device(ssid_pattern: String) -> Result<String, Box<dyn Error>> {
    if !SUPPORTED_PLATFORMS.contains(&env::consts::OS) {
        Err(format!(
            "Running on {}. Not running on supported platform: {:#?}",
            env::consts::OS,
            SUPPORTED_PLATFORMS
        )
            .into())
    } else {
        let network_device = "en0";
        // Requires the robot's AP to be in the preferred wifi list
        let script = "cloudcore/examples/get_wireless_networks.sh".to_string();
        let _ = Command::new("chmod")
            .args(&["0755", &script])
            .output()
            .unwrap()
            .stdout;
        let command = Command::new("sh")
            .args(&[&script, network_device, &ssid_pattern])
            .output();
        let buff = command.unwrap().stdout;
        let networks = str::from_utf8(&buff)
            .unwrap()
            .lines()
            .collect::<Vec<&str>>();
        println!("Found networks: {:#?}", networks);
        let first_network = networks.first();
        if let Some(network) = first_network {
            let network = network.trim();
            let status = Command::new("networksetup")
                .args(&["-setairportnetwork", network_device, network])
                .status()
                .unwrap();
            if !status.success() {
                completed(Err(format!(
                    "Failed to connect to {} Error status: {}",
                    network, status
                )
                    .into()));
                panic!();
            }
            println!("Connected to {} wireless network", network);
            get_gateway()
        } else {
            Err(format!("No preferred wireles networks matching {} pattern", ssid_pattern).into())
        }
    }
}

#[allow(dead_code)]
fn get_ip() -> Result<String, Box<dyn Error>> {
    if !SUPPORTED_PLATFORMS.contains(&env::consts::OS) {
        Err(format!(
            "Running on {}. Not running on supported platform: {:#?}",
            env::consts::OS,
            SUPPORTED_PLATFORMS
        )
            .into())
    } else {
        Ok(str::from_utf8(
            &Command::new("ipconfig")
                .args(&["getifaddr", "en0"])
                .output()
                .unwrap()
                .stdout,
        )
            .unwrap()
            .trim()
            .to_string())
    }
}

fn get_gateway() -> Result<String, Box<dyn Error>> {
    if !SUPPORTED_PLATFORMS.contains(&env::consts::OS) {
        Err(format!(
            "Running on {}. Not running on supported platform: {:#?}",
            env::consts::OS,
            SUPPORTED_PLATFORMS
        )
            .into())
    } else {
        let script = "cloudcore/examples/get_gateway.sh".to_string();
        let _ = Command::new("chmod")
            .args(&["0755", &script])
            .output()
            .unwrap()
            .stdout;
        let command = Command::new("sh").arg(&script).output();
        let output = command.unwrap().stdout;
        Ok(str::from_utf8(&output.to_owned())
            .unwrap()
            .trim()
            .to_string())
    }
}

fn get_device_ip_address(addr_wifi_manager: AtomicUsize) {
    thread::spawn(move || {
        println!(
            "Doing work to find device with pattern {}.....",
            SSID_PATTERN
        );
        let ip_address = match CONNECT_PROCESS {
            ConnectProcess::Full => connect_to_device(SSID_PATTERN.to_string()),
            ConnectProcess::Half => get_gateway(),
            ConnectProcess::Done => Ok(DEBUG_GATEWAY_IP.to_string()),
        };
        match ip_address {
            Ok(ip_address) => {
                println!("Connected to device with gateway IP: {}", &ip_address);
                let ptr_manager = addr_wifi_manager.load(Relaxed) as *mut WifiPairing;
                let manager = unsafe { &mut *ptr_manager };
                manager.start(ip_address);
            }
            Err(error) => completed(Err(error)),
        }
    });
}

fn get_wifi_network(addr_wifi_manager: AtomicUsize, wifi_networks: Vec<WifiNetwork>) {
    thread::spawn(move || {
        let args: Vec<String> = env::args().collect();
        if args.len() < 5 {
            completed(Err(
                "Need the name of the SSID to choose and it's password!".into(),
            ))
        } else {
            let ssid = &args[3];
            let ssid_pw = &args[4];
            if let Some(selected_network) = wifi_networks
                .into_iter()
                .find(|network| network.ssid().unwrap() == ssid) {
                let mut selected_network = selected_network
                    .clone();
                println!("Chose network: {:?}", &selected_network);
                selected_network.set_password(ssid_pw.to_string());
                let ptr_manager = addr_wifi_manager.load(Relaxed) as *mut WifiPairing;
                let manager = unsafe { &mut *ptr_manager };
                handle_wifi_network(manager, selected_network)
            }
        }
    });
}

fn completed(result: Result<String, Box<dyn Error>>) {
    if let Err(err) = result {
        *ERROR_MSG.lock().unwrap() = err.to_string();
    } else {
        *PAIRED_DSN.lock().unwrap() = result.unwrap();
    }
    *DONE.lock().unwrap() = true;
}

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    match run_happy_path_real_tests() {
        Ok(_) => println!("Pairing tests passed!"),
        Err(err) => println!("Pairing tests failed: {}", err),
    }
}
