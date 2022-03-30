use std::io::Write;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod repostatus;
mod repo;
mod tui;
mod config;

/// Zero based termion goto.
fn goto(x: u16, y: u16) -> termion::cursor::Goto {
    termion::cursor::Goto(x + 1, y + 1)
}

fn main() {
    let conf = config::Opts::new();
    tui(conf);
}

fn tui(conf: config::Opts) {
    let bg_current_cell = color::Bg(color::Rgb(75, 30, 15));
    let bg_reset = color::Bg(color::Reset);

    let fg_master_ok = color::Fg(color::Rgb(0, 175, 0));
    let fg_master_not_ok = color::Fg(color::Rgb(255, 180, 0));
    let fg_not_master_ok = color::Fg(color::Rgb(0, 200, 255));
    let fg_not_master_not_ok = color::Fg(color::Rgb(225, 0, 0));

    let fg_active_branch = color::Fg(color::Rgb(35, 200, 35));
    let fg_inactive_branch = color::Fg(color::Rgb(90, 90, 90));

    let bg_info = color::Bg(color::Rgb(20, 20, 20));
    let fg_info = color::Fg(color::Rgb(75, 75, 75));

    let fg_reset = color::Fg(color::Reset);

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdout = termion::screen::AlternateScreen::from(stdout);
    let mut keep_running = true;
    let mut tui = tui::Tui::new();

    let devdir_path = format!(
        "{}{}{}{}{}",
        goto(0, 0),
        bg_info,
        fg_info,
        conf.get_dev_dir().to_string_lossy(),
        bg_reset,
    );
    let header = format!(
        "{}{}{:>re$} |{:^st$}| Branches ------->",
        goto(0, 1),
        fg_info,
        "<------- Repo",
        "stat",
        re = conf.repo_name_width,
        st = conf.repo_status_width - 2,
    );
    let mut repos: Vec<repo::Repo> = conf.get_repo_paths()
        .iter()
        .map(|path| repo::Repo::new(path.to_path_buf(), &conf.repo_name_width))
        .collect();
    if repos.is_empty() {
        println!("No repos found.");
        return;
    }
    let footer = format!(
        "{}U: untracked, D: deleted, d: deleted staged, S: staged{}M: modified, N: new file, n: new file 2",
        goto(1, repos.len() as u16+4),
        goto(1, repos.len() as u16+5),
    );

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        write!(stdout, "{}", devdir_path).unwrap();
        write!(stdout, "{}", header).unwrap();
        write!(stdout, "{}", footer).unwrap();
        tui.reset();
        tui.row_count = repos.len();

        for (i, repo) in repos.iter_mut().enumerate() {
            tui.row_column_counts.push(repo.branches.len() as u16 + 2);

            if i == tui.current_row as usize {
                repo.update();
            }

            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();
            match repo.get_repo_state() {
                repo::RepoState::MasterOk => write!(stdout, "{}", fg_master_ok).unwrap(),
                repo::RepoState::MasterNotOk => write!(stdout, "{}", fg_master_not_ok).unwrap(),
                repo::RepoState::NotMasterOK => write!(stdout, "{}", fg_not_master_ok).unwrap(),
                repo::RepoState::NotMasterNotOK => write!(stdout, "{}", fg_not_master_not_ok).unwrap(),
            }
            {
                if tui.is_current_cell() {
                    write!(stdout, "{}", bg_current_cell).unwrap();
                }
                write!(stdout, "{}", repo.name).unwrap();
            }

            write!(stdout, "{}", bg_reset).unwrap();
            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();

            {
                if tui.is_current_cell() {
                    write!(stdout, "{}", bg_current_cell).unwrap();
                }
                write!(stdout, "[{}]", repo.status.to_string()).unwrap();
            }

            write!(stdout, "{}", fg_reset).unwrap();
            write!(stdout, "{}", bg_reset).unwrap();

            for branch in &repo.branches {
                write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();

                {
                    if tui.is_current_cell() {
                        write!(stdout, "{}", bg_current_cell).unwrap();
                    }
                    if branch == repo.current_branch.as_str() {
                        write!(stdout, "{}", fg_active_branch).unwrap();
                    } else {
                        write!(stdout, "{}", fg_inactive_branch).unwrap();
                    }
                    if tui.column > conf.get_max_width() {
                        write!(stdout, "...").unwrap();
                        write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
                        break;
                    } else {
                        write!(stdout, "{}", branch).unwrap();
                    }
                    write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
                }

                tui.adjust_column_width(branch.len() as u16);
            }

            tui.finished_row();
        }

        let branch_index = match tui.current_column_id {
            0 | 1 | 2 => 0_usize,
            _ => tui.current_column_id as usize - 2,
        };
        write!(
            stdout,
            "{}{}{} {{{}}} <-- {}{}",
            goto(0, repos.len() as u16 + 3),
            bg_info,
            repos[tui.current_row as usize].name,
            repos[tui.current_row as usize].current_branch,
            repos[tui.current_row as usize].branches[branch_index],
            bg_reset,
        )
        .unwrap();

        if !repos[tui.current_row as usize].status.is_ok() {
            write!(
                stdout,
                "{}{}",
                goto(0, repos.len() as u16 + 7),
                repos[tui.current_row as usize].status_text,
            ).unwrap();
        }

        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    keep_running = false;
                    break;
                }
                Key::Right | Key::Char('l') => {
                    tui.go(tui::MoveDirection::Right);
                    break;
                }
                Key::Left | Key::Char('h') => {
                    tui.go(tui::MoveDirection::Left);
                    break;
                }
                Key::Up | Key::Char('k') => {
                    tui.go(tui::MoveDirection::Up);
                    break;
                }
                Key::Down | Key::Char('j') => {
                    tui.go(tui::MoveDirection::Down);
                    break;
                }
                Key::Char('\n') => {
                    match tui.current_column_id {
                        0 => {
                            std::process::Command::new("gnome-terminal")
                            .arg(format!("--working-directory={}", repos[tui.current_row as usize].path.as_path().to_str().unwrap()))
                            .output().ok();
                        }
                        1 => {
                            repos[tui.current_row as usize].clear_stat();
                            break;
                        }
                        _ => {
                            let branch = repos[tui.current_row as usize].branches
                                [tui.current_column_id as usize - 2]
                                .to_owned();
                            repos[tui.current_row as usize].checkout_branch(branch);
                        }
                    }
                    break;
                }
                _ => {}
            }
        }
    }
}
