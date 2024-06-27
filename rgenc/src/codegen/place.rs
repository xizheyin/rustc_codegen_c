use crate::context::ToCContext;
use rust2cast::{CExpr, CStatement};
use rustc_middle::mir::{BasicBlockData, Place, Statement};

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_place(&self, p: &Place) -> CExpr {
        if p.projection.len() == 0 {
            return CExpr::Variable(p.local.as_usize().to_string());
        }

        CExpr::Variable(p.local.as_usize().to_string())
    }
}
