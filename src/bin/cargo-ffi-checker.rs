use core::str;
use std::process::Command;

use log::{debug, info};

fn get_cargo_metadata() {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("-q")
        .output()
        .expect("run cmd error:\"cargo metadata -q\"");
    if output.status.success() {
        let json: = serde_json::from_str(str::from_utf8(&output.stdout).unwrap())
    }
}

fn main() {
    pretty_env_logger::init();
    info!("start ffi checker");
    debug!("debug ffi checker");
}
