mod value;

use android_utilities::jni_exts::jlong::MantleJlong;
use android_utilities::jni_exts::jobject::MantleJObject;
use android_utilities::jni_exts::jstring::MantleJString;
use android_utilities::{JObjectRustBridge, CallbackStruct, RuntimeAndroid, to_java_result};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jlong;
use lazy_static::lazy_static;
use std::sync::Mutex;
use cloudcore::CloudCore;
use crate::cache::value::JavaCacheDataValue;
use crate::cloudcore_ffi_api::CLOUDCORE_API;

lazy_static! {
    static ref SET_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
    static ref GET_CB_STRUCT: Mutex<CallbackStruct> = Mutex::new(CallbackStruct::new());
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_CacheKt_setValue(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_path: JString,
    j_key: JString,
    j_value: JObject,
    j_callback: JObject,
) {
    let path = MantleJString(j_path).to_char_ptr(env);
    let key = MantleJString(j_key).to_char_ptr(env);
    let cache_value = JavaCacheDataValue::rust_object(MantleJObject(j_value), env).unwrap();
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    SET_CB_STRUCT.lock().unwrap().update(env, j_callback);
    let boxed_value = Box::into_raw(Box::new(cache_value));
    CLOUDCORE_API.cloudcore_set_value(cloudcore, path, key, boxed_value, |result| {
        let cb_struct = SET_CB_STRUCT.lock().unwrap();
        RuntimeAndroid::exec_sync(&cb_struct.jvm, result, &cb_struct.callback);
    });
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_sharkninja_cloudcore_CacheKt_getValue(
    env: JNIEnv,
    _class: JClass,
    ptr_cloudcore: jlong,
    j_path: JString,
    j_key: JString,
    j_callback: JObject,
) {
    let path = MantleJString(j_path).to_char_ptr(env);
    let key = MantleJString(j_key).to_char_ptr(env);
    let cloudcore = MantleJlong(ptr_cloudcore).to_pointer::<CloudCore>();
    GET_CB_STRUCT.lock().unwrap().update(env, j_callback);
    CLOUDCORE_API.cloudcore_get_value(cloudcore, path, key, |result| {
        let result = to_java_result::<_, JavaCacheDataValue>(result);
        let cb_struct = GET_CB_STRUCT.lock().unwrap();
        RuntimeAndroid::exec_sync(&cb_struct.jvm, result, &cb_struct.callback);
    });
}