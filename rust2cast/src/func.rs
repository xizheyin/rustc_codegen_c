use crate::{decl::CDeclaration, stmt::CStatement, ty::CType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CFunction {
    pub return_type: CType,
    pub name: String,
    pub parameters: Vec<CDeclaration>,
    pub local_variables: Vec<CDeclaration>,
    pub body: Vec<CStatement>,
}
