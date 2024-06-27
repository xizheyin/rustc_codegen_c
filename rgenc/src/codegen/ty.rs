use crate::context::ToCContext;
use rust2cast::CType;
use rustc_middle::ty::{Ty, TyKind};
use rustc_type_ir::IntTy;

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn codegen_ty(&self, typ: &Ty) -> CType {
        match typ.kind() {
            TyKind::Bool => CType::Bool,
            TyKind::Char => CType::Char,
            TyKind::Int(int_ty) => match int_ty {
                IntTy::Isize => CType::Isize,
                IntTy::I8 => todo!(),
                IntTy::I16 => todo!(),
                IntTy::I32 => CType::Int,
                IntTy::I64 => todo!(),
                IntTy::I128 => todo!(),
            },
            TyKind::Uint(_) => CType::Nop,
            TyKind::Float(_) => todo!(),
            TyKind::Adt(_, _) => CType::Nop,
            TyKind::Foreign(_) => todo!(),
            TyKind::Str => todo!(),
            TyKind::Array(_, _) => todo!(),
            TyKind::Pat(_, _) => todo!(),
            TyKind::Slice(_) => todo!(),
            TyKind::RawPtr(_, _) => CType::Nop,
            TyKind::Ref(_, _, _) => todo!(),
            TyKind::FnDef(_, _) => todo!(),
            TyKind::FnPtr(_) => CType::Nop,
            TyKind::Dynamic(_, _, _) => todo!(),
            TyKind::Closure(_, _) => CType::Nop,
            TyKind::CoroutineClosure(_, _) => todo!(),
            TyKind::Coroutine(_, _) => todo!(),
            TyKind::CoroutineWitness(_, _) => todo!(),
            TyKind::Never => todo!(),
            TyKind::Tuple(_) => CType::Nop,
            TyKind::Alias(_, _) => CType::Nop,
            TyKind::Param(_) => todo!(),
            TyKind::Bound(_, _) => todo!(),
            TyKind::Placeholder(_) => todo!(),
            TyKind::Infer(_) => todo!(),
            TyKind::Error(_) => todo!(),
        }
    }
}
