use crate::prelude::*;

pub fn get_root_path() -> ReposError<PathBuf> {
    let matches = Command::new("repos")
        .arg(Arg::new("rootpath"))
        .get_matches();
    let mut buff = PathBuf::new();
    let mut path_arg = Path::new("");
    let mut devdir = String::new();
    if let Some(rootdir) = matches.value_of("rootpath") {
        buff.push(rootdir);
        path_arg = Path::new(rootdir);
        if !path_arg.is_dir() {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Path argument is not a directory.",
            )));
        }
    } else {
        devdir = var(DEV_DIR_ENV_VAR)?;
        path_arg = Path::new(&devdir);
        if !path_arg.is_dir() {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                format!(
                    r#"Dir in dev env var: "{}" is not a directory."#,
                    DEV_DIR_ENV_VAR
                ),
            )));
        }
    }
    Ok(path_arg.canonicalize()?)
}
