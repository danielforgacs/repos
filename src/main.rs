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

    fn start_rows(&mut self) {
        self.row = 0
    }

    fn name_column(&mut self) -> u16 {
        self.first_branch = true;
        self.column = 0;
        self.column + 1
    }

    fn status_column(&mut self) -> u16 {
        let name_column_with: u16 = 15;
        self.column = name_column_with;
        self.column + 1
    }

    fn branch_column(&mut self, branch_name: &str) -> u16 {
        let status_column_width: u16 = 5;
        let gap = 1;
        if self.first_branch {
            self.column += status_column_width + gap;
            self.first_branch = false;
        } else {
            self.column += branch_name.len() as u16 + gap;
            // self.column += 15;
        }
        self.column + 1
    }
}

fn main() {
    let repos = {
        let repo1 = Repo::new("alpha", "12345", vec!["master"]);
        let repo2 = Repo::new("beta",  "[   ]", vec!["master", "dev"]);
        let repo3 = Repo::new("gamma", "[   ]", vec!["master", "hotfix"]);
        let repo4 = Repo::new("delta", "[   ]", vec!["master", "hotfix", "dev", "feature"]);
        let repo5 = Repo::new("0123456789", "[01234]", vec!["0123456789", "0123456789", "0123456789", "0123456789"]);
        let repo6 = Repo::new("abcdefghijklm", "ABCDEFGHIJK", vec!["abcdefghijklm", "ABCDEFGHIJK", "abcdefghijklm", "ABCDEFGHIJK", "abcdefghijklm"]);
        let repos = vec![repo1, repo2, repo3, repo4, repo5, repo6];
        repos
    };
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;
    let mut coord = Coord::new();

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        coord.start_rows();

        for repo in &repos {
            write!(stdout, "{}", termion::cursor::Goto(coord.name_column(), coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.name).unwrap();
            write!(stdout, "{}", termion::cursor::Goto(coord.status_column(), coord.row + 1)).unwrap();
            write!(stdout, "{}", repo.status).unwrap();

            // let mut previous_branch = String::new();
            let mut previous_branch = "";
            for branch in &repo.branches {
                write!(stdout, "{}", termion::cursor::Goto(coord.branch_column(previous_branch), coord.row + 1)).unwrap();
                write!(stdout, "{}", branch).unwrap();
                previous_branch = branch;
            }
            coord.inc_row();
        }
        keep_running = false;
    }
}
