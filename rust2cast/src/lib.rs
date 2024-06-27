mod decl;
mod expr;
mod fmt;
mod func;
mod stmt;
mod ty;

pub use decl::CDeclaration;
pub use expr::CExpr;
pub use func::CFunction;
pub use stmt::CStatement;
pub use ty::{CEnum, CStruct, CType, CUnion};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CProgram {
    pub structs: Vec<CStruct>,
    pub unions: Vec<CUnion>,
    pub enums: Vec<CEnum>,
    pub functions: Vec<CFunction>,
    pub global_variables: Vec<CDeclaration>,
}
