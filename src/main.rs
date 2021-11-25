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
    first_branch: bool,
}

impl Repo {
    fn new(name: &str, status: &str, branches: Vec<&str>) -> Self {
        let branches = branches.iter().map(|x| x.to_string()).collect();
        Self {
            name: name.to_string(),
            status: status.to_string(),
            branches,
        }
    }
}

impl Coord {
    fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            current_column: 0,
            current_row: 0,
            first_branch: true,
        }
    }

    fn inc_row(&mut self) {
        self.row += 1;
    }

    fn name_column(&mut self) -> u16 {
        self.first_branch = true;
        self.column = 0;
        self.column + 1
    }

    fn status_column(&mut self) -> u16 {
        self.column = 20;
        self.column + 1
    }

    fn branch_column(&mut self) -> u16 {
        let status_width: u16 = 5;
        if self.first_branch {
            self.column += status_width;
            self.column += 2;
            self.first_branch = false;
        } else {
            self.column += 10;
        }
        self.column
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
        coord.row = 0;

        for repo in &repos {
            write!(stdout, "{}", termion::cursor::Goto(coord.name_column(), coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.name).unwrap();
            write!(stdout, "{}", termion::cursor::Goto(coord.status_column(), coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.status).unwrap();
            for branch in &repo.branches {
                write!(stdout, "{}", termion::cursor::Goto(coord.branch_column(), coord.row + 1)).unwrap();
                write!(stdout, "{}", branch).unwrap();
            }
            coord.inc_row();
        }
        keep_running = false;
    }
}
