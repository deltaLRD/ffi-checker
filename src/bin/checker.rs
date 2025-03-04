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

use std::collections::BTreeMap;

use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;

fn main() {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    // let args = rustc_driver::args::raw_args(&early_dcx)
    //     .unwrap_or_else(|_| std::process::exit(rustc_driver::EXIT_FAILURE));
    let args = rustc_driver::args::raw_args(&early_dcx);
    rustc_driver::install_ctrlc_handler();
    
    let mut callback = ffi_checker::callback::Callback {
        is_deps: false,
        log_file: ffi_checker::utils::get_now_log_file(),
        ffi_map: BTreeMap::new(),
    };

    callback.log(&format!("args: {:?}\n", args));
    rustc_driver::run_compiler(&args, &mut callback);
}
