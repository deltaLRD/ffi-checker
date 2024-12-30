use crate::metadata::{MetaData, Package};
use core::str;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct RootPath {
    root: String,
}

pub fn get_proj_path() -> String {
    let res = Command::new("cargo")
        .arg("locate-project")
        .output()
        .unwrap();
    let binding = res.stdout.to_vec();
    let res = str::from_utf8(&binding).unwrap();
    let res: RootPath = serde_json::from_str(res).unwrap();
    res.root.to_owned()
}

pub fn get_current_crate(metadata: MetaData) -> Option<Package> {
    let proj_path = get_proj_path();
    info!("prj_path: {}", &proj_path);
    for package in metadata.packages.unwrap().iter() {
        if package.manifest_path.eq(&proj_path) {
            return Some(package.clone());
        }
    }
    None
}

pub fn compile_targets(metadata: MetaData, ffi_args: &mut Vec<String>) {
    let current_crate = get_current_crate(metadata.clone()).unwrap();
    for target in current_crate.targets.unwrap().iter() {
        let mut cmd = Command::new("cargo");
        cmd.arg("rustc");
        let mut args = std::env::args().skip(2);
        let kind = target.clone().kind.unwrap();
        match kind[0].as_str() {
            "bin" => {
                cmd.arg("--bin").arg(target.name.clone());
            }
            "lib" => {
                cmd.arg("--lib");
            }
            _ => continue,
        }
        while let Some(arg) = args.next() {
            if arg == "--" {
                break;
            }
            cmd.arg(arg);
        }

        ffi_args.extend(args);

        cmd.env("FFI_CHECKER_TOP_CRATE_NAME", target.name.clone());
        info!(
            "Setting env: FFI_CHECKER_TOP_CRATE_NAME={}",
            target.name.clone()
        );
        // linux only
        cmd.env(
            "RUSTFLAGS",
            "-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld --emit=llvm-ir,llvm-bc,mir",
        );
        cmd.env("CC", "clang");
        cmd.env("CFLAGS", "-flto=thin");
        cmd.env("LDFLAGS", "-Wl,-O2,--as-needed");

        info!("Command line: {:?}", cmd);

        let res = cmd.output().unwrap();
        if !res.status.success() {
            warn!("Command line failed with status: {}", res.status);
            println!(
                "Command line stdout: {}",
                str::from_utf8(&res.stdout).unwrap()
            );
            println!(
                "Command line stderr: {}",
                str::from_utf8(&res.stderr).unwrap()
            );
            std::process::exit(res.status.code().unwrap_or(-1));
        }
    }
}

#[cfg(test)]
mod tests {

    #[link(name = "test1")]
    extern "C" {
        fn greet();
        fn get_n_mem(n: libc::c_int) -> *mut libc::c_void;
    }

    #[test]
    fn test_ffi() {
        unsafe {
            greet();
            let ptr: *mut libc::c_void = get_n_mem(1024);
        }
    }
}