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
        use git2::{ErrorCode};

        #[derive(Eq, PartialEq)]
        enum Format {
            Long,
            Short,
            Porcelain,
        }


        let head = match self.repo.head() {
            Ok(head) => Some(head),
            Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
                None
            }
            Err(e) => return String::from("n/a"),
        };
        let head = head.as_ref().and_then(|h| h.shorthand());
        head.unwrap_or("HEAD (no branch)").to_string()
    }
}
