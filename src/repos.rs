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
    let mut previous_repo_name: Option<String> = None;
    let mut previous_branch_name: Option<String> = None;

    loop {
        let mut repos = collect_repos(&root_path)?;
        match repos_sort {
            RepoSort::Name => repos.sort_by_key(|k| k.name().to_owned()),
            RepoSort::CurrentBranch => repos.sort_by_key(|k| k.current_branch().to_owned()),
            RepoSort::Status => repos.sort_by_key(|k| k.status().to_string()),
        }
        tui.clear()?;

        if let Some(repo_name) = &previous_repo_name {
            for (index, repo) in repos.iter().enumerate() {
                if repo_name == repo.name() {
                    tui.set_selected_row(index as u16);
                    previous_repo_name = None;
                    if let Some(branch_name) = &previous_branch_name {
                        eprintln!("restoring branch: {}", branch_name);
                        let branches = match repos_sort {
                            RepoSort::Name => repo.branches().to_owned(),
                            RepoSort::CurrentBranch => repo.current_and_branches(),
                            RepoSort::Status => repo.branches().to_owned(),
                        };
                        for (column_index, name) in branches.iter().enumerate() {
                            if branch_name == name {
                                eprintln!("...new index: {}", column_index);
                                tui.set_selected_column(column_index as u16 + BRANCH_COLUMN_OFFSET);
                                previous_branch_name = None;
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }

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
            let selected_repo = &repos[usize::from(tui.selected_coord().get_row())];
            let selected_colum_type = tui.selected_coord().get_column().to_column();
            let selected_column = tui.selected_coord().get_column();
            let selected_row = tui.selected_coord().get_row();

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
                previous_repo_name = Some(selected_repo.name().to_string());
                match selected_colum_type {
                    Column::Branches => {
                        let branches = match repos_sort {
                            RepoSort::Name => selected_repo.branches().to_owned(),
                            RepoSort::CurrentBranch => selected_repo.current_and_branches(),
                            RepoSort::Status => selected_repo.branches().to_owned(),
                        };
                        // let index = usize::from(selected_column) - usize::from(BRANCH_COLUMN_OFFSET);
                        let index = usize::from(selected_column) - BRANCH_COLUMN_OFFSET as usize;
                        previous_branch_name = Some(branches[index].to_string());
                        eprintln!("storing prev brahcn: {}, {:?}", index, &previous_branch_name);
                    },
                    _ => {},
                };
                match repos_sort {
                    RepoSort::Name => repos_sort = RepoSort::Status,
                    RepoSort::Status => repos_sort = RepoSort::CurrentBranch,
                    RepoSort::CurrentBranch => repos_sort = RepoSort::Name,
                };
            }

            // Action
            if event == Event::Key(KeyCode::Enter.into()) {
                match selected_colum_type {
                    Column::Name => {}
                    Column::Status => {}
                    Column::Branches => {
                        if selected_repo.status().status_type() == StatusType::Clean {
                            let branches = match repos_sort {
                                RepoSort::Name => selected_repo.branches().to_owned(),
                                RepoSort::CurrentBranch => selected_repo.current_and_branches(),
                                RepoSort::Status => selected_repo.branches().to_owned(),
                            };
                            let branch = branches[usize::from(selected_column - BRANCH_COLUMN_OFFSET)].to_owned();
                            if branch != "(no branch)" {
                                let abs_branch = format!("refs/heads/{}", branch);
                                selected_repo.git_repo.set_head(&abs_branch)?;
                            }
                        }
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
