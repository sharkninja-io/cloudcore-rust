#[cfg(feature = "library")]
use std::collections::HashMap;
#[cfg(feature = "library")]
use lazy_static::lazy_static;
#[cfg(feature = "library")]
use std::sync::Mutex;
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "library")]
use crate::polling::poll::{Poll, PollConfig};
#[cfg(feature = "library")]
use mantle_utilities::to_static_ref;

#[cfg(feature = "library")]
lazy_static! {
    pub static ref POLL_MANAGER: Mutex<usize> = {
        let manager = PollManager::new();
        let boxed = Box::into_raw(Box::new(manager));
        let addr = boxed as usize;
        Mutex::new(addr)
    };
}

#[cfg(feature = "library")]
impl POLL_MANAGER {
    pub fn get_static_ref() -> &'static mut PollManager {
        let addr = *POLL_MANAGER.lock().unwrap();
        to_static_ref::<PollManager>(addr)
    }
}

#[cfg(feature = "library")]
pub struct PollManager {
    poll_map: HashMap<u32, usize>,
    polling: bool,
    next_poll_id: u32
}

#[cfg(feature = "library")]
impl PollManager {
    fn new() -> Self {
        Self {
            poll_map: HashMap::new(),
            polling: false,
            next_poll_id: 0
        }
    }

    fn next_poll_id(&mut self) -> u32 {
        self.next_poll_id += 1;
        self.next_poll_id
    }

    pub fn start_polling(&'static mut self) {
        if !self.polling {
            self.polling = true;
            for (_, address) in &self.poll_map {
                PollManager::start_poll(*address);
            }
        }
    }

    pub fn stop_polling(&mut self) {
        self.polling = false;
        for (_, address) in &mut self.poll_map {
            let poll = to_static_ref::<Poll>(*address);
            poll.set_on(false)
        }
    }

    pub fn add_poll(&mut self, config: PollConfig, cloudcore: &'static CloudCore) -> u32 {
        let poll_id = self.next_poll_id();
        let poll = Box::new(Poll::new(poll_id, config, cloudcore));
        let raw = Box::into_raw(poll);
        let addr = raw as usize;
        self.poll_map.insert(poll_id, addr);
        if self.polling {
            PollManager::start_poll(addr);
        }
        poll_id
    }

    fn start_poll(address: usize) {
        let poll_for_handle = to_static_ref::<Poll>(address);
        poll_for_handle.set_on(true);
        let handle = poll_for_handle.run();
        let poll = to_static_ref::<Poll>(address);
        poll.set_handle(handle);
    }

    pub fn update_poll(&mut self, poll_id: u32, new_config: PollConfig) {
        if let Some(address) = self.poll_map.get(&poll_id) {
            let poll = to_static_ref::<Poll>(*address);
            poll.update(new_config);
        }
    }

    pub fn remove_poll(&mut self, poll_id: u32) {
        if let Some(address) = self.poll_map.get(&poll_id) {
            let ptr = *address as *mut Poll;
            let mut poll = unsafe { *Box::from_raw(ptr) };
            poll.set_on(false);
            self.poll_map.remove(&poll_id);
        }
    }
}