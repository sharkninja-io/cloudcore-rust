use cloudcore::CloudCore;
use cloudcore::polling::PollConfig;

#[no_mangle]
pub unsafe extern "C" fn cloudcore_start_polling_manager(
    ptr_cloudcore: *mut CloudCore,
) {
    let cloudcore = &mut *ptr_cloudcore;
    cloudcore.start_polling_manager();
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_add_poll(
    ptr_cloudcore: *mut CloudCore,
    poll_config: *mut PollConfig,
) -> u32 {
    let cloudcore = &mut *ptr_cloudcore;
    let config = *Box::from_raw(poll_config);
    cloudcore.add_poll(config)
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_update_poll(
    ptr_cloudcore: *mut CloudCore,
    poll_id: *mut u32,
    new_config: *mut PollConfig,
) {
    let cloudcore = &mut *ptr_cloudcore;
    let config = *Box::from_raw(new_config);
    let id = *Box::from_raw(poll_id);
    cloudcore.update_poll(id, config);
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_remove_poll(
    ptr_cloudcore: *mut CloudCore,
    poll_id: *mut u32,
) {
    let cloudcore = &mut *ptr_cloudcore;
    let id = *Box::from_raw(poll_id);
    cloudcore.remove_poll(id);
}

#[no_mangle]
pub unsafe extern "C" fn cloudcore_stop_polling_manager(
    ptr_cloudcore: *mut CloudCore,
) {
    let cloudcore = &mut *ptr_cloudcore;
    cloudcore.stop_polling_manager();
}