#[cfg(feature = "library")]
use std::time::Duration;
#[cfg(feature = "library")]
use log::{debug, error};
#[cfg(feature = "library")]
use mantle_utilities::RUNTIME;
#[cfg(feature = "library")]
use tokio::task::JoinHandle;
#[cfg(feature = "library")]
use tokio::time::sleep;
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "signatures")]
use crate::properties::property::IoTProperty;

#[cfg(feature = "library")]
#[derive(Debug)]
pub struct Poll {
    #[allow(dead_code)]
    id: u32,
    property_names: Vec<String>,
    on: bool,
    running: bool,
    handle: Option<JoinHandle<()>>,
    sleep_time: u64,
    dsn: String,
    callback: fn(String, Vec<IoTProperty>),
    cloudcore: &'static CloudCore
}

#[cfg(feature = "library")]
impl Poll {
    pub fn new(new_id: u32, config: PollConfig, cloudcore: &'static CloudCore) -> Self {
        let mut poll = Self {
            id: new_id,
            property_names: vec![],
            on: true,
            running: false,
            handle: None,
            sleep_time: 5000,
            dsn: "".to_string(),
            callback: default_callback,
            cloudcore
        };
        poll.update(config);
        poll
    }

    pub fn update(&mut self, config: PollConfig) {
        if let Some(props_names) = config.property_names {
            if !props_names.is_empty() {
                self.property_names = props_names
            }
        }
        if let Some(on) = config.on {
            self.set_on(on)
        }
        if let Some(sleep_time) = config.sleep_time {
            self.sleep_time = sleep_time
        }
        if let Some(dsn) = config.dsn {
            self.dsn = dsn
        }
        if let Some(callback) = config.callback {
            self.callback = callback
        }
    }

    pub fn run(&'static mut self) -> JoinHandle<()> {
        RUNTIME.spawn(async move {
            if self.running {
                return;
            }
            while self.on {
                self.running = true;
                debug!("Awaiting properties for {}", self.dsn());
                match self.cloudcore.get_properties(self.dsn.clone(), self.property_names.clone(), "".to_string()).await.0 {
                    Ok(props) => (self.callback)(self.dsn.clone(), props),
                    Err(err) => error!("Error polling for properties: {:#?}", err)
                }
                debug!("Finished properties poll for {}. Sleeping for {} secs", self.dsn(), self.sleep_time()/1000);
                if self.on {
                    sleep(Duration::from_millis(self.sleep_time())).await;
                    if !self.on {
                        self.running = false;
                    }
                }
            }
            self.running = false;
        })
    }

    pub fn sleep_time(&self) -> u64 {
        self.sleep_time
    }
    pub fn dsn(&self) -> &str {
        &self.dsn
    }
    pub fn set_on(&mut self, on: bool) {
        self.on = on;
        if !on {
            if let Some(handle) = &self.handle {
                handle.abort();
            }
            self.running = false;
        }
    }
    pub fn set_handle(&mut self, handle: JoinHandle<()>) {
        self.handle = Some(handle);
    }
}

#[cfg(feature = "library")]
fn default_callback(_dsn: String, _prop_names: Vec<IoTProperty>) {}

#[cfg(feature = "signatures")]
pub struct PollConfig {
    property_names: Option<Vec<String>>,
    on: Option<bool>,
    sleep_time: Option<u64>,
    dsn: Option<String>,
    callback: Option<fn(String, Vec<IoTProperty>)>
}

#[cfg(feature = "signatures")]
impl PollConfig {
    pub fn new(property_names: Option<Vec<String>>,
               on: Option<bool>,
               sleep_time: Option<u64>,
               dsn: Option<String>,
               callback: Option<fn(String, Vec<IoTProperty>)>) -> Self {
        Self {
            property_names,
            on,
            sleep_time,
            dsn,
            callback
        }
    }
}