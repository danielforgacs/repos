use crate::prelude::*;

pub struct Repo {
    repo: Repository,
}

impl Repo {
    pub fn new(path: &PathBuf) -> ReposError<Self> {
        Ok(Self {
            repo: Repository::open(path)?,
        })
    }

    pub fn get_name(&self) -> String {
        self.repo
            .path()
            .components()
            .nth_back(1)
            .map(|f| f.as_os_str())
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap()
    }

    pub fn get_current_branch(&self) -> String {
        String::from("n/a")
    }
}
