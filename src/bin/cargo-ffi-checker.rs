use core::str;
use std::process::Command;

use log::{debug, info};

fn get_cargo_metadata() {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("-q")
        .arg("--format-version")
        .arg("1")
        .output()
        .expect("run cmd error:\"cargo metadata -q --format-verson 1\"");
    if output.status.success() {
        debug!("{:?}", str::from_utf8(&output.stdout).unwrap());

    }
}

fn main() {
    pretty_env_logger::init();
    println!("start");
    info!("start ffi checker");
    debug!("debug ffi checker");
    get_cargo_metadata();
}
