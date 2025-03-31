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

pub mod callback;
pub mod metadata;
pub mod utils;
// pub mod analysis;
use std::ffi::c_int;
pub unsafe fn unsafe_func() {
    unsafe {
        utils::greet();
        let ptr = utils::get_n_mem(128) as *mut c_int;
        ptr.write(20);
        ptr.offset(1).write(30);
        libc::free(ptr as *mut libc::c_void);
        let ptr_2 = utils::get_n_mem(128) as *mut c_int;
        ptr.write(40);
        println!("{:?}:{}", &ptr, &ptr.read());
        println!("{:?}:{}", &ptr_2, &ptr_2.read());
    }
}