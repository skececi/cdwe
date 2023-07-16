use crate::cmd::init::Shell;
use clap::{Subcommand, command, Parser};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Run {
        #[arg(long = "old_dir", required = true)]
        old_dir: String,
        #[arg(long = "new_dir", required = true)]
        new_dir: String,
    },
    #[command(arg_required_else_help = true)]
    Init {
        #[arg(value_name = "SHELL", required = true)]
        shell: Option<Shell>,
    },
}
