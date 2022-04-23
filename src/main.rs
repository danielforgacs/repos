mod funcs;

mod prelude {
    pub use crate::funcs::*;
    pub use clap::{Arg, Command};
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

fn main() {
    let root_path = match get_root_path() {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(path) => path,
    };
    dbg!(&root_path);
    let repo_paths = find_git_repos_in_dir(&root_path);
    dbg!(&repo_paths);
}
