use crate::{CDeclaration, CExpr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CStatement {
    Declaration(CDeclaration),
    Assignment(CExpr, CExpr),
    Return(CExpr),
    If(CExpr, Vec<CStatement>, Vec<CStatement>),
    While(CExpr, Vec<CStatement>),
    DoWhile(CExpr, Vec<CStatement>),
    For(CDeclaration, CExpr, CExpr, Vec<CStatement>),
    Switch(
        CExpr,
        Vec<(CExpr, Vec<CStatement>)>,
        Option<Vec<CStatement>>,
    ),
    Break,
    Continue,
    Goto(String),
    Label(String),
    Block(Vec<CStatement>),
    Expression(CExpr),

    /// temp
    NopStmt,
}
