use std::io::Write;
use std::path::PathBuf;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

enum RepoState {
    MasterOk,
    MasterNotOk,
    NotMasterOK,
    NotMasterNotOK,
}

struct Repo {
    name: String,
    path: PathBuf,
    status: RepoStatus,
    branches: Vec<String>,
    current_branch: String,
}
#[derive(Debug)]
struct RepoStatus {
    untracked: bool,
    deleted: bool,
    deleted_staged: bool,
    staged: bool,
    modified: bool,
    new_file: bool,
    new_file_2: bool,
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

impl RepoStatus {
    fn new() -> Self {
        Self {
            untracked: false,
            deleted: false,
            deleted_staged: false,
            staged: false,
            modified: false,
            new_file: false,
            new_file_2: false,
        }
    }

    fn is_ok(&self) -> bool {
        let has_bad_stuff = self.untracked ||
            self.deleted ||
            self.deleted_staged ||
            self.staged ||
            self.modified ||
            self.new_file ||
            self.new_file_2;
        !has_bad_stuff
    }
}

impl ToString for RepoStatus {
    fn to_string(&self) -> String {
        let empty_status = " ";
        let status_text = format!("{}{}{}{}{}{}{}",
            if self.untracked { "U" } else { empty_status },
            if self.deleted { "D" } else { empty_status },
            if self.deleted_staged { "d" } else { empty_status },
            if self.staged { "S" } else { empty_status },
            if self.modified { "M" } else { empty_status },
            if self.new_file { "N" } else { empty_status },
            if self.new_file_2 { "n" } else { empty_status },
        );
        status_text
    }
}

impl Repo {
    fn new(path: PathBuf) -> Self {
        let branches = Vec::new();
        let name = path
            .file_name()
            .expect("can't get repo name from path")
            .to_str()
            .unwrap()
            .to_string();
        let mut repo = Self {
            name,
            status: RepoStatus::new(),
            branches,
            path,
            current_branch: String::new(),
        };
        repo.update();
        repo
    }

