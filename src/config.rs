use std::path::{PathBuf, Path};

const DEV_DIR_ENV_VAR: &str = "DEVDIR";

#[derive(Clone)]
pub struct Opts {
    dev_dir: PathBuf,
    repo_paths: Vec<PathBuf>,
    max_width: u16,
}

impl Opts {
    pub fn new() -> Self {
        let dev_dir = match get_dev_dir() {
            Ok(path) => path,
            Err(_) => panic!("Can't find dev dir."),
        };
        let repo_paths = find_repo_dirs(&dev_dir);
        Opts {
            dev_dir,
            repo_paths,
            max_width: 150,

        }
    }

    pub fn get_max_width(&self) -> u16 {
        self.max_width
    }

    pub fn set_dev_dir(&mut self, path: String) -> Self {
        let mut new_opts = self.clone();
        let mut dev_dir = PathBuf::new();
        dev_dir.push(path);
        new_opts.dev_dir = dev_dir;
        new_opts.repo_paths = find_repo_dirs(&new_opts.dev_dir);
        new_opts
    }

    pub fn get_repo_paths(&self) -> &Vec<PathBuf> {
        &self.repo_paths
    }

    pub fn get_dev_dir(&self) -> &PathBuf {
        &self.dev_dir
    }
}

fn get_dev_dir() -> Result<PathBuf, std::io::Error> {
    let path = match std::env::var(DEV_DIR_ENV_VAR) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => std::env::current_dir(),
    };
    match path {
        Ok(path) => Ok(path),
        Err(error) => Err(error),
    }
}

fn find_repo_dirs(root: &Path) -> Vec<PathBuf> {
    let mut repos: Vec<PathBuf> = Vec::new();

    if let Ok(read_dir) = root.read_dir() {
        for dir in read_dir {
            if dir.as_ref().expect("msg").path().join(".git").is_dir() {
                repos.push(dir.unwrap().path().to_path_buf())
            }
        }
    }

    repos.sort_by_key(|x| x.to_str().unwrap().to_lowercase());
    repos
}
