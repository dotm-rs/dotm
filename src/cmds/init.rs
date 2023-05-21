use std::fs;

use clap::Args;
use colored::Colorize;
use thiserror::Error;
use xdg::BaseDirectoriesError;

use crate::{
    config::Config,
    constants::XDG_PREFIX,
    git::{self, GitCloneError},
    hint::Hint,
};

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Optional remote repository to init with
    #[arg(value_name = "URL")]
    repo: Option<String>,

    /// Optional profile name
    #[arg(short, long, default_value = "default")]
    profile: String,
}

#[derive(Debug, Error)]
pub enum InitCmdError {
    #[error("xdg error: {0}")]
    XdgError(#[from] BaseDirectoriesError),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("profile already exists")]
    ProfileAlreadyPresent,

    #[error("error while cloning: {0}")]
    GitCloneError(#[from] GitCloneError),

    #[error("init error: {0}")]
    InitError(#[from] gix::init::Error),
}

impl InitArgs {
    pub fn run(&self) -> Result<(), InitCmdError> {
        let base_data_dir = xdg::BaseDirectories::with_prefix(XDG_PREFIX)?.get_data_home();
        let profile_dir = base_data_dir.join(&self.profile);
        let git_dir = base_data_dir.join(".git");

        // Only create the base dir if needed
        if !base_data_dir.exists() {
            fs::create_dir_all(&base_data_dir)?;
        }

        // Only create the profile dir if needed
        if !profile_dir.exists() {
            fs::create_dir_all(&profile_dir)?;
        } else {
            let hint = Hint::builder()
                .with_title(format!("Profile \"{}\" already exists", &self.profile))
                .with_before_text("It seems like the selected profile already exists. To continue, use any of the following commands:")
                .with_items(vec![
                    format!(
                        "Use \"{}\" to create a new profile",
                        "dotm init (URL) --profile <PROFILE>".bright_blue()
                    ),
                    format!(
                        "Use \"{}\" to remove the profile",
                        format!("dotm profile remove {}", &self.profile).bright_blue()
                    ),
                ])
                .build();

            println!("{hint}");
            return Err(InitCmdError::ProfileAlreadyPresent);
        }

        println!("Initializing using profile {}", self.profile.green());

        let repo = match &self.repo {
            Some(url) => git::clone_repository(&url, git_dir)?,
            None => {
                // If the user didn't provide a remote URL, init will only create a bare repository
                let repo = gix::init_bare(git_dir)?;

                // Next create the config file
                let config = Config {
                    profile: self.profile.clone(),
                    profiles: vec![self.profile.clone()],
                };

                config.create_if_needed()?;

                repo
            }
        };

        Ok(())
    }
}
