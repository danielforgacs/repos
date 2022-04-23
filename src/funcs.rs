use crate::prelude::*;

const HELP_TEXT: &str = r#"CLI util to manage all git repositories in a specific directory.
The root of the repos is coming from the "DEVDIR" env var
or the first argument."#;

pub fn get_root_path() -> ReposError<PathBuf> {
    let matches = Command::new("repos")
        .arg(Arg::new("rootpath"))
        .about(HELP_TEXT)
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
