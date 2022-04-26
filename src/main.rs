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
    // let repos: Vec<_> = repo_paths
    // let repos = repo_paths
    //     .into_iter()
    //     .map(|f| Repo::new(&f))
    //     .map(|f| f.unwrap())
    //     .collect::<Vec<Repo>>();
    for repo in get_repos(&repo_paths) {
        println!(r#"{}::{}"#,
            repo.get_name(),
            repo.get_current_branch()
        )
        // print!("--> {}", repo.get_name());
        // print!("current branch:\t{}", repo.get_current_branch());
        // print!("status:\t{}", repo.get_status());
        // print!("branches:\t{}", repo.get_branches().join(" "));
    }

    // for repo in repos {
    // }

    Ok(())
}
