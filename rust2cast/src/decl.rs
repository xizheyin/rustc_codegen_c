use crate::{expr::CExpr, ty::CType};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CDeclaration {
    pub name: String,
    pub ctype: CType,
    pub initializer: Option<CExpr>,
}
