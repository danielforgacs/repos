use std::path::PathBuf;

use crate::repostatus;
use crate::tui;

pub enum RepoState {
    MasterOk,
    MasterNotOk,
    NotMasterOK,
    NotMasterNotOK,
}

pub struct Repo {
    pub name: String,
    pub path: PathBuf,
    pub status: repostatus::RepoStatus,
    pub branches: Vec<String>,
    pub current_branch: String,
    pub status_text: String,
}

impl Repo {
    pub fn new(path: PathBuf, name_width: &usize) -> Self {
        let mut name = path
            .file_name()
            .expect("can't get repo name from path")
            .to_str()
            .unwrap()
            .to_string();
        if name.len() >= *name_width {
            name.truncate(*name_width - 1);
            name.push('~');
        } else {
            // name = format!("{: >w$}", name, w = name_width);
            name = format!("{: <w$}", name, w = name_width);
        }
        let mut repo = Self {
            name,
            status: repostatus::RepoStatus::new(),
            branches: Vec::new(),
            path,
            current_branch: String::new(),
            status_text: String::new(),
        };
        repo.update();
        repo
    }

    pub fn update(&mut self) {
        self.update_status();
        self.update_branches();
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
        self.status.untracked = false;
        self.status.deleted = false;
        self.status.deleted_staged = false;
        self.status.staged = false;
        self.status.modified = false;
        self.status.new_file = false;
        self.status.new_file_2 = false;
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
        if !self.status.is_ok() {
            self.update_status_text();
        }
    }

    fn update_status_text(&mut self) {
        let output = std::process::Command::new("git")
            .arg("status")
            .current_dir(&self.path)
            .output()
            .expect("can't get status.");
        let mut output_fixed: Vec<u8> = Vec::new();
        for ch in output.stdout {
            if ch == '\n' as u8 {
                output_fixed.push('\r' as u8)
            }
            output_fixed.push(ch)
        }
        self.status_text = String::from_utf8(output_fixed)
            .expect("can't get status output");
    }

    pub fn get_repo_state(&self) -> RepoState {
        match self.current_branch.as_ref() {
            "master" => match self.status.is_ok() {
                true => RepoState::MasterOk,
                false => RepoState::MasterNotOk,
            },
            _ => match self.status.is_ok() {
                true => RepoState::NotMasterOK,
                false => RepoState::NotMasterNotOK,
            },
        }
    }

    pub fn clear_stat(&mut self) {
        std::process::Command::new("git")
            .arg("reset")
            .arg(".")
            .current_dir(&self.path)
            .output()
            .expect("Could not checkout repos.");
        std::process::Command::new("git")
            .arg("checkout")
            .arg(".")
            .current_dir(&self.path)
            .output()
            .expect("Could not checkout repos.");
        self.update_status();
    }

    pub fn checkout_branch(&mut self, branch: String) {
        std::process::Command::new("git")
            .arg("checkout")
            .arg(branch)
            .current_dir(&self.path)
            .output()
            .expect("Could not checkout repos.");
        self.update_current_branch();
    }
}
