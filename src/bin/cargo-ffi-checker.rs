use core::str;
use std::process::Command;
use ffi_checker::metadata::MetaData;

use log::{debug, info};
use ffi_checker::utils::compile_targets;

// 获取cargo metadata提供的信息
fn get_cargo_metadata() -> Result<MetaData, std::io::Error>{
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("-q")
        .arg("--format-version")
        .arg("1")
        .output()
        .expect("run cmd error:\"cargo metadata -q --format-verson 1\"");
    if output.status.success() {
        // debug!("{:?}", str::from_utf8(&output.stdout).unwrap());
        let metadata: MetaData = serde_json::from_str(str::from_utf8(&output.stdout).unwrap()).unwrap();
        Ok(metadata)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "metadata input invalid"))
    }
}

fn main() {
    pretty_env_logger::init();
    println!("start");
    info!("start ffi checker");
    debug!("debug ffi checker");
    let metadata = get_cargo_metadata().unwrap();
    debug!("{}", serde_json::to_string_pretty(&metadata).unwrap());
    if std::env::args().nth(1).unwrap().as_ref().to_string().contains("rustc")
        // && std::env::args().nth(1).unwrap().as_ref().to_string().contains(".rustup")
    {
        info!("start compiling");

    }
    // if "ffi-checker" == std::env::args().nth(1).unwrap().as_str() {
    let mut ffi_args = Vec::new();
    compile_targets(metadata, &mut ffi_args);
    //}
}
