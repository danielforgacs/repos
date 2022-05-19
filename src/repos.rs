use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();

    loop {
        let repos = collect_repos(&root_path)?;
        tui.clear()?;

        for repo in repos {
            tui.print(&limit_string(repo.name(), &(REPO_NAME_WIDTH as usize)))?;
            tui.print(&format!("{}", repo.get_status()))?;
            let current_branch = repo.get_current_branch();
            for branch in repo.get_branches() {
                if branch == current_branch {
                    tui.print_current_branch(&branch)?;
                } else {
                    tui.print(&branch)?;
                }
            }
            tui.new_line()?;
        }

        tui.flush()?;

        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
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
            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }
    }

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
