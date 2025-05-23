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

// use simple_file_logger::init_logger;
use log::{info, debug};
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // init_logger!("checker.log").unwrap();
    pretty_env_logger::init_timed();
    debug!("{:?}", std::env::args());
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    let args = rustc_driver::args::arg_expand_all(&early_dcx, std::env::args().map(|item|item).collect::<Vec<_>>().as_slice());

    
    let mut callback = ffi_checker::callback::Callback {
        is_deps: false,
    };

    info!("args: {:?}\n", args);
    let runner = rustc_driver::RunCompiler::new(&args, &mut callback);
    let _ = runner.run();
}
