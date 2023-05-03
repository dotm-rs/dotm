use std::fs;

use clap::Args;
use colored::Colorize;
use thiserror::Error;
use xdg::BaseDirectoriesError;

use crate::{constants::XDG_PREFIX, hint::Hint};

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

    #[error("url parse error: {0}")]
    UrlParseError(#[from] gix::url::parse::Error),

    #[error("clone error: {0}")]
    CloneError(#[from] gix::clone::Error),

    #[error("fetch error: {0}")]
    FetchError(#[from] gix::clone::fetch::Error),

    #[error("checkout error: {0}")]
    CheckoutError(#[from] gix::clone::checkout::main_worktree::Error),

    #[error("init error: {0}")]
    InitError(#[from] gix::init::Error),
}

impl InitArgs {
    pub fn run(&self) -> Result<(), InitCmdError> {
        println!("Initializing using profile {}", self.profile.green());

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
                .with_title("Profile already exists")
                .with_items(vec![
                    "Use \"dotm init (URL) --profile <PROFILE>\" to create a new profile".into(),
                    format!("Use \"dotm profile remove {}\" to remove the profile. Caution - This will delete all files associated with the profile!", &self.profile)
                ]);

            return Err(InitCmdError::ProfileAlreadyPresent);
        }

        match &self.repo {
            Some(url) => {
                // If the user provided a remote URL, init will clone and checkout the repository
                gix::interrupt::init_handler(|| {})?;
                let url = gix::url::parse(url.as_bytes().into())?;

                // Cloning
                let mut clone = gix::prepare_clone(url, git_dir)?;
                let (mut checkout, _) = clone
                    .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

                // Checkout
                let (repo, _) = checkout
                    .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
            }
            None => {
                // If the user didn't provide a remote URL, init will only create a bare repository
                let repo = gix::init_bare(git_dir)?;
            }
        }

        Ok(())
    }
}
