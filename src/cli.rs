use clap::{Parser, Subcommand};

use crate::cmds::init::InitArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the dotm. This creates all necessary directories and files.
    Init(InitArgs),

    /// Sync files between local machine and remote
    Sync,

    /// Add one or more files to be tracked by dotm
    Add,

    /// Remove one or more tracked files
    Remove,
}
