use std::{env::current_dir, path::PathBuf};

use crate::build_project::{self, CompileToolChain, ProjectDirs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "a rust codegen c controller")]
pub struct Cli {
    /// Output file, stdout if not present
    #[structopt(long, default_value = "output")]
    out_dir: String,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(StructOpt, Debug)]

enum SubCommand {
    Boot(Boot),
    Clean,
}

/// Build rgenc
#[derive(StructOpt, Debug)]
struct Boot {
    /// Activate release mode
    #[structopt(long)]
    release: bool,
}

impl Cli {
    /// parse and match the action
    pub fn parse_and_run() {
        let cli = Cli::from_args();

        let backend_source_dir = current_dir().unwrap().join("rgenc");
        let out_dir = PathBuf::from(cli.out_dir);
        let build_dir = out_dir.join("build");
        let dist_dir = out_dir.join("dist");

        let backend_dirs = ProjectDirs::new_and_create_dir(backend_source_dir, build_dir, dist_dir);
        let use_unstable_features = false;
        let compile_toolchain = CompileToolChain::default();

        match cli.subcmd {
            SubCommand::Boot(boot) => {
                let _backend = build_project::build_backend(
                    backend_dirs,
                    boot.release,
                    compile_toolchain,
                    use_unstable_features,
                );
                //println!("{:?}", backend);
            }
            SubCommand::Clean => {
                let _ = backend_dirs.clean_all();
            }
        };
    }
}
