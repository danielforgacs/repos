mod repo;
mod repos;
mod repostatus;
mod tui;
mod utils;

mod prelude {
    pub use crate::repo::*;
    pub use crate::repos::run;
    pub use crate::repostatus::*;
    pub use crate::tui::{CellStyle, Direction, Tui};
    pub use crate::utils::*;
    pub use clap::{Arg, Command as ClapCommand};
    pub use crossterm::{
        cursor::{MoveTo, MoveToColumn, MoveToNextLine},
        event::{poll, read, Event, KeyCode},
        style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
        terminal,
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
        Command, ExecutableCommand, QueueableCommand,
    };
    pub use git2::{ErrorCode, Repository, StatusOptions};
    pub use std::{
        env::var,
        fs,
        io::{self, Write},
        io::{stdout, Error, ErrorKind},
        path::{Path, PathBuf},
        thread::sleep,
        time::{Duration, Instant},
    };
    pub type ReposResult<T> = Result<T, Box<dyn std::error::Error>>;
    pub const DEV_DIR_ENV_VAR: &str = "DEVDIR";
    pub const UPDATE_DELAY_SECS: f32 = 0.5;
    pub const REPO_NAME_WIDTH: u16 = 28;
    pub const MAX_BRANCH_NAME_WIDTH: usize = 15;
}

use prelude::*;

fn main() {
    if let Err(error) = get_root_path().and_then(run) {
        eprintln!("Error: {}", error);
    };
}
