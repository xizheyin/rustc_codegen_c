use crate::context::ToCContext;
use rust2cast::CStatement;
use rustc_middle::mir::{BasicBlockData, Statement};

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_statement(&self, stmt: &Statement) -> CStatement {
        match &stmt.kind {
            rustc_middle::mir::StatementKind::Assign(assign) => {
                //many cases
                let (lhs, rhs) = &(*assign.clone());
                let clhs = self.codegen_place(&lhs);
                let crhs = self.codegen_rvalue(&rhs);
                return CStatement::Assignment(clhs, crhs);
            }
            rustc_middle::mir::StatementKind::FakeRead(_) => {}
            rustc_middle::mir::StatementKind::SetDiscriminant {
                place,
                variant_index,
            } => {
                //switch
            }
            rustc_middle::mir::StatementKind::Deinit(_) => todo!(),
            rustc_middle::mir::StatementKind::StorageLive(_) => return CStatement::NopStmt,
            rustc_middle::mir::StatementKind::StorageDead(_) => return CStatement::NopStmt,
            rustc_middle::mir::StatementKind::Retag(_, _) => todo!(),
            rustc_middle::mir::StatementKind::PlaceMention(_) => todo!(),
            rustc_middle::mir::StatementKind::AscribeUserType(_, _) => todo!(),
            rustc_middle::mir::StatementKind::Coverage(_) => todo!(),
            rustc_middle::mir::StatementKind::Intrinsic(_) => todo!(),
            rustc_middle::mir::StatementKind::ConstEvalCounter => todo!(),
            rustc_middle::mir::StatementKind::Nop => todo!(),
        }
        return CStatement::NopStmt;
    }
}
