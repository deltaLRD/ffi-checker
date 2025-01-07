use std::{fs::File, io::Write};

use rustc_ast;
use rustc_driver;
use rustc_hir::{self, ForeignItemRef, HirId};
use rustc_interface;
use rustc_middle;

#[derive(Debug)]
pub struct Callback {
    pub is_deps: bool,
    pub log_file: File,
    pub ffi_map: std::collections::BTreeMap<HirId, ForeignItemRef>,
}

impl rustc_driver::Callbacks for Callback {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

    fn after_crate_root_parsing(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &rustc_ast::Crate,
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
        if self.is_deps {
            return rustc_driver::Compilation::Continue;
        }
        let hir: rustc_middle::hir::map::Map<'_> = tcx.hir();
        for id in hir.items() {
            let item = hir.item(id);
            
            if let rustc_hir::ItemKind::ForeignMod { abi, items } = item.kind {
                match self.log_file.write(format!("{:?}\n", &item).as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        log::warn!("{}", e);
                    }
                }
                for item in items {
                    self.ffi_map.insert(item.id.hir_id(), item.clone()).unwrap();
                }
                
            }
        }

        rustc_driver::Compilation::Continue
    }
}
