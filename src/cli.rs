use structopt::StructOpt;

use crate::build_project::{self, CompileToolChain, ProjectDirs};

#[derive(StructOpt, Debug)]
#[structopt(about = "a rust codegen c controller")]
pub enum RustCodegenCCli {
    Build(Build),
}

/// Build rgenc
#[derive(StructOpt, Debug)]
struct Build {
    /// Activate debug mode
    #[structopt(long)]
    release: bool,

    /// Input file
    #[structopt(name = "INPUT")]
    input: String,

    /// Output file, stdout if not present
    #[structopt(name = "OUTPUT", short, long)]
    output: Option<String>,
}

impl RustCodegenCCli {
    /// parse and match the action
    pub fn parse_and_run() {
        let rcgc = RustCodegenCCli::from_args();

        let dirs = ProjectDirs::default();
        let use_unstable_features = true;
        let compile_toolchain = CompileToolChain::default();

        match rcgc {
            Self::Build(build) => build_project::build_backend(
                dirs,
                build.release,
                compile_toolchain,
                use_unstable_features,
            ),
        };
    }
}
