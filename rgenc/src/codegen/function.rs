use rust2cast::{CDeclaration, CFunction, CType};
use rustc_middle::{
    mir::{Body, HasLocalDecls},
    ty::{Instance, ParamEnv, TyKind},
};

use crate::context::ToCContext;

type CFnSig = (CType, Vec<CDeclaration>);

/// Codegen MIR functions into gotoc
impl<'tcx> ToCContext<'tcx> {
    fn codegen_declare_variables(&mut self, mir_body: &Body) -> Vec<CDeclaration> {
        let mut cdecls = vec![];

        for (idx, local_decl) in mir_body.local_decls().iter_enumerated() {
            if idx.as_usize() <= mir_body.arg_count {
                continue;
            }
            //FIXME: parse type

            cdecls.push(CDeclaration {
                name: idx.as_usize().to_string(),
                ctype: CType::Int,
                initializer: None,
            })
        }
        cdecls
    }

    pub(crate) fn codegen_function(
        &mut self,
        instance: Instance<'tcx>,
        name: String,
    ) -> Result<CFunction, String> {
        // check wether the instance is indeed a function.
        if let TyKind::FnDef(_, _) = instance.ty(self.tcx, ParamEnv::reveal_all()).kind() {
            //ALL OK.
        } else if let TyKind::Closure(_, _) = instance.ty(self.tcx, ParamEnv::reveal_all()).kind() {
            //println!("CLOSURE")
            return Err("0".to_string());
        } else {
            eprintln!("fn item {instance:?} is not a function definition type. Skippping.");
            return Err(format!(
                "fn item {instance:?} is not a function definition type."
            ));
        }

        let cfn_sig = self.extract_signature(&instance);
        let return_type = cfn_sig.0;
        let parameters = cfn_sig.1;

        let mir_body = self.tcx.instance_mir(instance.def);
        let mut body = vec![];

        let local_variables = self.codegen_declare_variables(mir_body);

        for bb in mir_body.basic_blocks.reverse_postorder() {
            //TODO generate statement
            let bb_data = mir_body.basic_blocks.get(*bb).unwrap();
            body.extend(self.codegen_basicblock(bb.as_usize(), bb_data).into_iter());
        }

        let cfunction = CFunction {
            return_type,
            name,
            parameters,
            local_variables,
            body,
        };

        println!("{}", cfunction.to_c_code());

        Ok(cfunction)
    }

    pub fn declare_function(&mut self, instance: Instance) {}

    fn extract_signature(&self, instance: &Instance<'tcx>) -> CFnSig {
        let mut cparams = vec![];

        let fn_ty = instance.ty(self.tcx, ParamEnv::reveal_all());

        // the function signature with generic parameters replaced

        let fn_sig = match fn_ty.kind() {
            TyKind::FnDef(_, _) => fn_ty.fn_sig(self.tcx),
            TyKind::Closure(_, args) => args.as_closure().sig(),
            _ => todo!("Can't get signature of {fn_ty}"),
        };

        for (idx, input) in fn_sig.inputs().iter().enumerate() {
            let ctype = self.codegen_ty(input.skip_binder());
            let cdecl = CDeclaration {
                name: idx.to_string(),
                ctype,
                initializer: None,
            };
            cparams.push(cdecl);
        }

        let cret = self.codegen_ty(&fn_sig.output().skip_binder());

        //println!("signature: {:#?}", fn_sig);
        (cret, cparams)
    }
}
