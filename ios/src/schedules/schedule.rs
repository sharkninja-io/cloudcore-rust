use log::debug;
use std::os::raw::{c_char, c_uint};
use ffi_utilities::{CRustBridge, MantleString, RustCBridge, MantleList, MantleOptionString, MantleStringPointer};

#[repr(C)]
#[derive(Debug)]
pub struct Schedule {
    active: bool,
    day_occur_of_month: *const MantleList<u32>, 
    days_of_month: *const MantleList<u32>,
    days_of_week: *const MantleList<u32>,
    device_id: *const c_uint,
    direction: *const c_char,
    display_name: *const c_char,
    duration: *const u32,
    end_date: *const c_char,
    end_time_each_day: *const c_char,
    fixed_actions: bool,
    interval: *const u32,
    months_of_year: *const MantleList<u32>,
    name: *const c_char,
    start_date: *const c_char,
    start_time_each_day: *const c_char,
    key: *const u32,
    time_before_end: *const c_char,
    utc: bool,
    version: *const c_char
}

impl RustCBridge<cloudcore::schedules::Schedule> for Schedule {
    fn new_c_object(rust_schedule: &cloudcore::schedules::Schedule) -> Self {
        Self {
            active: match rust_schedule.active() {
                Some(value) => value,
                None => false,
            },
            day_occur_of_month: match rust_schedule.day_occur_of_month() {
                Some(value) => {
                    let array = MantleList::<u32>::from_list(value.to_vec());
                    let boxed = Box::new(array);
                    Box::into_raw(boxed)
                },
                None => std::ptr::null(),
            },
            days_of_month: match rust_schedule.days_of_month() {
                Some(value) => {
                    let array = MantleList::<u32>::from_list(value.to_vec());
                    let boxed = Box::new(array);
                    Box::into_raw(boxed)
                },
                None => std::ptr::null(),
            },
            days_of_week:  match rust_schedule.days_of_week() {
                Some(value) => {
                    let array = MantleList::<u32>::from_list(value.to_vec());
                    let boxed = Box::new(array);
                    Box::into_raw(boxed)
                },
                None => std::ptr::null(),
            },
            device_id: match rust_schedule.device_id() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null()
            },
            direction: MantleString(rust_schedule.direction().to_owned()).to_ptr(),
            display_name: MantleOptionString(rust_schedule.display_name().to_owned()).to_ptr(),
            duration: match rust_schedule.duration() {
                Some(value) => value as *const u32,
                None => std::ptr::null(),
            },
            end_date: MantleOptionString(rust_schedule.end_date().to_owned()).to_ptr(),
            end_time_each_day: MantleOptionString(rust_schedule.end_time_each_day().to_owned()).to_ptr(),
            fixed_actions: match rust_schedule.fixed_actions() {
                Some(value) => value,
                None => false,
            },
            interval: match rust_schedule.interval() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null(),
            },
            months_of_year: match rust_schedule.months_of_year() {
                Some(value) => {
                    let array = MantleList::<u32>::from_list(value.to_vec());
                    let boxed = Box::new(array);
                    Box::into_raw(boxed)
                },
                None => std::ptr::null(),
            },
            name: MantleString(rust_schedule.name().to_owned()).to_ptr(),
            start_date: MantleString(rust_schedule.start_date().to_owned()).to_ptr(),
            start_time_each_day: MantleString(rust_schedule.start_time_each_day().to_owned()).to_ptr(),
            key: match rust_schedule.key() {
                Some(val) => Box::into_raw(Box::new(val)),
                None => std::ptr::null(),
            },
            time_before_end: MantleOptionString(rust_schedule.time_before_end().to_owned()).to_ptr(),
            utc: match rust_schedule.utc() {
                Some(value) => value,
                None => false,
            },
            version: MantleOptionString(rust_schedule.version().to_owned()).to_ptr()
        }
    }
}
impl CRustBridge<cloudcore::schedules::Schedule> for Schedule {
    unsafe fn new_rust_object(
        c_object_ptr: *const Self,
    ) -> Option<cloudcore::schedules::Schedule> {
        if c_object_ptr.is_null() {
            debug!("schedule pointer was null");
            Option::None
        } else {
            let obj_ref = & *c_object_ptr;
            let to_vec = |mantle_list: *const MantleList<u32>| {
                let c_list = *Box::from_raw(mantle_list as *mut MantleList<*const u32>);
                let list = c_list.map_list(|v|{
                    let v = Box::from_raw(v as *mut u32);
                    *v
                });
                list
            };
            let schedule = cloudcore::schedules::Schedule::new(
                Some(obj_ref.active), 
                if obj_ref.day_occur_of_month.is_null() {
                    None
                } else {
                    let list = to_vec(obj_ref.day_occur_of_month);
                    Some(list)
                },
                if obj_ref.days_of_month.is_null() {
                    None
                } else {
                    let list = to_vec(obj_ref.days_of_month);
                    Some(list)
                },
                if obj_ref.days_of_week.is_null() {
                    None
                } else {
                    let list = to_vec(obj_ref.days_of_week);
                    Some(list)
                }, 
                if obj_ref.device_id.is_null() {
                    None
                } else {
                    let device_id = Box::from_raw(obj_ref.device_id as *mut u32);
                    Some(*device_id)
                },
                MantleStringPointer(obj_ref.direction).to_string(),
                MantleStringPointer(obj_ref.display_name).to_option_string(),
                if obj_ref.duration.is_null() {
                    None
                } else {
                    let value = Box::from_raw(obj_ref.duration as *mut u32);
                    Some(*value)
                }, 
                MantleStringPointer(obj_ref.end_date).to_option_string(),
                MantleStringPointer(obj_ref.end_time_each_day).to_option_string(), 
                Some(obj_ref.fixed_actions), 
                if obj_ref.interval.is_null() {
                    None
                } else {
                    let value = Box::from_raw(obj_ref.interval as *mut u32);
                    Some(*value)
                }, 
                if obj_ref.months_of_year.is_null() {
                    None
                } else {
                    let list = to_vec(obj_ref.months_of_year);
                    Some(list)
                }, 
                MantleStringPointer(obj_ref.name).to_string(),
                MantleStringPointer(obj_ref.start_date).to_string(),
                MantleStringPointer(obj_ref.start_time_each_day).to_string(), 
                if obj_ref.key.is_null() {
                    None
                } else {
                    let value = Box::from_raw(obj_ref.key as *mut u32);
                    Some(*value)
                },
                MantleStringPointer(obj_ref.time_before_end).to_option_string(),
                Some(obj_ref.utc), 
                MantleStringPointer(obj_ref.version).to_option_string());
            Some(schedule)
        }
    }
}

