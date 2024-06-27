use crate::context::ToCContext;
use rust2cast::{CExpr, CStatement};
use rustc_middle::mir::{BasicBlockData, Operand, Place, Statement};

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_operand(&self, operand: &Operand) -> CExpr {
        match operand {
            Operand::Copy(p) | Operand::Move(p) => self.codegen_place(p),
            Operand::Constant(_) => CExpr::Nop,
        }
    }
}
