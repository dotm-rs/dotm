use clap::{Args, Subcommand};
use thiserror::Error;

#[derive(Debug, Args)]
pub struct ProfileArgs {
    #[command(subcommand)]
    command: ProfileCommands,
}

#[derive(Debug, Subcommand)]
pub enum ProfileCommands {
    #[command(alias("status"))]
    Info,
}

#[derive(Debug, Error)]
pub enum ProfileCommandError {}

impl ProfileArgs {
    pub fn run(&self) -> Result<(), ProfileCommandError> {
        Ok(())
    }
}
