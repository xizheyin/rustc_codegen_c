// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This file contains functions related to codegenning MIR functions into gotoc

use stable_mir::mir::mono::Instance;
use stable_mir::mir::{Body, Local};
use stable_mir::ty::{RigidTy, TyKind};
use stable_mir::CrateDef;
use std::collections::BTreeMap;

use crate::context::ToCContext;

/// Codegen MIR functions into gotoc
impl<'tcx> ToCContext<'tcx> {
    /// Declare variables according to their index.
    /// - Index 0 represents the return value.
    /// - Indices [1, N] represent the function parameters where N is the number of parameters.
    /// - Indices that are greater than N represent local variables.
    fn codegen_declare_variables(&mut self, body: &Body) {}

    pub(crate) fn codegen_function(&mut self, instance: Instance) {
        let name = instance.mangled_name();

        let body = instance.body().unwrap();
        self.codegen_function_prelude(&body);
    }

    /// Codegen changes required due to the function ABI.
    /// We currently untuple arguments for RustCall ABI where the `spread_arg` is set.
    fn codegen_function_prelude(&mut self, body: &Body) {
        if let Some(spread_arg) = body.spread_arg() {
            self.codegen_spread_arg(body, spread_arg);
        }
    }

    /// MIR functions have a `spread_arg` field that specifies whether the
    /// final argument to the function is "spread" at the LLVM/codegen level
    /// from a tuple into its individual components. (Used for the "rust-
    /// call" ABI, necessary because the function traits and closures cannot have an
    /// argument list in MIR that is both generic and variadic, so Rust
    /// allows a generic tuple).
    ///
    /// These tuples are used in the MIR to invoke a shim, and it's used in the shim body.
    ///
    /// The `spread_arg` represents the the local variable that is to be "spread"/untupled.
    /// However, the function body itself may refer to the members of
    /// the tuple instead of the individual spread parameters, so we need to add to the
    /// function prelude code that _retuples_, that is, writes the arguments
    /// back to a local tuple that can be used in the body.
    ///
    /// See:
    /// <https://rust-lang.zulipchat.com/#narrow/stream/182449-t-compiler.2Fhelp/topic/Determine.20untupled.20closure.20args.20from.20Instance.3F>
    fn codegen_spread_arg(&mut self, body: &Body, spread_arg: Local) {}

    pub fn declare_function(&mut self, instance: Instance) {}
}
