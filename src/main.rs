use std::path::PathBuf;
use std::fs::{read_to_string};
use std::process::Command;
use structopt::StructOpt;
use termion::color;

const REPO_NAME_WIDTH: usize = 20;
const BRANCH_NAME_WIDTH: usize = 35;
const DEVDIR_ENV_VAR: &str = "DEVDIR";
const GIT_SUBDIR: &str = "/.git";
const GIT_HEAD_REL_PATH: &str = "/.git/HEAD";

struct DevDir {
    _path: PathBuf,
    repos: Vec<Repo>,
}

struct Repo {
    name: String,
    path: PathBuf,
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

#[derive(StructOpt)]
struct Opt {
    /// Set "DEVDIR" env var for easier use.
    #[structopt(parse(from_os_str), env = DEVDIR_ENV_VAR, default_value = ".")]
    path: PathBuf,
    /// Include repos with "master" branch and "Ok" status.
    #[structopt(short = "-a")]
    show_all: bool,
}

impl Opt {
    fn new() -> Self {
        let mut opt = Self::from_args();
        opt.path = std::fs::canonicalize(opt.path).unwrap();
        opt
    }
}

impl DevDir {
    fn new(devdir: PathBuf) -> Self {
        let mut repos: Vec<Repo> = Vec::new();

        for entry in devdir.read_dir().unwrap() {
            let entry = match entry {
                Ok(entry) => entry.path(),
                Err(_) => PathBuf::new(),
            };
            let entry_git = entry.to_str().unwrap().to_string() + GIT_SUBDIR;
            if !std::path::Path::new(&entry_git).is_dir() {
                continue;
            }
            let repo = Repo::new(entry);
            repos.push(repo);
        }

        repos.sort_by(|repo_a, repo_b| repo_a.name.to_lowercase().cmp(&repo_b.name.to_lowercase()));
        DevDir {
            _path: devdir,
            repos,
        }
    }
}

impl Repo {
    fn new(path: PathBuf) -> Self {
        let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
        if name.len() > REPO_NAME_WIDTH {
            name = String::from(&name[..REPO_NAME_WIDTH - 1]);
            name += "~";
        }
        Self { name, path }
    }

    fn branch(&self) -> String {
        let mut head_file = PathBuf::new();
        head_file.push(self.path.to_str().unwrap().to_string() + GIT_HEAD_REL_PATH);
        let githead: String = read_to_string(&head_file).unwrap();
        let githead = githead.trim().to_string();
        let mut branch = githead.split("/").last().unwrap().to_string();
        if branch.len() > BRANCH_NAME_WIDTH {
            branch = branch[..BRANCH_NAME_WIDTH - 1].to_string();
            branch += "~";
        }
        branch
    }

    fn status(&self) -> RepoStatus {
        let status_stdout = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .current_dir(&self.path)
            .output()
            .unwrap()
            .stdout;
        let status_stdout = String::from_utf8(status_stdout).unwrap();
        let mut status = RepoStatus::new();
        let status_mark_width = 2;

        for line in status_stdout.lines() {
            match &line[..status_mark_width] {
                "??" => status.untracked = true,
                " D" => status.deleted = true,
                "D " => status.deleted_staged = true,
                "M " => status.staged = true,
                " M" => status.modified = true,
                "A " => status.new_file = true,
                "AM" => status.new_file_2 = true,
                _ => (),
            };
        }
        status
    }
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

fn main() {
    let opt = Opt::new();
    if !opt.path.is_dir() {
        println!("Bad path: \"{}\"!\nWhat a bimbo...?!??! How are you even a programmer? ;)", opt.path.as_path().display());
        return
    }
    check_repos(opt);
}

fn check_repos(opt: Opt) {
    let color_info = format!("{}", color::Fg(color::Rgb(75, 75, 75)));
    let color_ok = format!("{}", color::Fg(color::Green));
    let color_bad_status = format!("{}", color::Fg(color::Rgb(200, 80, 0)));
    let color_reset = format!("{}", color::Fg(color::Reset));
    print!("{}{}{}", color_info, opt.path.as_path().display(), color_reset);
    let devdir = DevDir::new(opt.path);
    let mut print_text = "".to_string();
    let header = format!("\n{}{:>re$} |{:^st$}| {:br$}{}",
        color_info,
        "<------- Repo",
        "Status",
        "Branch ------->",
        color_reset,
        re=REPO_NAME_WIDTH,
        st=7,
        br=BRANCH_NAME_WIDTH);
    print_text.push_str(&header);

    for repo in devdir.repos {
        let branch = repo.branch();
        let is_branch_master = branch == "master";
        let status = repo.status();
        if is_branch_master && repo.status().is_ok() {
            if !opt.show_all {
                continue;
            }
            print_text += &color_ok;
        }
        if !repo.status().is_ok() {
            print_text += &color_bad_status;
        }
        let branch_txt = if is_branch_master { "".to_string() } else { branch };
        print_text += format!("\n{:>rw$} [{}] {:bw$}",
            repo.name,
            status.to_string(),
            branch_txt,
            rw=REPO_NAME_WIDTH,
            bw=BRANCH_NAME_WIDTH).as_str();
        print_text += &color_reset;
    }
    print_text += &color_info;
    print_text += "\nU: untracked, D: deleted, d: deleted staged, S: staged\
    \nM: modified, N: new file, n: new file 2";
    print_text += &color_reset;
    print!("{}\n", print_text);
}
