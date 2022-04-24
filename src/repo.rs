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
        let head = match self.repo.head() {
            Ok(head) => Some(head),
            Err(ref e)
                if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound =>
            {
                None
            }
            Err(_error) => return String::from("n/a"),
        };
        let head = head.as_ref().and_then(|h| h.shorthand());
        head.unwrap_or("HEAD (no branch)").to_string()
    }

    /// Get all local branches
    pub fn get_branches(&self) -> Vec<String> {
        self.repo
            .branches(None)
            .unwrap()
            .map(|f| f.unwrap())
            .map(|f| f.0)
            .map(|f| f.name().unwrap().unwrap().to_string())
            .collect()
    }
}
