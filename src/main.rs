use std::io::Write;
use termion::raw::IntoRawMode;
use termion::color;
use termion::input::TermRead;
use termion::event::Key;

const NAME_COLUMN_WIDTH: u16 = 20;
const STATUS_COLUMN_WIDTH: u16 = 5;

struct Repo {
    name: String,
    status: String,
    branches: Vec<String>,
}

struct Coord {
    column: u16,
    column_id: u16,
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
            column_id: 0,
            row: 0,
            current_column: 0,
            current_row: 0,
            first_branch: true,
        }
    }

    fn inc_row(&mut self) {
        self.row += 1;
    }

    fn go_up(&mut self) {
        if self.current_row > 0 {
            self.current_row -= 1;
        }
    }

    fn go_down(&mut self) {
        self.current_row += 1;
    }

    fn go_right(&mut self) {
        self.current_column += 1;
    }

    fn go_left(&mut self) {
        if self.current_column > 0 {
            self.current_column -= 1;
        }
    }

    fn start_rows(&mut self) {
        self.row = 0
    }

    fn name_column(&mut self) -> u16 {
        self.first_branch = true;
        self.column_id = 0;
        self.column = 0;
        self.column + 1
    }

    fn status_column(&mut self) -> u16 {
        self.column_id = 1;
        self.column += NAME_COLUMN_WIDTH;
        self.column + 1
    }

    fn branch_column(&mut self, branch_name: &str) -> u16 {
        self.column_id += 1;
        let gap = 1;
        if self.first_branch {
            self.column += STATUS_COLUMN_WIDTH + gap;
            self.first_branch = false;
        } else {
            self.column += branch_name.len() as u16 + gap;
        }
        self.column + 1
    }

    fn is_current_line(&self) -> bool {
        self.row == self.current_row
    }

    fn is_current_cell(&self) -> bool {
        self.column_id == self.current_column && self.is_current_line()
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

    let current_cell_color = color::Bg(color::Rgb(55, 55, 55));

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;
    let mut coord = Coord::new();

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        coord.start_rows();

        for repo in &repos {
            write!(stdout, "{}", termion::cursor::Goto(coord.name_column(), coord.row + 1)).unwrap();
            {
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.name, w=NAME_COLUMN_WIDTH as usize).unwrap();
                if coord.is_current_line() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }
            write!(stdout, "{}", termion::cursor::Goto(coord.status_column(), coord.row + 1)).unwrap();
            {
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.status, w=STATUS_COLUMN_WIDTH as usize).unwrap();
                if coord.is_current_line() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            let mut previous_branch = "";

            for branch in &repo.branches {
                write!(stdout, "{}", termion::cursor::Goto(coord.branch_column(previous_branch), coord.row + 1)).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{}", branch).unwrap();
                if coord.is_current_line() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
                previous_branch = branch;
            }
            coord.inc_row();
        }

        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    keep_running = false;
                    break;
                },
                Key::Right | Key::Char('l') => {
                    coord.go_right();
                    break;
                },
                Key::Left | Key::Char('h') => {
                    coord.go_left();
                    break;
                },
                Key::Up | Key::Char('k') => {
                    coord.go_up();
                    break;
                },
                Key::Down | Key::Char('j') => {
                    coord.go_down();
                    break;
                },
                _ => {}
            }
        }

    }
}
