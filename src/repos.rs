use crate::prelude::*;

#[derive(PartialEq)]
pub enum RepoSort {
    Alpha,
    Status,
    CurrentBranch,
}

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    tui.print(&format!("{}", crossterm::cursor::Hide))?;
    let mut repo_sort = RepoSort::Alpha;

    loop {
        tui.clear()?;
        let repos = collect_repos(&root_path, &repo_sort)?;
        tui.set_max_selected_column(repos[tui.selected_coord().get_row() as usize].branches().len() as u16 + 1);

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

            for branch in repo.branches() {
                if branch == repo.current_branch() {
                    tui.cell_style = CellStyle::CurrentBranch;
                } else {
                    tui.cell_style = CellStyle::Branch;
                }
                tui.print(&limit_text(&branch, &MAX_BRANCH_NAME_WIDTH))?;
            }
            tui.new_line()?;
        }

        let sel_cell_branch = if tui.selected_coord().get_column().to_column() == Column::Branches {
            repos[tui.selected_coord().get_row() as usize].branches()[tui.selected_coord().get_column() as usize - 2].as_str()
        } else {
            ""
        };

        tui.print_status(
            &repos[tui.selected_coord().get_row() as usize].name(),
            &repos[tui.selected_coord().get_row() as usize].current_branch(),
            sel_cell_branch,
        )?;

        tui.flush()?;

        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;

            // quit.
            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            } else {
                on_keypress_action(&event, &mut tui, &repos, &mut repo_sort)?;
            };
        }
    }

    tui.print(&format!("{}", crossterm::cursor::Show))?;
    disable_raw_mode()?;
    Ok(())
}

fn on_keypress_action(event: &Event, tui: &mut Tui, repos: &Vec<Repo>, repo_sort: &mut RepoSort) -> ReposResult<()>{
    // Navogation.
    if *event == Event::Key(KeyCode::Up.into()) || *event == Event::Key(KeyCode::Char('k').into()) {
        tui.go(Direction::Up);
    }
    if *event == Event::Key(KeyCode::Down.into()) || *event == Event::Key(KeyCode::Char('j').into()) {
        tui.go(Direction::Down);
    }
    if *event == Event::Key(KeyCode::Left.into()) || *event == Event::Key(KeyCode::Char('h').into()) {
        tui.go(Direction::Left);
    }
    if *event == Event::Key(KeyCode::Right.into()) || *event == Event::Key(KeyCode::Char('l').into()) {
        tui.go(Direction::Right);
    }

    //Sorting.
    if *event == Event::Key(KeyCode::Char('s').into()) {
        *repo_sort = match repo_sort {
            RepoSort::Alpha => RepoSort::Status,
            RepoSort::Status => RepoSort::CurrentBranch,
            RepoSort::CurrentBranch => RepoSort::Alpha,
        }
    }

    // Action
    if *event == Event::Key(KeyCode::Enter.into()) {
        match tui.selected_coord().get_column().to_column() {
            Column::Branches => {
                let branch_index = tui.selected_coord().get_column() as usize - 2;
                let repo = &repos[tui.selected_coord().get_row() as usize];
                let branch = repo.branches()[branch_index].to_string();
                repo.checkout_branch(branch)?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn collect_repos(path: &Path, sort: &RepoSort) -> ReposResult<Vec<Repo>> {
    let mut repos: Vec<Repo> = Vec::new();
    for dir in find_git_repos_in_dir(path)? {
        repos.push(Repo::new(&dir)?)
    }
    match sort {
        RepoSort::Alpha => repos.sort_by_key(|r| r.name().to_string()),
        RepoSort::Status => repos.sort_by_key(|r| r.status().to_string()),
        RepoSort::CurrentBranch => repos.sort_by_key(|r| r.current_branch().to_string()),
    }
    if *sort == RepoSort::CurrentBranch {
        repos.iter_mut().for_each(|repo| repo.set_current_branch_as_first());
    } else {
        repos.iter_mut().for_each(|repo| repo.sort_branches());
    }
    Ok(repos)
}
