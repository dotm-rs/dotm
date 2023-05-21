use clap::{Parser, Subcommand};

use crate::cmds::{init::InitArgs, profile::ProfileArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize dotm. This creates all necessary directories and files.
    Init(InitArgs),

    /// Sync files between local machine and remote
    Sync,

    /// Add one or more files to be tracked by dotm
    Add,

    /// Remove one or more tracked files
    #[command(alias("rm"))]
    Remove,

    #[command(alias("pr"))]
    Profile(ProfileArgs),
}
