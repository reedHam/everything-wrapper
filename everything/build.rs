use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let target_feature = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let is_release = target_feature.contains("crt-static");

    let profile = if is_release { "release" } else { "debug" };

    let dll_path = Path::new(r"C:\github\everything-wrapper\Everything-SDK\dll\Everything64.dll");

    let target_path = Path::new(&target_dir)
        .join(profile)
        .join("deps")
        .join("Everything64.dll");

    if !target_path.exists() {
        fs::copy(&dll_path, &target_path).expect(&format!(
            "Failed to copy Everything.dll from {:?} to {:?}",
            dll_path, target_path
        ));
    }
}
