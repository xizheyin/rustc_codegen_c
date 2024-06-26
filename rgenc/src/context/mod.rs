use rust2cast::CProgram;
use rustc_middle::ty::print::{with_forced_trimmed_paths, with_no_trimmed_paths};
use rustc_middle::{mir::mono::MonoItem, ty::TyCtxt};

pub(crate) struct ToCContext<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub ast: CProgram,
}

impl<'tcx> ToCContext<'tcx> {
    pub(crate) fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            ast: CProgram::default(),
        }
    }

    pub(crate) fn codegen_item(&mut self, item: &MonoItem<'tcx>) {
        match item {
            MonoItem::Fn(instance) => {
                // println!("fn name: {}", item.symbol_name(self.tcx));
                // println!(
                //     "fn unmangled name: {}",
                //     //with_forced_trimmed_paths
                //     with_no_trimmed_paths!(self.tcx.def_path_str(instance.def_id()))
                // );
                let _ = self.codegen_function(*instance);
            }
            MonoItem::Static(static_def) => todo!(),
            MonoItem::GlobalAsm(asm) => {
                eprintln!("Unsupport: GlobalAsm {:?}", asm);
            }
        }
    }
}

fn get_mono_item_name<'tcx>(tcx: TyCtxt<'tcx>, mono_item: &MonoItem<'tcx>) -> Option<String> {
    match mono_item {
        MonoItem::Fn(instance) => {
            // 获取函数的 DefId
            let def_id = instance.def_id();
            // 使用 TyCtxt 来获取未改名的名字
            Some(tcx.def_path_str(def_id))
        }
        MonoItem::Static(def_id) => {
            // 对于静态项或全局汇编，直接使用 DefId
            Some(tcx.def_path_str(*def_id))
        }
        MonoItem::GlobalAsm(_) => None,
    }
}
