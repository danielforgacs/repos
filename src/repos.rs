use crate::prelude::*;

enum RepoSort {
    Name,
    CurrentBranch,
    Status,
}

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    tui.print(&format!("{}", crossterm::cursor::Hide))?;
    let mut repos_sort = RepoSort::Name;

    loop {
        let mut repos = collect_repos(&root_path)?;
        match repos_sort {
            RepoSort::Name => repos.sort_by_key(|k| k.name().to_owned()),
            RepoSort::CurrentBranch => repos.sort_by_key(|k| k.current_branch().to_owned()),
            RepoSort::Status => repos.sort_by_key(|k| k.status().to_string()),
        }
        tui.clear()?;

        for repo in repos.iter() {
            if repo.is_on_master() && repo.status().status_type() == StatusType::Clean {
                tui.set_cell_style(CellStyle::CleanMaster);
            } else if repo.is_on_master() && repo.status().status_type() != StatusType::Clean {
                tui.set_cell_style(CellStyle::DirtyMaster);
            } else if !repo.is_on_master() && repo.status().status_type() == StatusType::Clean {
                tui.set_cell_style(CellStyle::CleanBranch);
            } else if !repo.is_on_master() && repo.status().status_type() != StatusType::Clean {
                tui.set_cell_style(CellStyle::DirtyBranch);
            }
            tui.print(&text_to_width(repo.name(), &(REPO_NAME_WIDTH as usize)))?;
            tui.print(&format!("{}", repo.status()))?;
            let branches = match repos_sort {
                RepoSort::Name => repo.branches().to_owned(),
                RepoSort::CurrentBranch => repo.current_and_branches(),
                RepoSort::Status => repo.branches().to_owned(),
            };
            for branch in branches {
                if branch == repo.current_branch() {
                    tui.cell_style = CellStyle::CurrentBranch;
                } else {
                    tui.cell_style = CellStyle::Branch;
                }
                tui.print(&limit_text(&branch, &MAX_BRANCH_NAME_WIDTH))?;
            }
            tui.new_line()?;
        }

        tui.flush()?;

        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            // Navogation.
            if event == Event::Key(KeyCode::Up.into()) || event == Event::Key(KeyCode::Char('k').into()) {
                tui.go(Direction::Up);
            }
            if event == Event::Key(KeyCode::Down.into()) || event == Event::Key(KeyCode::Char('j').into()) {
                tui.go(Direction::Down);
            }
            if event == Event::Key(KeyCode::Left.into()) || event == Event::Key(KeyCode::Char('h').into()) {
                tui.go(Direction::Left);
            }
            if event == Event::Key(KeyCode::Right.into()) || event == Event::Key(KeyCode::Char('l').into()) {
                tui.go(Direction::Right);
            }
            //Sorting.
            if event == Event::Key(KeyCode::Char('s').into()) {
                match repos_sort {
                    RepoSort::Name => repos_sort = RepoSort::Status,
                    RepoSort::Status => repos_sort = RepoSort::CurrentBranch,
                    RepoSort::CurrentBranch => repos_sort = RepoSort::Name,
                };
            }
            if event == Event::Key(KeyCode::Enter.into()) {
                let coord = tui.selected_coord();
                let repo = &repos[coord.1 as usize];
                if coord.0 == 1 {
                    // Clean status here
                } else if coord.0 > 1 && repo.status().status_type() == StatusType::Clean {
                    let branch = &repo.branches()[(coord.0 - 2) as usize];
                    if branch != "(no branch)" {
                        let abs_branch = format!("refs/heads/{}", branch);
                        repo.git_repo.set_head(&abs_branch)?;
                    }
                }
            }
            // quit.
            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }
    }

    tui.print(&format!("{}", crossterm::cursor::Show))?;
    disable_raw_mode()?;
    Ok(())
}

fn collect_repos(path: &Path) -> ReposResult<Vec<Repo>> {
    let mut repos: Vec<Repo> = Vec::new();
    for dir in find_git_repos_in_dir(path)? {
        repos.push(Repo::new(&dir)?)
    }
    Ok(repos)
}
