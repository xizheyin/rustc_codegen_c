use super::fs_utils::ProjectDirs;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Debug)]
pub struct CompileToolChain {
    pub(crate) cargo: PathBuf,
    pub(crate) rustc: PathBuf,
    pub(crate) rustflags: Vec<String>,
    pub(crate) triple: String,
    pub(crate) runner: Vec<String>,
}

/// default toolchain for current machine
impl Default for CompileToolChain {
    fn default() -> Self {
        use super::toolchain_info::*;
        Self {
            cargo: get_cargo_path(),
            rustc: get_rustc_path(),

            rustflags: Default::default(),

            triple: get_host_triple(&get_rustc_path()),
            runner: Default::default(),
        }
    }
}

impl CompileToolChain {}

#[derive(Debug)]
pub struct CargoProject {
    project_dirs: ProjectDirs,
    target_name: String,
}

impl CargoProject {
    pub(crate) fn new(project_dirs: ProjectDirs, target: &str) -> Self {
        Self {
            project_dirs,
            target_name: target.to_string(),
        }
    }

    pub(crate) fn source_dir(&self) -> PathBuf {
        self.project_dirs.get_source_dir()
    }

    pub(crate) fn manifest_path(&self) -> PathBuf {
        self.project_dirs.get_manifest_dir()
    }

    pub(crate) fn target_dir(&self) -> PathBuf {
        self.project_dirs.build_dir.join(&self.target_name)
    }

    fn base_cmd(&self, command: &str, cargo: &Path) -> Command {
        let mut cmd = Command::new(cargo);

        cmd.arg(command)
            .arg("--manifest-path")
            .arg(self.manifest_path())
            .arg("--target-dir")
            .arg(self.target_dir())
            .arg("--locked");

        cmd
    }

    #[must_use]
    fn build_cmd(&self, command: &str, compile_toolchain: &CompileToolChain) -> Command {
        let mut cmd = self.base_cmd(command, &compile_toolchain.cargo);

        cmd.arg("--target").arg(&compile_toolchain.triple);

        cmd.env("RUSTC", &compile_toolchain.rustc);

        rustflags_to_cmd_env(&mut cmd, "RUSTFLAGS", &compile_toolchain.rustflags);

        if !compile_toolchain.runner.is_empty() {
            cmd.env(
                format!(
                    "CARGO_TARGET_{}_RUNNER",
                    compile_toolchain.triple.to_uppercase().replace('-', "_")
                ),
                compile_toolchain.runner.join(" "),
            );
        }

        cmd
    }

    pub(crate) fn clean(&self) {
        let _ = self.project_dirs.clean_all();
    }

    /// cargo build with specified arguments
    pub(crate) fn build(&self, compiler: &CompileToolChain) -> Command {
        self.build_cmd("build", compiler)
    }

    /// cargo run with specified arguments
    pub(crate) fn run(&self, compiler: &CompileToolChain) -> Command {
        self.build_cmd("run", compiler)
    }
}

// Adapted from https://github.com/rust-lang/cargo/blob/6dc1deaddf62c7748c9097c7ea88e9ec77ff1a1a/src/cargo/core/compiler/build_context/target_info.rs#L750-L77
pub(crate) fn rustflags_from_env(kind: &str) -> Vec<String> {
    // First try CARGO_ENCODED_RUSTFLAGS from the environment.
    // Prefer this over RUSTFLAGS since it's less prone to encoding errors.
    if let Ok(a) = std::env::var(format!("CARGO_ENCODED_{}", kind)) {
        if a.is_empty() {
            return Vec::new();
        }
        return a.split('\x1f').map(str::to_string).collect();
    }

    // Then try RUSTFLAGS from the environment
    if let Ok(a) = std::env::var(kind) {
        let args = a
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        return args.collect();
    }

    // No rustflags to be collected from the environment
    Vec::new()
}

/// environment variables: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub(crate) fn rustflags_to_cmd_env(cmd: &mut std::process::Command, kind: &str, flags: &[String]) {
    cmd.env(format!("CARGO_ENCODED_{}", kind), flags.join("\x1f"));
}
