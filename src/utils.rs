use crate::metadata::{MetaData, Package};
use core::str;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, process::Command};

#[derive(Debug, Deserialize, Serialize)]
struct RootPath {
    root: String,
}

pub fn compile_time_sysroot() -> Option<String> {
    if option_env!("RUSTC_STAGE").is_some() {
        return None;
    }
    let home =
        option_env!("RUSTUP_HOME").unwrap_or(option_env!("MULTIRUST_HOME").unwrap_or("~/.rustup"));
    let toolchain = option_env!("RUSTUP_TOOLCHAIN")
        .unwrap_or(option_env!("MULTIRUST_TOOLCHAIN").unwrap_or("nightly"));
    Some(match (home, toolchain) {
        (home, toolchain) => format!("{}/toolchains/{}", home, toolchain),
    })
}

pub fn arg_is_exist(name: &str) -> bool {
    let mut args = std::env::args().into_iter();
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => {
                return false;
            }
        };
        if !arg.ends_with(name) {
            continue;
        } else {
            return true;
        }
    }
}

pub fn get_now_log_file() -> File {
    let _ = std::fs::create_dir("./logs");
    let now = std::time::SystemTime::now();
    let now_str = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let log_file =
        std::fs::File::create_new(format!("./logs/ffi_checker_{}.log", now_str)).unwrap();
    log_file
}

pub fn file_log(s: &str) {
    let now = std::time::SystemTime::now();
    let now_str = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut log_file =
        std::fs::File::create_new(format!("./logs/ffi_checker_{}.log", now_str)).unwrap();
    log_file.write(format!("{}\n", &s).as_bytes()).unwrap();
}

/// Gets the value of a `name`.
/// For example, get_arg_flag_value("--manifest-path")
/// Supports two styles: `--name value` or `--name=value`
pub fn get_arg_flag_value(name: &str) -> Option<String> {
    // Stop searching at `--`.
    let mut args = std::env::args().take_while(|val| val != "--");
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => return None,
        };
        if !arg.starts_with(name) {
            continue;
        }
        // Strip leading `name`.
        let suffix = &arg[name.len()..];
        if suffix.is_empty() {
            // This argument is exactly `name`; the next one is the value.
            return args.next();
        } else if suffix.starts_with('=') {
            // This argument is `name=value`; get the value.
            // Strip leading `=`.
            return Some(suffix[1..].to_owned());
        }
    }
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
                cmd
                    // .arg("--crate-type")
                    .arg("--bin")
                    // .arg("--crate-name")
                    .arg(target.name.clone());
            }
            "lib" => {
                cmd.arg("--lib");
            }
            _ => continue,
        }
        // -Z unpretty=hir-tree
        // -Z unpretty=thir-tree
        // cmd.arg("--").arg("-Z").arg("unpretty=thir-tree");
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

        let mut path = std::env::current_exe().expect("current executable path invalid");
        path.set_file_name("checker");
        // cmd.env("RUSTC_WRAPPER", path.clone());
        cmd.env("RUSTC", path.clone());
        cmd.env("CC", "clang");

        info!("Setting env: RUSTC={:?}", path);

        // linux only
        // generate llvm ir, llvm bc, mir
        cmd.env(
            "RUSTFLAGS",
            "-Clinker=clang -Clink-arg=-fuse-ld=lld --emit=asm,dep-info,link,llvm-ir,llvm-bc,metadata,mir,obj",
        );
        cmd.env("CC", "clang");
        cmd.env("CFLAGS", "-flto=thin -emit-llvm");
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

#[link(name = "test1")]
extern "C" {
    pub fn greet();
    pub fn get_n_mem(n: libc::c_int) -> *mut libc::c_void;
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ffi() {
        unsafe {
            greet();
            let ptr: *mut libc::c_void = get_n_mem(1024);
            libc::free(ptr);
            let ptr: *mut libc::c_int = ptr as *mut libc::c_int;
            // ptr.write(10);
        }
    }
}
