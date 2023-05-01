use std::fs;

use clap::Args;
use thiserror::Error;
use xdg::BaseDirectoriesError;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Optional repo to init with
    #[arg(value_name = "URL")]
    repo: Option<String>,

    #[arg(default_value = "default")]
    directory: String,
}

#[derive(Debug, Error)]
pub enum InitCmdError {
    #[error("xdg error: {0}")]
    XdgError(#[from] BaseDirectoriesError),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

impl InitArgs {
    pub fn run(&self) -> Result<(), InitCmdError> {
        let dir = xdg::BaseDirectories::with_prefix("dotm")?.get_data_home();
        fs::create_dir_all(dir)?;

        match &self.repo {
            Some(url) => {
                gix::interrupt::init_handler(|| {})?;
                let url = gix::url::parse(url.as_bytes().into());
            }
            None => todo!(),
        }

        Ok(())
    }
}
