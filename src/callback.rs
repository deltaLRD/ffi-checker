use log::{info, debug};
// use rustc_ast::{self, ast};
use rustc_driver;
// use rustc_hir::{self, ForeignItemRef, HirId};
use rustc_interface::Queries;
use rustc_middle::{self, ty::TyCtxt};
use std::{collections::HashSet, fs::File, io::Write, path::Path};

use crate::utils::get_arg_flag_value;

#[derive(Debug)]
pub struct Callback {
    pub is_deps: bool,
}

impl rustc_driver::Callbacks for Callback {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

    fn after_crate_root_parsing<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries
            .global_ctxt()
            .unwrap()
            .borrow()
            .enter(|tcx| self.run_analysis(compiler, tcx));

        rustc_driver::Compilation::Continue
    }
}

impl Callback {
    pub fn new() -> Self {
        if std::env::var_os("FFI_CHECKER_IS_DEPS").is_some() {
            Self {
                is_deps: true
            }
        } else {
            Self {
                is_deps: false,
            }
        }
    }
    fn run_analysis<'tcx, 'compiler>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) {
        let top_crate_name =
            get_arg_flag_value("--crate-name").expect("arg --crate-name not found");
        let should_skip = vec!["build_script_build"];
        if should_skip.contains(&top_crate_name.as_str()) {
            debug!("skip: {:?}", &top_crate_name.as_str());
            return;
        }
        info!("{:?}", &top_crate_name);
        let mut pub_funcs = HashSet::new();
        let mut ffi_funcs = HashSet::new();

        if let Some((entry_def_id, _)) = tcx.entry_fn(()) {
            let item_name = tcx.item_name(entry_def_id).to_ident_string();
            pub_funcs.insert(item_name);
        }

        for def_id in tcx.iter_local_def_id() {
            let kind: rustc_hir::def::DefKind = tcx.def_kind(def_id);
            // if kind.is_fn_like() && tcx.local_visibility(def_id).is_public() {
            //     let ident = tcx.opt_item_ident(def_id.to_def_id()).unwrap();
            //     info!(
            //         "public_function:\nid: {:#?}\nident: {:#?}\nkind: {:#?}\n",
            //         &def_id, &ident, &kind
            //     );
            //     pub_funcs.insert(ident.to_string());
            // }
            if kind.is_fn_like() && tcx.is_foreign_item(def_id) {
                let ident = tcx.opt_item_ident(def_id.to_def_id()).unwrap();
                info!(
                    "ffi_function:\nid: {:#?}\nident: {:#?}\nkind: {:#?}\n",
                    &def_id, &ident, &kind
                );
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
        
    }
}
