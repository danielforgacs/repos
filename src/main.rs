use std::io::Write;
use termion::raw::IntoRawMode;
use termion::color;
use termion::input::TermRead;
use termion::event::Key;

struct Repo {
    name: String,
    status: String,
    branches: Vec<String>,
}

struct Coord {
    column: u16,
    column_id: u16,
    current_column_id: u16,
    row_column_counts: Vec<u16>,
    row: u16,
    current_row: u16,
    row_count: usize,
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
            current_column_id: 0,
            row_column_counts: Vec::new(),
            row: 0,
            current_row: 0,
            row_count: 0,
        }
    }

    fn reset(&mut self) {
        self.row = 0;
        self.column = 0;
    }

    fn row(&self) -> u16 {
        self.row
    }

    fn finished_row(&mut self) {
        self.column = 0;
        self.column_id = 0;
        self.row += 1;
    }

    fn go_up(&mut self) {
        if self.current_row > 0 {
            self.current_row -= 1;
        }
        self.validate_current_column()
    }

    fn go_down(&mut self) {
        if self.current_row < self.row_count as u16 - 1 {
            self.current_row += 1;
        }
        self.validate_current_column()
    }

    fn go_right(&mut self) {
        self.current_column_id += 1;
        self.validate_current_column()
    }

    fn go_left(&mut self) {
        if self.current_column_id > 0 {
            self.current_column_id -= 1;
        }
    }

    fn validate_current_column(&mut self) {
        if self.current_column_id > self.row_column_counts[self.current_row as usize] - 1 {
            self.current_column_id = self.row_column_counts[self.current_row as usize] - 1
        }
    }

    fn column(&mut self) -> u16 {
        let value = self.column;
        self.column += 15;
        self.column_id += 1;
        value
    }

    fn column_width(&self) -> usize {
        15
    }

    fn is_current_cell(&self) -> bool {
        self.column_id == self.current_column_id + 1&& self.row == self.current_row
    }
}

fn goto(x: u16, y: u16) -> termion::cursor::Goto {
    termion::cursor::Goto(x + 1, y + 1)
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
        coord.reset();
        coord.row_count = repos.len();

        for repo in &repos {
            coord.row_column_counts.push(repo.branches.len() as u16 + 2);

            {
                write!(stdout, "{}", goto(coord.column(), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.name, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            {
                write!(stdout, "{}", goto(coord.column(), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.status, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            for branch in &repo.branches {
                write!(stdout, "{}", goto(coord.column(), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", branch, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            coord.finished_row();
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
