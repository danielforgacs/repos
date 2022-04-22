use crate::prelude::*;

pub fn get_root_path() -> ReposError<PathBuf> {
    let matches = Command::new("repos")
        .arg(
            Arg::new("rootpath")
        )
        .get_matches();
    let mut buff = PathBuf::new();
    let mut path_arg = Path::new("");
    let mut devdir = String::new();
    if let Some(rootdir) = matches.value_of("rootpath") {
        buff.push(rootdir);
        path_arg = Path::new(rootdir);
    } else {
        devdir = var(DEV_DIR_ENV_VAR)?;
        path_arg = Path::new(&devdir);
    }
    if !path_arg.is_dir() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")));
    }
    Ok(path_arg.to_path_buf())
}
