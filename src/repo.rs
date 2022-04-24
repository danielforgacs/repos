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
        String::from("n/a")
    }

    pub fn get_current_branch(&self) -> String {
        String::from("n/a")
    }
}
