use std::io::Write;
use std::path::PathBuf;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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
    fn new(path: PathBuf, status: &str) -> Self {
        let branches = Vec::new();
        let name = path
            .file_name()
            .expect("can't get repo name from path")
            .to_str()
            .unwrap()
            .to_string();
        let mut repo = Self {
            name,
            status: status.to_string(),
            branches,
            path,
        };
        repo.update_branches();
        repo
    }

    fn update_branches(&mut self) {
        let mut branches: Vec<String> = Vec::new();
        let output = std::process::Command::new("git")
            .arg("branch")
            .current_dir(&self.path)
            .output()
            .expect("Could not get branches");
        if !output.status.success() {
            branches.push("(no branch)".to_string());
        }
        let mut git_output: Vec<String> = String::from_utf8(output.stdout)
            .expect("can't extract git output.")
            .lines()
            .map(|x| x[2..].to_string())
            .collect();
        git_output.sort_by_key(|a| a.to_lowercase());
        self.branches = git_output;
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
        self.column_id == self.current_column_id + 1 && self.row == self.current_row
    }
}

fn goto(x: u16, y: u16) -> termion::cursor::Goto {
    termion::cursor::Goto(x + 1, y + 1)
}

fn main() {
    let dev_dir = get_dev_dir();
    let repo_paths = find_repo_dirs(dev_dir);
    let repos: Vec<Repo> = repo_paths
        .iter()
        .map(|path| Repo::new(path.to_path_buf(), "status"))
        .collect();
    tui(repos);
}

fn get_dev_dir() -> PathBuf {
    match std::env::var("DEVDIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => std::env::current_dir().unwrap(),
    }
}

fn find_repo_dirs(root: PathBuf) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = Vec::new();

    if let Ok(read_dir) = root.read_dir() {
        for dir in read_dir {
            if dir.as_ref().expect("msg").path().join(".git").is_dir() {
                repos.push(dir.unwrap().path().to_path_buf())
            }
        }
    }
    repos.sort_by(|a, b| {
        a.to_str()
            .unwrap()
            .to_lowercase()
            .cmp(&b.to_str().unwrap().to_lowercase())
    });
    repos
}

fn tui(mut repos: Vec<Repo>) {
    let current_cell_color = color::Bg(color::Rgb(75, 30, 15));
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;
    let mut coord = Tui::new();
    let repo_count = repos.len();

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        coord.reset();
        coord.row_count = repo_count;

        for repo in repos.iter_mut() {
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

        for repo in repos.iter_mut() {
            repo.update_branches();
        }

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    keep_running = false;
                    break;
                }
                Key::Right | Key::Char('l') => {
                    coord.go_right();
                    break;
                }
                Key::Left | Key::Char('h') => {
                    coord.go_left();
                    break;
                }
                Key::Up | Key::Char('k') => {
                    coord.go_up();
                    break;
                }
                Key::Down | Key::Char('j') => {
                    coord.go_down();
                    break;
                }
                _ => {}
            }
        }
    }
}
