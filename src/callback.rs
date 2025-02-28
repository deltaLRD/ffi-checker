use std::{collections::HashSet, fs::File, io::Write, path::Path, str};

use rustc_ast;
use rustc_driver;
use rustc_hir::{self, ForeignItemRef, HirId};
use rustc_interface;
use rustc_middle;

use crate::utils::get_arg_flag_value;

#[derive(Debug)]
pub struct Callback {
    pub is_deps: bool,
    pub log_file: File,
    pub ffi_map: std::collections::BTreeMap<HirId, ForeignItemRef>,
}

impl Callback {
    pub fn log(&mut self, msg: &str) {
        self.log_file.write(msg.as_bytes()).unwrap();
        self.log_file.flush().unwrap();
    }
}

impl rustc_driver::Callbacks for Callback {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

    fn after_crate_root_parsing(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &mut rustc_ast::Crate,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        // if self.is_deps {
        //     return rustc_driver::Compilation::Continue;
        // }
        let top_crate_name =
            get_arg_flag_value("--crate-name").expect("arg --crate-name not found");
        let should_skip = vec!["build_script_build"];
        if should_skip.contains(&top_crate_name.as_str()) {
            return rustc_driver::Compilation::Continue;
        }
        self.log(&top_crate_name);
        self.log("\n");
        let mut pub_funcs = HashSet::new();
        let mut ffi_funcs = HashSet::new();

        if let Some((entry_def_id, entry_fn_type)) = tcx.entry_fn(()) {
            // collecting entry function like main function
            self.log(&format!(
                "id: {:?}\tentry_fn_type: {:?}\n",
                &entry_def_id, &entry_fn_type
            ));
            let item_name = tcx.item_name(entry_def_id).to_ident_string();
            self.log(&format!("{:?}\n", &item_name));
            pub_funcs.insert(item_name);
        }

        // let mut top_crate_num = 0;
        // for crate_num in tcx.crates(()) {
        //     if top_crate_name.eq(&tcx.crate_name(*crate_num).to_string()) {
        //         top_crate_num = crate_num.as_u32();
        //     }
        // }

        for def_id in tcx.iter_local_def_id() {
            let kind = tcx.def_kind(def_id);
            if kind.is_fn_like() && tcx.local_visibility(def_id).is_public() {
                let ident = tcx.item_ident(def_id.into());
                self.log(&format!(
                    "public_function:\nid: {:#?}\nident: {:#?}\nkind: {:#?}\n",
                    &def_id, &ident, &kind
                ));
                pub_funcs.insert(ident.to_string());
            }
            if kind.is_fn_like() && tcx.is_foreign_item(def_id) {
                let ident = tcx.item_ident(def_id.into());
                self.log(&format!(
                    "ffi_function:\nid: {:#?}\nident: {:#?}\nkind: {:#?}\n",
                    &def_id, &ident, &kind
                ));
                ffi_funcs.insert(ident);
            }
        }

        if !Path::new("target/entry_points").exists() {
            std::fs::create_dir_all("target/entry_points")
                .expect("Failed to create `entry_points` dict");
            let file_path = Path::new("target/entry_points").join(top_crate_name);
            if !file_path.exists() {
                let mut file = File::create(file_path).expect("Failed to create file");
                for entry in pub_funcs {
                    file.write_all(format!("Entry: {}\n", entry).as_bytes())
                        .unwrap();
                }
                for ffi in ffi_funcs {
                    file.write_all(format!("FFI: {}\n", ffi).as_bytes())
                        .unwrap();
                }
            }
        }

        rustc_driver::Compilation::Continue
    }
}
