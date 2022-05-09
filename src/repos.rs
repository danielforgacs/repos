use crate::prelude::*;

pub fn run(root_path: PathBuf) -> ReposError<()> {
    enable_raw_mode()?;
    let mut tui = Tui::new();
    loop {
        if poll(Duration::from_secs_f32(UPDATE_DELAY_SECS))? {
            let event = read()?;
            println!("Event::{:?}\r", event);
            if event == Event::Key(KeyCode::Up.into()) {
            }
            if event == Event::Key(KeyCode::Down.into()) {
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            tui.clear();
            let mut repos = collect_repos(&root_path)?;
            for repo in repos {
                println!("{:<20}::{:<25}::{:<15}::{:<50}\r",
                    repo.get_name(),
                    repo.get_current_branch(),
                    repo.get_status(),
                    repo.get_branches().join(" ")
                )
            };
        }
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
