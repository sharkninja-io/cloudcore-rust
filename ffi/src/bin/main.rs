use std::process::Command;

fn main() {
    // Build iOS
    let for_release = !cfg!(debug_assertions);
    let release_flag = "--release";
    let targets = vec![
        "aarch64-apple-ios",
        "x86_64-apple-ios",
    ];
    let variant = if for_release { "release" } else { "debug" };
    println!("Building {} iOS", variant);
    targets.into_iter().for_each(|target| {
        let mut args = vec!["build", "--target", target];
        if for_release {
            args.push(release_flag);
        }
        let status = Command::new("cargo").args(&args).status().unwrap();
        if !status.success() {
            println!("Failed to create {} cloudcore static library: {}", target, status);
        } else {
            println!("Created {} cloudcore static library", target);
        }
    });
    println!(
        "{} iOS libraries built",
        variant.to_uppercase()
    )
}