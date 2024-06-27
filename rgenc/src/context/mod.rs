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
                let item_mangled_name = item.symbol_name(self.tcx).to_string();
                let _item_unmangled_name =
                    with_no_trimmed_paths!(self.tcx.def_path_str(instance.def_id()));
                println!("fn name: {}", item_mangled_name);
                //println!("fn name: {}", item_unmangled_name);

                let _ = self.codegen_function(*instance, item_mangled_name);
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

/// Escapes the name of a function
pub fn function_name(name: rustc_middle::ty::SymbolName) -> String {
    let mut name: String = name.to_string();

    if name.len() > 1000 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        //TODO: make hashes consitant!
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        format!("{}_{}", &name[..1000], calculate_hash(&name)).into()
    } else {
        name.into()
    }
}
