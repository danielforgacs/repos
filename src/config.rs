use std::path::PathBuf;

const DEV_DIR_ENV_VAR: &str = "DEVDIR";

#[derive(Clone)]
pub struct Opts {
    dev_dir: std::path::PathBuf,
    max_width: u16,
}

impl Opts {
    pub fn new() -> Self {
        let dev_dir = match get_dev_dir() {
            Ok(path) => path,
            Err(_) => panic!("Can't find dev dir."),
        };
        Opts {
            dev_dir,
            max_width: 150,
        }
    }

    pub fn set_dev_dir(&mut self, path: String) -> Self {
        self.clone()
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
