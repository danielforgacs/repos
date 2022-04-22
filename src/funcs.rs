use crate::prelude::*;

pub fn get_root_path() {
    let matches = Command::new("repos")
        .arg(
            Arg::new("rootpath")
        )
        .get_matches();
}
