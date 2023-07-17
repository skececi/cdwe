mod cmd;
mod config;
use anyhow::Result;
use clap::Parser;
use cmd::{init_shell, run, Cli};
use config::Config;

fn main() -> Result<()> {
    let matches = Cli::parse();

    match matches.command {
        cmd::Commands::Init { shell } => {
            init_shell(None, shell.unwrap())?;
        }
        cmd::Commands::Run { old_dir, new_dir } => {
            let config = Config::from_config_file(&format!(
                "{}/{}",
                std::env::var("HOME").unwrap(),
                "cdwe.toml"
            ))?;
            run(&config, old_dir, new_dir)?;
        }
    }

    Ok(())
}
