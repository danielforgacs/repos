use crate::prelude::*;

pub struct Repo {
    pub repo: Repository,
    current_branch: String,
}

impl Repo {
    pub fn new(path: &PathBuf) -> ReposError<Self> {
        let repo  = Repository::open(path)?;
        Ok(Self {
            repo,
            current_branch: String::new(),
        })
    }
}
