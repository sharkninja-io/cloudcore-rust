use android_utilities::java_class_names::CLASSNAMES;
use android_utilities::java_signatures::{BOOL_SIG, INT_SIG, INTEGER_SIG, STRING_SIG, VOID_SIG};
use android_utilities::{JavaClass, JObjectRustBridge};
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::option_traits::AndroidOption;
use android_utilities::jni_exts::string::AndroidString;
use ctor::ctor;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jint, jintArray, jobject, jsize};
use log::error;
use cloudcore::schedules::Schedule;
use crate::JAVA_PACKAGE;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaSchedule::full_name(None));
}

pub struct JavaSchedule(pub Schedule);
impl JavaClass<Schedule> for JavaSchedule {
    fn full_name(_instance: Option<&Self>) -> String {
        let mut name = JAVA_PACKAGE.to_owned();
        name.push_str("Schedule");
        name
    }

    fn signature(_instance: Option<&Self>) -> String {
        [
            "(",
            BOOL_SIG,
            format!("[{}",INT_SIG).as_str(),
            format!("[{}",INT_SIG).as_str(),
            format!("[{}",INT_SIG).as_str(),
            INTEGER_SIG,
            STRING_SIG,
            STRING_SIG,
            INTEGER_SIG,
            STRING_SIG,
            STRING_SIG,
            BOOL_SIG,
            INTEGER_SIG,
            format!("[{}",INT_SIG).as_str(),
            STRING_SIG,
            STRING_SIG,
            STRING_SIG,
            INTEGER_SIG,
            STRING_SIG,
            BOOL_SIG,
            STRING_SIG,
            ")",
            VOID_SIG,
        ]
            .concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaSchedule::signature(None);

        let active = match self.0.active() {
            Some(value) => JValue::Bool(value.into()),
            None => JValue::Bool(false.into())
        };

        let day_occur_of_month = int_vec_to_j_value(self.0.day_occur_of_month(), jni_env);
        let days_of_month = int_vec_to_j_value(self.0.days_of_month(), jni_env);
        let days_of_week = int_vec_to_j_value(self.0.days_of_week(), jni_env);

        let device_id = self.0.device_id().to_j_value(jni_env);
        let direction = AndroidString(self.0.direction().to_owned()).to_jstring(jni_env);
        let display_name = self.0.display_name().map(String::from).to_j_value(jni_env);
        let duration = self.0.duration().to_j_value(jni_env);
        let end_date = self.0.end_date().map(String::from).to_j_value(jni_env);
        let end_time_each_day = self.0.end_time_each_day().map(String::from).to_j_value(jni_env);
        let fixed_actions = match self.0.fixed_actions() {
            Some(value) => JValue::Bool(value.into()),
            None => JValue::Bool(false.into())
        };
        let interval = self.0.interval().to_j_value(jni_env);

        let months_of_year = int_vec_to_j_value(self.0.months_of_year(), jni_env);

        let name = AndroidString(self.0.name().to_owned()).to_jstring(jni_env);
        let start_date = AndroidString(self.0.start_date().to_owned()).to_jstring(jni_env);
        let start_time_each_day = AndroidString(self.0.start_time_each_day().to_owned()).to_jstring(jni_env);
        let key = self.0.key().to_j_value(jni_env);
        let time_before_end= self.0.time_before_end().map(String::from).to_j_value(jni_env);
        let utc = match self.0.utc() {
            Some(value) => JValue::Bool(value.into()),
            None => JValue::Bool(false.into())
        };
        let version= self.0.version().map(String::from).to_j_value(jni_env);

        // ** Order matters!!! Refer to com/sharkninja/cloudcore/Schedules **
        let args = &[
            active,
            day_occur_of_month,
            days_of_month,
            days_of_week,
            device_id,
            JValue::from(direction.into_inner()),
            display_name,
            duration,
            end_date,
            end_time_each_day,
            fixed_actions,
            interval,
            months_of_year,
            JValue::from(name.into_inner()),
            JValue::from(start_date.into_inner()),
            JValue::from(start_time_each_day.into_inner()),
            key,
            time_before_end,
            utc,
            version,
        ];
        let schedule_object = jni_env
            .new_object(j_class, signature, args)
            .unwrap_or_else(|err| {
                error!("Error creating Schedule for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *schedule_object
    }

    fn new(rust_object: Schedule) -> Self {
        Self(rust_object)
    }
}

impl JObjectRustBridge<Schedule> for JavaSchedule {
    fn rust_object(j_object: MantleJObject, env: JNIEnv) -> Option<Schedule> {
        if !j_object.0.is_null()
            && env
            .is_instance_of(j_object.0, JavaSchedule::full_name(None))
            .unwrap()
        {
            let active = j_object.to_optional_bool_field(env, "active");
            let day_occur_of_month = to_unsigned_int_vec(&j_object, env, "dayOccurOfMonth");
            let days_of_month = to_unsigned_int_vec(&j_object, env, "daysOfMonth");
            let days_of_week = to_unsigned_int_vec(&j_object, env, "daysOfWeek");
            let device_id = j_object.to_optional_unsigned_int_field(env, "deviceID");
            let direction = j_object.to_string_field(env, "direction");
            let display_name = j_object.to_optional_string_field(env, "displayName");
            let duration = j_object.to_optional_unsigned_int_field(env, "duration");
            let end_date = j_object.to_optional_string_field(env, "endDate");
            let end_time_each_day= j_object.to_optional_string_field(env, "endTimeEachDay");
            let fixed_actions = j_object.to_optional_bool_field(env, "fixedActions");
            let interval = j_object.to_optional_unsigned_int_field(env, "interval");
            let months_of_year = to_unsigned_int_vec(&j_object, env, "monthsOfYear");
            let name = j_object.to_string_field(env, "name");
            let start_date = j_object.to_string_field(env, "startDate");
            let start_time_each_day= j_object.to_string_field(env, "startTimeEachDay");
            let key= j_object.to_optional_unsigned_int_field(env, "key");
            let time_before_end= j_object.to_optional_string_field(env, "timeBeforeEnd");
            let utc = j_object.to_optional_bool_field(env, "utc");
            let version= j_object.to_optional_string_field(env, "version");

            let schedule = Schedule::new(
                active,
                day_occur_of_month,
                days_of_month,
                days_of_week,
                device_id,
                direction,
                display_name,
                duration,
                end_date,
                end_time_each_day,
                fixed_actions,
                interval,
                months_of_year,
                name,
                start_date,
                start_time_each_day,
                key,
                time_before_end,
                utc,
                version,
            );
            Some(schedule)
        } else {
            None
        }
    }
}

// Helper functions to transform between Vec<u32> and jintArray and JValue
pub fn int_vec_to_j_value<'a>(vec: Option<&Vec<u32>>, jni_env: JNIEnv) -> JValue<'a> {
    match vec {
        None => JValue::from(JObject::null()),
        Some(vec) => {
            let array = jni_env.new_int_array(vec.len() as jsize)
                .unwrap_or_else(|err| {
                    error!("Could not create int array: {:?}", err);
                    jni_env.exception_describe().unwrap();
                    panic!();
                });
            let jint_array = vec.into_iter().map(|int| *int as jint).collect::<Vec<jint>>();
            jni_env.set_int_array_region(array, 0, jint_array.as_slice())
                .unwrap_or_else(|err| {
                    error!("Could not copy vec contents into int array: {:?}", err);
                    jni_env.exception_describe().unwrap();
                    panic!();
                });
            JValue::from(JObject::from(array))
        }
    }
}

pub fn j_object_to_j_int_array(mantle_j_object: &MantleJObject, jni_env: JNIEnv, name: &str, sig: &str) -> jintArray {
    *jni_env
        .get_field(mantle_j_object.0, name, format!("[{}", sig))
        .unwrap_or_else(|err| {
            error!("Error getting array field: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
        .l()
        .unwrap_or_else(|err| {
            error!("Error converting field to j object array: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        }) as jintArray
}

pub fn to_unsigned_int_vec(mantle_j_object: &MantleJObject, jni_env: JNIEnv, name: &str) -> Option<Vec<u32>> {
    let array = j_object_to_j_int_array(mantle_j_object, jni_env, name, INT_SIG);
    if array.is_null() {
        None
    } else {
        Some(int_array_to_unsigned_int_vec(array, jni_env))
    }
}

pub fn int_array_to_unsigned_int_vec(array: jintArray, env: JNIEnv) -> Vec<u32> {
    let len = env.get_array_length(array).unwrap();
    let mut vec = vec![0 as jint].repeat(len as usize);
    let slice = vec.as_mut_slice();
    env.get_int_array_region(array, 0, slice).unwrap();
    slice.into_iter().map(|j_int| *j_int as u32).collect::<Vec<u32>>()
}