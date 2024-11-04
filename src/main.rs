use std::collections::HashMap;
//use std::env::var;
use std::process::Command;

fn main() {
    // Build iOS
    let for_release = !cfg!(debug_assertions);
    let release_flag = "--release";
    let mut args = vec!["lipo"];
    if for_release {
        args.push(release_flag);
    }
    let variant = if for_release { "release" } else { "debug" };
    println!("Building {} iOS", variant);
    let status = Command::new("cargo").args(&args).status().unwrap();
    if !status.success() {
        println!("Failed to create iOS universal library: {}", status);
        panic!()
    }
    // Build Android
    println!("Building {} Android", variant);
    let targets = vec![
        "i686-linux-android",
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ];
    let abis = vec!["x86", "armeabi-v7a", "arm64-v8a"];
    let map: HashMap<_, _> = targets.into_iter().zip(abis.into_iter()).collect();
    for (target, abi) in &map {
        println!("Building {} {}", variant, target);
        let mut args = vec!["build", "--target", target, "--package", "android"];
        if for_release {
            args.push(release_flag);
        }
        let status = Command::new("cargo")
            .args(&args)
            .status()
            .unwrap();
        if status.success() {
            println!("Copying {} target: {} to ABI: {}", variant, target, abi);
            Command::new("cp")
                .arg(format!("target/{}/{}/libcloudcore.so", target, variant))
                .arg(format!("../android/cloudcore/src/main/jniLibs/{}/", abi))
                .status()
                .unwrap();
        } else {
            println!(
                "Failed to create {} android library for {}: {}",
                variant, target, status
            );
            panic!()
        }
    }
    println!(
        "{} libraries built and copied to platform projects",
        variant.to_uppercase()
    )
}
