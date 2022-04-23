use crate::prelude::*;

pub fn get_root_path() -> ReposError<PathBuf> {
    let matches = Command::new("repos")
        .arg(Arg::new("rootpath"))
        .get_matches();
    if let Some(rootdir) = matches.value_of("rootpath") {
        let mut buff = PathBuf::new();
        buff.push(rootdir);
        let path_arg = Path::new(rootdir);
        if !path_arg.is_dir() {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Path argument is not a directory.",
            )));
        }
        Ok(path_arg.canonicalize()?)
    } else {
        let devdir = var(DEV_DIR_ENV_VAR)?;
        let path_arg = Path::new(&devdir);
        if !path_arg.is_dir() {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                format!(
                    r#"Dir in dev env var: "{}" is not a directory."#,
                    DEV_DIR_ENV_VAR
                ),
            )));
        }
        Ok(path_arg.canonicalize()?)
    }
}
