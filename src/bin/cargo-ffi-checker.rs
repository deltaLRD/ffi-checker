use core::str;
use std::process::Command;
use ffi_checker::metadata::MetaData;

use log::{debug, info};
use ffi_checker::utils::compile_targets;

// 获取cargo metadata提供的信息
fn get_cargo_metadata() -> Result<MetaData, std::io::Error>{
    debug!("get cargo metadata");
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("-q")
        .arg("--format-version")
        .arg("1")
        .output()
        .expect("run cmd error:\"cargo metadata -q --format-verson 1\"");
    if output.status.success() {
        let metadata: MetaData = serde_json::from_str(str::from_utf8(&output.stdout).unwrap()).unwrap();
        Ok(metadata)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "metadata input invalid"))
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    debug!("args: {:?}", std::env::args());
    info!("start ffi checker");
    debug!("debug ffi checker");
    info!("start ffi checker");

    let metadata = get_cargo_metadata().unwrap();
    let mut ffi_args = Vec::new();
    compile_targets(metadata, &mut ffi_args);
    debug!("{:?}", &ffi_args);

}
