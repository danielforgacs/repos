mod funcs;
mod repo;

mod prelude {
    pub use crate::funcs::*;
    pub use crate::repo::*;
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
    for path in repo_paths {
        println!("_________________________________________");
        let repo = Repo::new(&path)?;
        println!("name:\t\t{}", repo.get_name());
        println!("current branch:\t{}", repo.get_current_branch());
    }
    Ok(())
}
