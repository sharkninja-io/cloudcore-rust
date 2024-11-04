use std::collections::HashMap;

use crate::{CloudCore, urls};
use crate::cloudcore::{ApplicationInfo, AylaRegionEnvironment, CN_PROD_DEVICE_URL, CN_PROD_USER_URL, EU_PROD_USER_URL, NA_PROD_DEVICE_URL, NA_PROD_USER_URL, SessionParameters};

pub unsafe fn get_cloudcore() -> &'static mut CloudCore {
    let _cc = CloudCore::new(
        urls::CRATE_WORKSPACE.to_string()
    );
    let cc = CloudCore::shared();
    create_ayla_region_environment_map(cc);
    return cc;
}

fn create_ayla_region_environment_map(cloudcore: &mut CloudCore) {
    let mut map: HashMap<AylaRegionEnvironment, SessionParameters> = HashMap::new();

    let na_prod = SessionParameters {
        app_info: ApplicationInfo {
            app_id: "Shark-iOS-field-id".to_string(),
            app_secret: "Shark-iOS-field-_wW7SiwgrHN8dpU_ugCattOoDk8".to_string(),
        },
        user_url: NA_PROD_USER_URL.to_string(),
        device_url: NA_PROD_DEVICE_URL.to_string(),
    };
    map.insert(AylaRegionEnvironment::NAProd, na_prod);
    let cn_prod = SessionParameters {
        app_info: ApplicationInfo {
            app_id: "Shark-iOS-CNField-vQ-id".to_string(),
            app_secret: "Shark-iOS-CNField-CwiOekSWdjbn3fqeLhM3cfKIeQc".to_string(),
        },
        user_url: CN_PROD_USER_URL.to_string(),
        device_url: CN_PROD_DEVICE_URL.to_string(),
    };
    map.insert(AylaRegionEnvironment::CNProd, cn_prod);
    let eu_prod = SessionParameters {
        app_info: ApplicationInfo {
            app_id: "Shark-iOS-EUField-gQ-id".to_string(),
            app_secret: "Shark-iOS-EUField-mXS6lxzOi40Ebwn2-s6T6ZzfVXs".to_string()
        },
        user_url: EU_PROD_USER_URL.to_string(),
        device_url: EU_PROD_USER_URL.to_string(),
    };
    map.insert(AylaRegionEnvironment::EUProd, eu_prod);
    cloudcore.ayla_region_environment_map = map;
}