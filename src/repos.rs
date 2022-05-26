use crate::prelude::*;

enum RepoSort {
    Alpha,
    Status,
    CurrentBranch,
}

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    tui.print(&format!("{}", crossterm::cursor::Hide))?;
    let mut repo_sort = RepoSort::Alpha;
    let mut prev_sel_repo: Option<String> = None;
    let mut prev_sel_branch: Option<String> = None;

    loop {
        tui.clear()?;
        let repos = match repo_sort {
            RepoSort::Alpha => {
                let mut new_repos = collect_repos(&root_path)?;
                new_repos.sort_by_key(|f| f.name().to_string());
                new_repos
            },
            RepoSort::Status => {
                let mut new_repos = collect_repos(&root_path)?;
                new_repos.sort_by_key(|f| f.status().to_string());
                new_repos
            },
            RepoSort::CurrentBranch => {
                let mut new_repos = collect_repos(&root_path)?;
                new_repos.sort_by_key(|f| f.current_branch().to_string());
                new_repos
            },
        };
        let mut tui_branches: Vec<Vec<String>> = Vec::new();
        if let Some(ref branch) = prev_sel_repo {
            for (index, repo) in repos.iter().enumerate() {
                if repo.name() == branch {
                    tui.set_selected_row(index as u16);
                    break;
                }
            }
            // prev_sel_repo = None;
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

            let branches = match repo_sort {
                RepoSort::CurrentBranch => {
                    let mut branches = vec![repo.current_branch().to_string()];
                    branches.extend(repo.branches().iter().filter(|f| f != &repo.current_branch()).cloned());
                    branches
                },
                _ => repo.branches().to_owned(),
            };

            // if let Some(name) = prev_sel_branch {
            //     eprintln!("looking for branch: {}, all repo branches: {:?}", &name, &branches);
            //     for (index, branch) in branches.iter().enumerate() {
            //         eprintln!("\t...checking: {}", branch);
            //         if branch == &name {
            //             eprintln!("found index: {}", &index);
            //             tui.set_selected_column((index + 2) as u16);
            //             break;
            //         }
            //     }
            //     prev_sel_branch = None;
            // }

            let mut branch_vec: Vec<String> = Vec::new();

            for (index, branch) in branches.iter().enumerate() {
                match &prev_sel_repo {
                    Some(reponame) => {
                        if reponame == repo.name() {

                            if let Some(name) = &prev_sel_branch {
                                eprintln!("looking for {} == {}", &name, &branch);
                                if name == branch {
                                    tui.set_selected_column(index as u16 + 2);
                                    // prev_sel_branch = None;
                                }
                            };
                        }

                    },
                    _ => {},
                }
                if branch == repo.current_branch() {
                    tui.cell_style = CellStyle::CurrentBranch;
                } else {
                    tui.cell_style = CellStyle::Branch;
                }
                tui.print(&limit_text(&branch, &MAX_BRANCH_NAME_WIDTH))?;
                branch_vec.push(branch.to_string());
            }
            prev_sel_branch = None;
            tui.new_line()?;
            tui_branches.push(branch_vec);
        }

        tui.flush()?;

        prev_sel_repo = None;
        prev_sel_branch = None;

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
                prev_sel_repo = Some(repos[tui.selected_coord().get_row() as usize].name().to_string());
                if tui.selected_coord().get_column() > 1 {
                    prev_sel_branch = Some(tui_branches[tui.selected_coord().get_row() as usize][(tui.selected_coord().get_column() - 2) as usize].to_string());
                    eprintln!("\n\n---\nstoring branch index: {}, name: {:?}", &tui.selected_coord().get_column() - 2, &prev_sel_branch);
                }
                repo_sort = match repo_sort {
                    RepoSort::Alpha => RepoSort::Status,
                    RepoSort::Status => RepoSort::CurrentBranch,
                    RepoSort::CurrentBranch => RepoSort::Alpha,
                }
            }

            // Action/tmp/tmp.LccouINZ9I__repos_test/
            if event == Event::Key(KeyCode::Enter.into()) {
                match tui.selected_coord().get_column().to_column() {
                    Column::Branches => {
                        let branch_name = tui_branches[tui.selected_coord().get_row() as usize][tui.selected_coord().get_column() as usize - 2].to_string();
                        repos[tui.selected_coord().get_row() as usize].checkout_branch(&branch_name)?;
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
