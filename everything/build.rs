use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").expect("PROFILE environment variable not found");
    let dll_path = Path::new("../everything_sys/Everything-SDK/dll/Everything64.dll");

    let target_path = Path::new(&target_dir)
        .join(profile)
        .join("deps")
        .join("Everything64.dll");

    if !target_path.exists() {
        fs::copy(dll_path, target_path).expect("Failed to copy Everything.dll");
    }
}
