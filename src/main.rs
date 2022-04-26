mod funcs;
mod repo;
mod repostatus;

mod prelude {
    pub use crate::funcs::*;
    pub use crate::repo::*;
    pub use crate::repostatus::*;
    pub use clap::{Arg, Command};
    pub use git2::{ErrorCode, Repository};
    pub use std::{
        env::var,
        fs, io,
        io::{Error, ErrorKind},
        path::{Path, PathBuf},
    };

    pub type ReposError<T> = Result<T, Box<dyn std::error::Error>>;
    pub const DEV_DIR_ENV_VAR: &str = "DEVDIR";
    pub const REPO_NAME_LENGTH: usize = 20;
    pub const BRANCH__NAME_LENGTH: usize = 20;
}

use prelude::*;

fn main() -> ReposError<()> {
    let root_path = match get_root_path() {
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
        Ok(path) => path,
    };
    let repo_paths = match find_git_repos_in_dir(&root_path) {
        Ok(paths) => paths,
        Err(error) => {
            println!("Could not get repository paths: {}", error);
            return Ok(());
        }
    };
    for repo in get_repos(&repo_paths) {
        println!(r#"{}::{}::{}::{}"#,
            repo.get_name(),
            repo.get_current_branch(),
            repo.get_status(),
            repo.get_branches().join(" ")
        )
    }
    Ok(())
}
