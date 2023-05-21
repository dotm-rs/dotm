use std::path::PathBuf;

use gix::Repository;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitCloneError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("url parse error: {0}")]
    UrlParseError(#[from] gix::url::parse::Error),

    #[error("clone error: {0}")]
    CloneError(#[from] gix::clone::Error),

    #[error("clone fetch error: {0}")]
    CloneFetchError(#[from] gix::clone::fetch::Error),

    #[error("clone checkout error: {0}")]
    CloneCheckoutError(#[from] gix::clone::checkout::main_worktree::Error),
}

/// Clones a remote [`Repository`] from `url` and places git worktree data into
/// `git_dir`.
pub fn clone_repository(url: &str, git_dir: PathBuf) -> Result<Repository, GitCloneError> {
    // If the user provided a remote URL, init will clone and checkout the repository
    gix::interrupt::init_handler(|| {})?;
    let url = gix::url::parse(url.into())?;

    // Cloning
    let mut clone = gix::prepare_clone(url, git_dir)?;
    let (mut checkout, _) =
        clone.fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

    // Checkout
    let (repo, _) =
        checkout.main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

    Ok(repo)
}
