use std::io::Write;
use std::path::{PathBuf};
use termion::raw::IntoRawMode;
use termion::color;
use termion::input::TermRead;
use termion::event::Key;

struct Repo {
    name: String,
    path: PathBuf,
    status: String,
    branches: Vec<String>,
}

struct Tui {
    column: u16,
    column_id: u16,
    current_column_id: u16,
    row_column_counts: Vec<u16>,
    row: u16,
    current_row: u16,
    row_count: usize,
}

impl Repo {
    fn new(path: PathBuf, name: &str, status: &str, branches: Vec<&str>) -> Self {
        let branches = branches.iter().map(|x| x.to_string()).collect();
        let name = path.file_name().expect("can't get repo name from path").to_str().unwrap().to_string();
        Self {
            name: name.to_string(),
            status: status.to_string(),
            branches,
            path,
        }
    }
}

impl Tui {
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

    fn column(&mut self, width: Option<usize>) -> u16 {
        let width = match width {
            Some(w) => w as u16 + 1,
            Option::None => self.column_width() as u16,
        };
        let width = match self.column_id {
            0 => 0,
            _ => width,
        };
        self.column += width;
        self.column_id += 1;
        self.column
    }

    fn column_width(&self) -> usize {
        25
    }

    fn is_current_cell(&self) -> bool {
        self.column_id == self.current_column_id + 1&& self.row == self.current_row
    }
}

fn goto(x: u16, y: u16) -> termion::cursor::Goto {
    termion::cursor::Goto(x + 1, y + 1)
}

fn main() {
    let dev_dir = get_dev_dir();
    let repo_paths = find_repo_dirs(dev_dir);
    let repos: Vec<Repo> = repo_paths.iter().map(|path| Repo::new(path.to_path_buf(), "name", "status", vec!["master"])).collect();
    tui(repos);
}

fn get_dev_dir() -> PathBuf {
    let dev_path = match std::env::var("DEVDIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => PathBuf::from(std::env::current_dir().unwrap()),
    };
    dev_path
}

fn find_repo_dirs(root: PathBuf) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = Vec::new();

    for read_dir in root.read_dir() {
        for dir in read_dir {
            if dir.as_ref().expect("msg").path().join(".git").is_dir() {
                repos.push(dir.unwrap().path().to_path_buf())
            }
        }
    }
    repos
}

fn tui(repos: Vec<Repo>) {
    // let repos = {
    //     let path = PathBuf::from("");
    //     let repo1 = Repo::new(path.to_path_buf(), "alpha", "12345", vec!["master"]);
    //     let repo2 = Repo::new(path.to_path_buf(), "beta",  "[   ]", vec!["master", "dev"]);
    //     let repo3 = Repo::new(path.to_path_buf(), "gamma", "[   ]", vec!["master", "hotfix"]);
    //     let repo4 = Repo::new(path.to_path_buf(), "delta", "[   ]", vec!["master", "hotfix", "dev", "feature"]);
    //     let repo5 = Repo::new(path.to_path_buf(), "0123456789", "[01234]", vec!["0123456789", "0123456789", "0123456789", "0123456789"]);
    //     let repo6 = Repo::new(path.to_path_buf(), "abcdefghijklm", "ABCDEFGHIJK", vec!["abcdefghijklm", "ABCDEFGHIJK", "abcdefghijklm", "ABCDEFGHIJK", "abcdefghijklm"]);
    //     let repos = vec![repo1, repo2, repo3, repo4, repo5, repo6];
    //     repos
    // };

    let current_cell_color = color::Bg(color::Rgb(75, 30,15));
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;
    let mut coord = Tui::new();

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        coord.reset();
        coord.row_count = repos.len();

        for repo in &repos {
            coord.row_column_counts.push(repo.branches.len() as u16 + 2);

            {
                write!(stdout, "{}", goto(coord.column(Option::None), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.name, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            {
                write!(stdout, "{}", goto(coord.column(Option::None), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", repo.status, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
            }

            let mut previous_branch_width: Option<usize> = Option::None;

            for branch in &repo.branches {
                write!(stdout, "{}", goto(coord.column(previous_branch_width), coord.row())).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", current_cell_color).unwrap(); }
                write!(stdout, "{:w$}", branch, w=coord.column_width()).unwrap();
                if coord.is_current_cell() { write!(stdout, "{}", color::Bg(color::Reset)).unwrap(); }
                previous_branch_width = Some(branch.len());
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
