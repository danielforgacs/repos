use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposError<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();

    loop {
        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Up.into()) {
                tui.go(Direction::Up);
            }
            if event == Event::Key(KeyCode::Down.into()) {
                tui.go(Direction::Down);
            }
            if event == Event::Key(KeyCode::Left.into()) {
                tui.go(Direction::Left);
            }
            if event == Event::Key(KeyCode::Right.into()) {
                tui.go(Direction::Right);
            }
            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }

        let repos = collect_repos(&root_path)?;
        tui.set_row_count(repos.len() as u16);
        tui.clear()?;

        for repo in repos {
            tui.print(&repo.get_name())?;
            tui.print(&format!("{}", repo.get_status()))?;
            tui.new_line()?;
        }

        tui.flush()?;
    }

    disable_raw_mode()?;
    Ok(())
}

fn collect_repos(path: &Path) -> ReposError<Vec<Repo>> {
    let mut repos: Vec<Repo> = Vec::new();
    for dir in find_git_repos_in_dir(path)? {
        repos.push(Repo::new(&dir)?)
    }
    Ok(repos)
}
