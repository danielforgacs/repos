use crate::prelude::*;

pub struct Repo {
    repo: Repository,
    name: String,
    current_branch: String,
}

impl Repo {
    pub fn new(path: &PathBuf) -> ReposResult<Self> {
        let repo = Repository::open(path)?;
        let current_branch = read_current_branch(&repo);
        let name = repo
            .path()
            .components()
            .nth_back(1)
            .map(|f| f.as_os_str())
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();

        Ok(Self { repo, name, current_branch })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn current_branch(&self) -> &str {
        &self.current_branch
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
        let mut status_options = StatusOptions::new();
        status_options.include_untracked(true);
        status_options.include_ignored(true);
        let mut stats = self
            .repo
            .statuses(Some(&mut status_options))
            .unwrap()
            .iter()
            .map(|f| f.status())
            .collect::<Vec<_>>();
        stats.sort_unstable();
        stats.dedup();
        Status::new().set_from_vec(stats)
    }
}

fn read_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
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


#[cfg(test)]
mod test {
    use super::*;

    /// Expected test result need to be generated for the tests.
    /// They can't be committed into this repo, becouse the're
    /// repos themselves.
    ///
    /// THIS PART SHOULD BE BE AUTOMATED
    const REPO_NAME: &str = "grouped_branches";
    const REPO_PATH: &str = "/tmp/tmp.PHBl9PWOQL__repos_test/grouped_branches/";

    #[test]
    fn init_repo() {
        let repo = Repo::new(&PathBuf::from(REPO_PATH)).unwrap();
        assert_eq!(repo.name(), REPO_NAME);
    }
}
