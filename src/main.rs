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
        time::{Duration, Instant},
        thread::sleep,
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
    println!("--> {}", root_path.to_string_lossy());
    for repo_path in find_git_repos_in_dir(&root_path)? {
        let repo = Repo::new(&repo_path)?;
        println!(r#"{}::{}::{}::{}"#,
            repo.get_name(),
            repo.get_current_branch(),
            repo.get_status(),
            repo.get_branches().join(" ")
        )
    };
    Ok(())
}
