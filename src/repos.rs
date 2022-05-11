use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposError<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();

    loop {
        eprintln!("::{:?}", std::time::Instant::now());
        eprintln!("{:#?}", tui);
        let mut repos = collect_repos(&root_path)?;
        tui.set_row_count(repos.len() as u16);
        tui.clear();

        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            println!("Event::{:?}\r", event);
            if event == Event::Key(KeyCode::Up.into()) {
                tui.go(Direction::Up);
            }
            if event == Event::Key(KeyCode::Down.into()) {
                tui.go(Direction::Down);
            }
            if event == Event::Key(KeyCode::Char('q') .into()) {
                break;
            }
        } else {
            for repo in repos {
                tui.print(&repo.get_name());
                tui.new_line();
            };
        }

        tui.flush();
    }

    disable_raw_mode()?;
    Ok(())
}

fn collect_repos(path: &Path) -> ReposError<Vec<Repo>> {
    let mut repos: Vec<Repo> = Vec::new();
    for dir in find_git_repos_in_dir(&path)? {
        repos.push(
            Repo::new(&dir)?
        )
    }
    Ok(repos)
}
