use std::collections::HashMap;
use std::env;
use std::process::Command;

fn main() {
    // Build Android
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Need a jniLibs directory to write the shared libraries to!");
    }
    let jni_dir = &args[1];
    let for_release = !cfg!(debug_assertions);
    let release_flag = "--release";
    let variant = if for_release { "release" } else { "debug" };
    println!("Building {} Android", variant);
    let targets = [
        "i686-linux-android",
        "x86_64-linux-android",
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ];
    let abis = ["x86", "x86_64", "armeabi-v7a", "arm64-v8a"];
    let map: HashMap<_, _> = targets.iter().zip(abis.iter()).collect();
    let cargo_paths = ["../ffi", "."];
    let libs_suffixes = ["ffi", "android"];
    let fs_map: HashMap<_, _> = cargo_paths.iter().zip(libs_suffixes.iter()).collect();
    for (dir, suffix) in fs_map {
        for (target, abi) in &map {
            println!("Building {} {} {} shared library", variant, target, suffix);
            let manifest_path = &format!("{}/Cargo.toml", dir);
            let mut args = vec!["build", "--target", target, "--manifest-path", manifest_path];
            if for_release {
                args.push(release_flag);
            }
            let status = Command::new("cargo")
                .args(&args)
                .status()
                .unwrap();
            if status.success() {
                println!("Copying {} target: {} {} shared library to ABI: {}", variant, target, suffix, abi);
                let cp_status = Command::new("cp")
                    .arg(format!("../target/{}/{}/libcloudcore_{}.so", target, variant, suffix))
                    .arg(format!("{}/{}/", jni_dir, abi))
                    .status()
                    .unwrap();
                if !cp_status.success() {
                    panic!(
                        "Failed to copy {} {} library for {}: {}",
                        variant, suffix, target, status
                    );
                }
            } else {
                panic!(
                    "Failed to create {} {} library for {}: {}",
                    variant, suffix, target, status
                );
            }
        }
        println!("Android shared libraries made for cloudcore_{}.so", suffix);
    }
    println!(
        "{} Android libraries built and copied to platform projects",
        variant.to_uppercase()
    )
}
