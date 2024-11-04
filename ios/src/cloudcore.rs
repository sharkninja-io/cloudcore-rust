use std::collections::HashMap;
use std::os::raw::c_char;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use cloudcore::CloudCore;
use cloudcore::cloudcore::{ApplicationInfo, AylaRegionEnvironment};

#[allow(improper_ctypes)]
extern "C" {
    fn cloudcore_create(
        os_dir: *const c_char,
        hash_map: *mut HashMap<AylaRegionEnvironment, ApplicationInfo>
    ) -> *const CloudCore;
    fn cloudcore_destroy(
        ptr_cloudcore: *mut CloudCore,
    );
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_create(
    os_dir: *const c_char
) -> *const CloudCore {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    let region_map = create_ayla_region_environment_map();
    let region_map = Box::into_raw(Box::new(region_map));
    cloudcore_create(os_dir, region_map)
}

#[no_mangle]
pub unsafe extern "C" fn ios_cloudcore_destroy(
    ptr_cloudcore: *mut CloudCore,
) {
    cloudcore_destroy(ptr_cloudcore)
}

fn create_ayla_region_environment_map() -> HashMap<AylaRegionEnvironment, ApplicationInfo> {
    let mut map: HashMap<AylaRegionEnvironment, ApplicationInfo> = HashMap::new();

    map.insert(AylaRegionEnvironment::NAProd, ApplicationInfo {
        app_id: "Shark-iOS-field-id".to_string(),
        app_secret: "Shark-iOS-field-_wW7SiwgrHN8dpU_ugCattOoDk8".to_string()
    });

    map.insert(AylaRegionEnvironment::NADev, ApplicationInfo {
        app_id: "Vac-iOS-dev-id".to_string(),
        app_secret: "Vac-iOS-dev-TFn4WRa14qNGae7eYh5M0HPSap0".to_string()
    });

    map.insert(AylaRegionEnvironment::EUProd, ApplicationInfo {
        app_id: "Shark-iOS-EUField-gQ-id".to_string(),
        app_secret: "Shark-iOS-EUField-mXS6lxzOi40Ebwn2-s6T6ZzfVXs".to_string()
    });

    map.insert(AylaRegionEnvironment::CNProd, ApplicationInfo {
        app_id: "Shark-iOS-CNField-vQ-id".to_string(),
        app_secret: "Shark-iOS-CNField-CwiOekSWdjbn3fqeLhM3cfKIeQc".to_string()
    });

    map.insert(AylaRegionEnvironment::CNDev, ApplicationInfo {
        app_id: "Shark-iOS-CNDev-uw-id".to_string(),
        app_secret: "Shark-iOS-CNDev-bCzWLZTULUPVJD-4vt97dd4ufjU".to_string()
    });

    map
}
