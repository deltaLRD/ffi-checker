use log::info;
use rustc_ast::{self, ast};
use rustc_driver;
use rustc_hir::{self, ForeignItemRef, HirId};
use rustc_middle::{self, ty::TyCtxt};
use std::{collections::HashSet, fs::File, io::Write, path::Path};

use crate::utils::get_arg_flag_value;

#[derive(Debug)]
pub struct Callback {
    pub is_deps: bool,
    pub ffi_map: std::collections::BTreeMap<HirId, ForeignItemRef>,
}

impl rustc_driver::Callbacks for Callback {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

    fn after_crate_root_parsing<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _krate: &mut ast::Crate,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        // queries
        //     .global_ctxt()
        //     .unwrap()
        //     .borrow()
        //     .enter(|tcx| self.run_analysis(compiler, tcx));


        // if self.is_deps {
        //     return rustc_driver::Compilation::Continue;
        // }
        let top_crate_name =
            get_arg_flag_value("--crate-name").expect("arg --crate-name not found");
        let should_skip = vec!["build_script_build"];
        if should_skip.contains(&top_crate_name.as_str()) {
            return rustc_driver::Compilation::Continue;
        }
        info!("{:?}", &top_crate_name);
        let mut pub_funcs = HashSet::new();
        let mut ffi_funcs = HashSet::new();
        // queries.global_ctxt().unwrap().steal().hir_arena
        if let Some((entry_def_id, entry_fn_type)) = tcx.entry_fn(()) {
            // collecting entry function like main function
            info!(
                "id: {:?}\tentry_fn_type: {:?}\n",
                &entry_def_id, &entry_fn_type
            );
            let item_name = tcx.item_name(entry_def_id).to_ident_string();
            info!("{:?}\n", &item_name);
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
                info!(
                    "public_function:\nid: {:#?}\nident: {:#?}\nkind: {:#?}\n",
                    &def_id, &ident, &kind
                );
                pub_funcs.insert(ident.to_string());
            }
            if kind.is_fn_like() && tcx.is_foreign_item(def_id) {
                let ident = tcx.item_ident(def_id.into());
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

        rustc_driver::Compilation::Continue
    }
}

// impl Callback {
//     fn run_analysis<'tcx, 'compiler>(
//         &mut self,
//         _compiler: &rustc_interface::interface::Compiler,
//         tcx: TyCtxt<'tcx>,
//     ) {
//         let top_crate_name =
//             get_arg_flag_value("--crate-name").expect("arg --crate-name not found");
//         let should_skip = vec!["build_script_build"];
//         if should_skip.contains(&top_crate_name.as_str()) {
//             return;
//         }
//         info!("{:?}", &top_crate_name);
//         let mut pub_funcs = HashSet::new();
//         let mut ffi_funcs = HashSet::new();

//         if let Some((entry_def_id, _)) = tcx.
//     }
// }
