mod utils;
mod repo;
mod repostatus;

mod prelude {
    pub use crate::utils::*;
    pub use crate::repo::*;
    pub use crate::repostatus::*;
    pub use clap::{Arg, Command};
    pub use git2::{ErrorCode, Repository};
    pub use std::{
        env::var,
        fs, io,
        io::{Error, ErrorKind, stdout},
        path::{Path, PathBuf},
        time::{Duration, Instant},
        thread::sleep,
    };
    pub use crossterm::{
        execute,
        cursor::position,
        event::{poll, read, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    };
    pub type ReposError<T> = Result<T, Box<dyn std::error::Error>>;
    pub const DEV_DIR_ENV_VAR: &str = "DEVDIR";
    pub const UPDATE_DELAY_SECS: f32 = 0.5;
}

use prelude::*;

fn run(root_path: PathBuf) -> ReposError<()> {
    enable_raw_mode()?;
    loop {
        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            println!("Event::{:?}\r", event);
            if event == Event::Key(KeyCode::Up.into()) {
            }
            if event == Event::Key(KeyCode::Down.into()) {
            }
            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            execute!(stdout(), Clear(ClearType::All))?;
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
    disable_raw_mode()?;
    Ok(())
}

fn main() {
    if let Err(error) = get_root_path().and_then(run) {
        eprintln!("Error: {}", error);
        return;
    };
}
