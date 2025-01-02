use rustc_ast;
use rustc_driver;
use rustc_hir;
use rustc_interface;
use rustc_middle;

use log::debug;



pub struct Callback {
    pub is_deps: bool,
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
        _compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        if self.is_deps {
            return rustc_driver::Compilation::Continue;
        }
        let hir: rustc_middle::hir::map::Map<'_> = tcx.hir();
        for id in hir.items() {
            let item = hir.item(id);
            if let rustc_hir::ItemKind::ForeignMod { abi, items } = item.kind {
                debug!("abi: {:?}, items: {:?}", &abi, &items);
            }
        }

        rustc_driver::Compilation::Continue
    }
}