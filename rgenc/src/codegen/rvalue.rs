use crate::context::ToCContext;
use rust2cast::{CExpr, CStatement};
use rustc_middle::mir::{BasicBlockData, Place, Rvalue, Statement};

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_rvalue(&self, rvalue: &Rvalue) -> CExpr {
        match rvalue {
            Rvalue::Use(o) => self.codegen_operand(o),
            Rvalue::Repeat(_, _) => todo!(),
            Rvalue::Ref(_, _, _) => CExpr::Nop,
            Rvalue::ThreadLocalRef(_) => todo!(),
            Rvalue::AddressOf(_, _) => todo!(),
            Rvalue::Len(_) => todo!(),
            Rvalue::Cast(_, _, _) => CExpr::Nop,
            Rvalue::BinaryOp(_, _) => todo!(),
            Rvalue::NullaryOp(_, _) => todo!(),
            Rvalue::UnaryOp(_, _) => todo!(),
            Rvalue::Discriminant(_) => todo!(),
            Rvalue::Aggregate(_, _) => CExpr::Nop,
            Rvalue::ShallowInitBox(_, _) => todo!(),
            Rvalue::CopyForDeref(_) => todo!(),
        }
    }
}
