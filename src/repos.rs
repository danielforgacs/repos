use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposResult<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();

    loop {
        let repos = collect_repos(&root_path)?;
        tui.clear()?;

        for repo in repos {
            tui.print(&text_to_width(repo.name(), &(REPO_NAME_WIDTH as usize)))?;
            tui.print(&format!("{}", repo.status()))?;
            for branch in repo.branches() {
                if branch == repo.current_branch() {
                    // TODO: add current branch style here
                    tui.print(&limit_text(branch, &MAX_BRANCH_NAME_WIDTH))?;
                } else {
                    tui.print(&limit_text(branch, &MAX_BRANCH_NAME_WIDTH))?;
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
