use std::io::Write;
use termion::raw::IntoRawMode;

struct Repo {
    name: String,
    status: String,
    branches: Vec<String>,
}

struct Coord {
    column: u16,
    row: u16,
    current_column: u16,
    current_row: u16,
}

impl Repo {
    fn new(name: &str, status: &str, branches: Vec<&str>) -> Self {
        let branches = branches.iter().map(|x| x.to_string()).collect();
        Self { name: name.to_string(), status: status.to_string(), branches, }
    }
}

impl Coord {
    fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            current_column: 0,
            current_row: 0,
        }
    }

    fn inc_row(&mut self) {
        self.row += 1;
    }
}

fn main() {
    let repos = {
        let repo1 = Repo::new("alpha", "[   ]", vec!["master"]);
        let repo2 = Repo::new("beta",  "[   ]", vec!["master", "dev"]);
        let repo3 = Repo::new("gamma", "[   ]", vec!["master", "hotfix"]);
        let repo4 = Repo::new("delta", "[   ]", vec!["master", "hotfix", "dev", "feature"]);
        let repos = vec![repo1, repo2, repo3, repo4];
        repos
    };
    let mut coord = Coord::new();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for repo in &repos {
            write!(stdout, "{}", termion::cursor::Goto(coord.column + 1, coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.name).unwrap();
            write!(stdout, "{}", termion::cursor::Goto(coord.column + 1 + 20, coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.status).unwrap();
            coord.inc_row();
        }
        keep_running = false;
    }
}
