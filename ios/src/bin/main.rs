use std::collections::HashMap;
use std::process::Command;

fn main() {
    // Build iOS
    let for_release = !cfg!(debug_assertions);
    let release_flag = "--release";
    let targets = [
        "aarch64-apple-ios",
        "x86_64-apple-ios",
    ];
    let variant = if for_release { "release" } else { "debug" };
    let cargo_paths = ["../ffi", "."];
    let libs_suffixes = ["ffi", "ios"];
    let fs_map: HashMap<_, _> = cargo_paths.iter().zip(libs_suffixes.iter()).collect();
    for (dir, suffix) in fs_map {
        for target in targets {
            println!("Building {} {} static library", target, suffix);
            let manifest_path = &format!("{}/Cargo.toml", dir);
            let mut args = vec!["build", "--target", target, "--manifest-path", manifest_path];
            if for_release {
                args.push(release_flag);
            }
            let status = Command::new("cargo").args(&args).status().unwrap();
            if !status.success() {
                println!("Failed to create {} {} static library: {}", target, suffix, status);
                panic!()
            } else {
                println!("Created {} {} static library", target, suffix);
            }
        }
        println!("iOS static libraries made for cloudcore_{}.a", suffix);
    }
    println!(
        "{} iOS libraries built",
        variant.to_uppercase()
    )
}