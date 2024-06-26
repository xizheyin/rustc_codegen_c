// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This file contains functions related to codegenning MIR functions into gotoc

use core::panic;

use rust2cast::{CDeclaration, CType};
use rustc_middle::{
    mir::Body,
    ty::{Instance, ParamEnv, TyKind},
};

use crate::context::ToCContext;

type CFnSig = (CType, Vec<CDeclaration>);

/// Codegen MIR functions into gotoc
impl<'tcx> ToCContext<'tcx> {
    /// Declare variables according to their index.
    /// - Index 0 represents the return value.
    /// - Indices [1, N] represent the function parameters where N is the number of parameters.
    /// - Indices that are greater than N represent local variables.
    fn codegen_declare_variables(&mut self, body: &Body) {}

    pub(crate) fn codegen_function(&mut self, instance: Instance<'tcx>) -> Result<(), String> {
        // check wether the instance is indeed a function.
        if let TyKind::FnDef(_, _) = instance.ty(self.tcx, ParamEnv::reveal_all()).kind() {
            //ALL OK.
        } else if let TyKind::Closure(_, _) = instance.ty(self.tcx, ParamEnv::reveal_all()).kind() {
            //println!("CLOSURE")
        } else {
            eprintln!("fn item {instance:?} is not a function definition type. Skippping.");
            return Ok(());
        }

        let cfn_sig = self.extract_signature(&instance);

        let body = self.tcx.instance_mir(instance.def);

        Ok(())
    }

    pub fn declare_function(&mut self, instance: Instance) {}

    fn extract_signature(&self, instance: &Instance<'tcx>) {
        let fn_ty = instance.ty(self.tcx, ParamEnv::reveal_all());
        // the function signature with generic parameters replaced

        let fn_sig = match fn_ty.kind() {
            TyKind::FnDef(_, _) => fn_ty.fn_sig(self.tcx),
            TyKind::Closure(_, args) => args.as_closure().sig(),
            _ => todo!("Can't get signature of {fn_ty}"),
        };
        //println!("signature: {:#?}", fn_sig);
    }
}
