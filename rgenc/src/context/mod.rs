use rust2cast::CProgram;
use rustc_middle::{mir::mono::MonoItem, ty::TyCtxt};

pub(crate) struct ToCContext<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub ast: CProgram,
}

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            ast: CProgram::default(),
        }
    }

    pub(crate) fn codegen_item(&mut self, item: &MonoItem) {}
}
