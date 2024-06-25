use crate::decl::CDeclaration;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CType {
    Int,
    Float,
    Double,
    Char,
    Void,
    Struct(String),
    Union(String),
    Enum(String),
    Pointer(Box<CType>),
    Array(Box<CType>, usize),
    Function(Box<CType>, Vec<CType>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CStruct {
    pub name: String,
    pub fields: Vec<CDeclaration>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CUnion {
    pub name: String,
    pub fields: Vec<CDeclaration>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CEnum {
    pub name: String,
    pub variants: Vec<(String, Option<i32>)>,
}
