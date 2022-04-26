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

    pub fn get_status(&self) -> Status {
        let mut stats = self.repo
            .statuses(None)
            .unwrap()
            .iter()
            .map(|f| f.status())
            .collect::<Vec<_>>();
        stats.sort_unstable();
        stats.dedup();
        dbg!(&stats);


        Status {}
    }
}

pub fn get_repos(paths: &Vec<PathBuf>) -> Vec<Repo> {
    paths.into_iter()
        .map(|f| Repo::new(&f))
        .map(|f| f.unwrap())
        .collect::<Vec<Repo>>()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Expected test result need to be generated for the tests.
    /// They can't be committed into this repo, becouse the're
    /// repos themselves.
    ///
    /// THIS PART SHOULD BE BE AUTOMATED
    const REPO_NAME: &str = "grouped_branches";
    const REPO_PATH: &str = "/tmp/tmp.x91X9GHwu0__repos_test/grouped_branches/";

    #[test]
    fn init_repo() {
        let repo = Repo::new(&PathBuf::from(REPO_PATH))
            .unwrap();
        assert_eq!(repo.get_name(), REPO_NAME);
    }
}
