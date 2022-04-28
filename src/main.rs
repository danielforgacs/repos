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
    pub use crossterm::{
        cursor::position,
        event::{poll, read, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode},
    };
    pub type ReposError<T> = Result<T, Box<dyn std::error::Error>>;
    pub const DEV_DIR_ENV_VAR: &str = "DEVDIR";
    pub const UPDATE_DELAY_SECS: f32 = 1.5;
}

use prelude::*;

fn print_events(root_path: &PathBuf) -> ReposError<()> {
    loop {
        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            println!("Event::{:?}\r", event);
            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            for repo_path in find_git_repos_in_dir(&root_path)? {
                let repo = Repo::new(&repo_path)?;
                println!("{}::{}::{}::{}\r",
                    repo.get_name(),
                    repo.get_current_branch(),
                    repo.get_status(),
                    repo.get_branches().join(" ")
                )
            };
        }
    }
    Ok(())
}

fn main() -> ReposError<()> {
    let root_path = match get_root_path() {
        Err(err) => {
            println!("{}\r", err);
            return Ok(());
        }
        Ok(path) => path,
    };
    println!("--> [DEVDIR:{}]\r", root_path.to_string_lossy());
    enable_raw_mode()?;
    print_events(&root_path)?;
    disable_raw_mode()?;
    Ok(())
}
