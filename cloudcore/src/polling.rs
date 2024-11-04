mod poll;
mod manager;

#[cfg(feature = "signatures")]
pub use poll::PollConfig;
#[cfg(feature = "library")]
use crate::CloudCore;
#[cfg(feature = "library")]
use manager::POLL_MANAGER;

#[cfg(feature = "library")]
impl CloudCore {
    pub fn start_polling_manager(&self) {
        POLL_MANAGER::get_static_ref().start_polling();
    }

    pub fn add_poll(&'static self, config: PollConfig) -> u32 {
        POLL_MANAGER::get_static_ref().add_poll(config, self)
    }

    pub fn update_poll(&self, poll_id: u32, new_config: PollConfig) {
        POLL_MANAGER::get_static_ref().update_poll(poll_id, new_config);
    }

    pub fn remove_poll(&self, poll_id: u32) {
        POLL_MANAGER::get_static_ref().remove_poll(poll_id);
    }

    pub fn stop_polling_manager(&self) {
        POLL_MANAGER::get_static_ref().stop_polling();
    }
}
