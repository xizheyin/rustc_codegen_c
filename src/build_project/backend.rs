use crate::build_project::toolchain_info::get_file_name;

use super::fs_utils::ProjectDirs;
use super::project::CargoProject;
use super::project::CompileToolChain;
use super::project::{rustflags_from_env, rustflags_to_cmd_env};
use std::env;
use std::path::PathBuf;

pub fn build_backend(
    dirs: ProjectDirs,
    release: bool,
    compile_toolchain: CompileToolChain,
    use_unstable_features: bool,
) -> PathBuf {
    let cg_c_proj: CargoProject = CargoProject::new(dirs, "cg_c");

    let mut cmd = cg_c_proj.build(&compile_toolchain);

    let mut rustflags = rustflags_from_env("RUSTFLAGS");

    rustflags.push("-Zallow-features=rustc_private".to_owned());

    if env::var("CG_CLIF_EXPENSIVE_CHECKS").is_ok() {
        // Enabling debug assertions implicitly enables the clif ir verifier
        cmd.env("CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS", "true");
        cmd.env("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "true");
    }

    if use_unstable_features {
        cmd.arg("--features").arg("unstable-features");
    }

    let channel = if release {
        cmd.arg("--release");
        "release"
    } else {
        "debug"
    };

    rustflags_to_cmd_env(&mut cmd, "RUSTFLAGS", &rustflags);

    eprintln!("[BUILD] rustc_codegen_cranelift");
    super::cmd_utils::spawn_and_wait(cmd);

    cg_c_proj
        .target_dir()
        .join(&compile_toolchain.triple)
        .join(channel)
        .join(get_file_name(&compile_toolchain.rustc, "rgenc", "dylib"))
}
