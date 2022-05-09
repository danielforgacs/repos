mod utils;
mod repo;
mod repos;
mod repostatus;

mod prelude {
    pub use crate::utils::*;
    pub use crate::repo::*;
    pub use crate::repos::run;
    pub use crate::repostatus::*;
    pub use clap::{
        Arg,
        Command,
    };
    pub use git2::{
        ErrorCode,
        Repository,
    };
    pub use std::{
        fs,
        io,
        env::var,
        io::{Error, ErrorKind, stdout},
        path::{Path, PathBuf},
        time::{Duration, Instant},
        thread::sleep,
    };
    pub use crossterm::{
        event::{
            poll,
            read,
            Event,
            KeyCode,
        },
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
        },
    };
    pub type ReposError<T> = Result<T, Box<dyn std::error::Error>>;
    pub const DEV_DIR_ENV_VAR: &str = "DEVDIR";
    pub const UPDATE_DELAY_SECS: f32 = 0.5;
}

use prelude::*;

fn main() {
    if let Err(error) = get_root_path().and_then(run) {
        eprintln!("Error: {}", error);
    };
}
