use crate::context::ToCContext;
use rust2cast::{CExpr, CStatement};
use rustc_middle::mir::{BasicBlockData, Terminator};

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_basicblock(&self, bb_idx: usize, bb: &BasicBlockData) -> Vec<CStatement> {
        let mut cstmts = vec![];
        cstmts.push(CStatement::Label("Label".to_string() + &bb_idx.to_string()));

        for stmt in &bb.statements {
            cstmts.push(self.codegen_statement(stmt));
        }

        cstmts.push(self.codegen_terminator(bb.terminator()));

        cstmts
    }

    fn codegen_terminator(&self, termi: &Terminator) -> CStatement {
        match &termi.kind {
            rustc_middle::mir::TerminatorKind::Return => {
                return CStatement::Return(CExpr::Variable("0".to_string()))
            }
            _ => {
                return CStatement::NopStmt;
            }
        }
    }
}
