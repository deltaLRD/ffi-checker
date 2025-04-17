#![feature(rustc_private)]
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
use core::str;
use ffi_checker::{metadata::MetaData, utils::generate_llvm_bitcode};
use std::process::Command;

use ffi_checker::utils::compile_targets;
use log::{debug, info};

// 获取cargo metadata提供的信息
fn get_cargo_metadata() -> Result<MetaData, std::io::Error> {
    debug!("get cargo metadata");
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("-q")
        .arg("--format-version")
        .arg("1")
        .output()
        .expect("run cmd error:\"cargo metadata -q --format-verson 1\"");
    if output.status.success() {
        let metadata: MetaData =
            serde_json::from_str(str::from_utf8(&output.stdout).unwrap()).unwrap();
        Ok(metadata)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "metadata input invalid",
        ))
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init_timed();
    // debug!("args: {:?}", std::env::args());
    info!("start ffi checker");

    let metadata = get_cargo_metadata().unwrap();
    let mut ffi_args = Vec::new();
    let mut target_name = Vec::new();
    compile_targets(metadata, &mut ffi_args, &mut target_name);
    debug!("{:?}", &ffi_args);

    generate_llvm_bitcode(&target_name);

}
