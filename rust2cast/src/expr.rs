use crate::CType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CExpr {
    Literal(String),
    Variable(String),
    BinaryOp(String, Box<CExpr>, Box<CExpr>),
    UnaryOp(String, Box<CExpr>),
    FunctionCall(String, Vec<CExpr>),
    StructAccess(Box<CExpr>, String),
    ArrayAccess(Box<CExpr>, Box<CExpr>),
    Cast(Box<CType>, Box<CExpr>),
    Sizeof(Box<CType>),
    Nop,
}
