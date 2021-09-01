mod root {
    pub struct Root {
        pub name: String,
        pub dirs: std::fs::ReadDir,
        // alldirs_iter: std::fs::ReadDir,
    }
}

use root::Root;

impl Root {
    fn new() -> Result<Self, std::io::Error> {
        let pwd: std::path::PathBuf = match std::env::current_dir() {
            Ok(pwd) => pwd,
            Err(error) => return std::result::Result::Err(error),
        };

        let name: String = match pwd.as_path().to_str() {
            Some(pwd3) => String::from(pwd3),
            None => String::from(""),
        };

        let rootdirs = match std::fs::read_dir(&name) {
            Ok(dirs) => dirs,
            Err(error) => return Result::Err(error),
        };
    
        Result::Ok(Root {
            name: name,
            dirs: rootdirs,
        })
    }
}

fn main() {
    let root = match Root::new() {
        Ok(root) => root,
        Err(_) => {
            println!("Could not read path.");
            return
        }
    };

    for dir_opt in root.dirs {
        let dir: std::fs::DirEntry = match dir_opt {
            Ok(dir) => dir,
            _ => continue,
        };

        let stringdir: String = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            _ => continue,
        };

        let githead: String = format!("{}/{}/.git/HEAD", root.name, stringdir);
        let githead: String = match std::fs::read_to_string(&githead) {
            Ok(head) => head.trim().to_string(),
            _ => continue,
        };

        if githead != "ref: refs/heads/master" {
            println!("{: <35} {}", stringdir, githead);
        };
    };
}
