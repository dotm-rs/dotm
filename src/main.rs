use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod cmds;
mod config;
mod constants;
mod hint;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => match cmd {
            Commands::Init(args) => args.run()?,
            Commands::Sync => todo!(),
            Commands::Add => todo!(),
            Commands::Remove => todo!(),
        },
        None => todo!(),
    }

    Ok(())
}
