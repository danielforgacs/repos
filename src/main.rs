mod funcs;

mod prelude {
    pub use crate::funcs::get_root_path;
    pub use clap::{Arg, Command};
    pub use std::env::var;
    pub use std::path::{Path, PathBuf};

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
    dbg!(root_path);
}
