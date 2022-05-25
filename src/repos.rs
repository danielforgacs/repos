use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    tui.print(&format!("{}", crossterm::cursor::Hide))?;

    loop {
        tui.clear()?;
        let repos = collect_repos(&root_path)?;

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
            for branch in repo.branches().iter().cloned() {
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

        let selected_column = tui.selected_coord().get_column();
        let selected_row = tui.selected_coord().get_row();
        let selected_repo = &repos[usize::from(selected_row)];
        let selected_repo_name = selected_repo.name();
        let mut selected_branch_name: Option<String> = None;
        if selected_column.to_column() == Column::Branches {
            selected_branch_name = Some(
                selected_repo.branches()[usize::from(selected_column - BRANCH_COLUMN_OFFSET)]
                    .to_string(),
            );
        }

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
            }

            // Action
            if event == Event::Key(KeyCode::Enter.into()) {
                eprintln!("{}:{}, {}", selected_column, selected_row, selected_repo_name);
                match selected_column.to_column() {
                    Column::Branches => {
                        if let Some(branch) = selected_branch_name {
                            if branch != NO_BRANCH_TEXT {
                                eprintln!("branch: {}", branch);
                                let abs_branch = format!("refs/heads/{}", branch);
                                selected_repo.git_repo.set_head(&abs_branch)?;
                            }
                        }
                    }
                    _ => {}
                };
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