    fn update(&mut self) {
        self.update_branches();
        self.update_status();
        self.update_current_branch();
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

    fn update_current_branch(&mut self) {
        let branch = std::process::Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .current_dir(&self.path)
            .output()
            .expect("can't get current branch");
        self.current_branch = String::from_utf8(branch.stdout)
            .expect("can't convert branch name")
            .as_str()
            .trim()
            .to_string();
    }

    fn update_status(&mut self) {
        let status_mark_width = 2;
        let output = std::process::Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .current_dir(&self.path)
            .output()
            .expect("can't get status.");
        for line in String::from_utf8(output.stdout)
            .expect("can't get status output")
            .lines() {
                match &line[..status_mark_width] {
                    "??" => self.status.untracked = true,
                    " D" => self.status.deleted = true,
                    "D " => self.status.deleted_staged = true,
                    "M " => self.status.staged = true,
                    " M" => self.status.modified = true,
                    "A " => self.status.new_file = true,
                    "AM" => self.status.new_file_2 = true,
                    _ => (),
                };
            }
    }

    fn is_clean(&self) -> bool {
        self.current_branch == "master".to_string() && self.status.is_ok()
    }

    fn get_repo_state(&self) -> RepoState {
        match self.current_branch.as_ref() {
            "master" => {
                match self.status.is_ok() {
                    true => return RepoState::MasterOk,
                    false => return RepoState::MasterNotOk,
                }
            },
            _ => {
                match self.status.is_ok() {
                    true => return RepoState::NotMasterOK,
                    false => return RepoState::NotMasterNotOK,
                }
            },
        };
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

    fn column(&mut self) -> u16 {
        match self.column_id {
            0 => {},
            1 => self.column += 28,
            _ => self.column += 10,
        };
        self.column_id += 1;
        self.column
    }

    fn adjust_column_width(&mut self, width: u16) {
        self.column -= 10;
        self.column += width + 1;
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
        .map(|path| Repo::new(path.to_path_buf()))
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

    repos.sort_by_key(|x| x
        .to_str()
        .unwrap()
        .to_lowercase());
    repos
}

fn tui(mut repos: Vec<Repo>) {
    let bg_current_cell = color::Bg(color::Rgb(75, 30, 15));
    let bg_reset = color::Bg(color::Reset);

    let fg_master_ok = color::Fg(color::Rgb(0, 175, 0));
    let fg_master_not_ok = color::Fg(color::Rgb(255, 180, 0));
    let fg_not_master_ok = color::Fg(color::Rgb(0, 200, 255));
    let fg_not_master_not_ok = color::Fg(color::Rgb(225, 0, 0));

    let fg_active_branch = color::Fg(color::Rgb(35, 200, 35));
    let fg_inactive_branch = color::Fg(color::Rgb(90, 90, 90));

    let fg_reset = color::Fg(color::Reset);

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut keep_running = true;
    let mut tui = Tui::new();
    let repo_count = repos.len();

    while keep_running {
        write!(stdout, "{}", termion::clear::All).unwrap();
        tui.reset();
        tui.row_count = repo_count;

        for repo in repos.iter_mut() {
            tui.row_column_counts.push(repo.branches.len() as u16 + 2);

            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();
            {
                if tui.is_current_cell() { write!(stdout, "{}", bg_current_cell).unwrap(); }
                match repo.get_repo_state() {
                    RepoState::MasterOk => write!(stdout, "{}", fg_master_ok).unwrap(),
                    RepoState::MasterNotOk => write!(stdout, "{}", fg_master_not_ok).unwrap(),
                    RepoState::NotMasterOK => write!(stdout, "{}", fg_not_master_ok).unwrap(),
                    RepoState::NotMasterNotOK => write!(stdout, "{}", fg_not_master_not_ok).unwrap(),
                }
                write!(stdout, "{:w$}", repo.name, w=28).unwrap();
                write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
            }


            write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();
            {
                if tui.is_current_cell() { write!(stdout, "{}", bg_current_cell).unwrap(); }
                write!(stdout, "[{}]", repo.status.to_string()).unwrap();
                write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
            }

            for branch in &repo.branches {
                write!(stdout, "{}", goto(tui.column(), tui.row())).unwrap();

                {
                    if tui.is_current_cell() { write!(stdout, "{}", bg_current_cell).unwrap(); }
                    if branch == repo.current_branch.as_str() {
                        write!(stdout, "{}", fg_active_branch).unwrap();
                    } else {
                        write!(stdout, "{}", fg_inactive_branch).unwrap();
                    }
                    write!(stdout, "{}", branch).unwrap();
                    write!(stdout, "{}{}", bg_reset, fg_reset).unwrap();
                }

                tui.adjust_column_width(branch.len() as u16);
            }

            tui.finished_row();
        }

        let current_repo = &repos[tui.current_row as usize];

        write!(stdout, "{}", goto(5, &(repos.len() as u16) + 2)).unwrap();
        write!(stdout, "{}", current_repo.name).unwrap();

        stdout.flush().unwrap();

        // for mut repo in repos {
        for repo in repos.iter_mut() {
            repo.update();
        }

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    keep_running = false;
                    break;
                }
                Key::Right | Key::Char('l') => {
                    tui.go_right();
                    break;
                }
                Key::Left | Key::Char('h') => {
                    tui.go_left();
                    break;
                }
                Key::Up | Key::Char('k') => {
                    tui.go_up();
                    break;
                }
                Key::Down | Key::Char('j') => {
                    tui.go_down();
                    break;
                }
                Key::Char('\n') => {
                    let repo = &repos[tui.current_row as usize];
                    write!(stdout, "{}", goto(5, 26)).unwrap();
                    write!(stdout, "{}.{}", repo.name, tui.current_column_id).unwrap();
                    stdout.flush().unwrap();
                }
                _ => {}
            }
        }
    }
}
