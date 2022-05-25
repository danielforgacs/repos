use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    tui.print(&format!("{}", crossterm::cursor::Hide))?;

    loop {
        tui.clear()?;
        let repos = collect_repos(&root_path)?;
        let mut tui_branches: Vec<Vec<String>> = Vec::new();

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

            // let branches = repo.branches().iter().cloned();
            let mut branches = vec![repo.current_branch().to_string()];
            branches.extend(repo.branches().iter().filter(|f| f != &repo.current_branch()).cloned());

            let mut branch_vec: Vec<String> = Vec::new();

            for branch in branches {
                if branch == repo.current_branch() {
                    tui.cell_style = CellStyle::CurrentBranch;
                } else {
                    tui.cell_style = CellStyle::Branch;
                }
                tui.print(&limit_text(&branch, &MAX_BRANCH_NAME_WIDTH))?;
                branch_vec.push(branch.to_string());
            }
            tui.new_line()?;
            tui_branches.push(branch_vec);
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
            }

            // Action
            if event == Event::Key(KeyCode::Enter.into()) {
                // let branch_name = repos[tui.selected_coord().get_row() as usize].branches()[(tui.selected_coord().get_column() - 2) as usize].to_string();
                let branch_name = tui_branches[tui.selected_coord().get_row() as usize][tui.selected_coord().get_column() as usize - 2].to_string();
                let mut index = 999;
                for (i, name) in repos[tui.selected_coord().get_row() as usize].branches().iter().enumerate() {
                    if name == &branch_name {
                        index = i;
                    }
                }

                if index == 999 {
                    break;
                }

                match tui.selected_coord().get_column().to_column() {
                    Column::Branches => {
                        // let branch_index = (tui.selected_coord().get_column() - 2) as usize;
                        repos[tui.selected_coord().get_row() as usize].checkout_branch(index)?;
                    },
                    _ => {},
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
