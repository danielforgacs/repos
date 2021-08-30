struct Root {
    path: String,
}

impl Root {
    fn new() -> Root {
        Root{path: "1".to_string()}
    }
}

fn main() {
    let pwd: std::path::PathBuf = match std::env::current_dir() {
        Ok(pwd) => pwd,
        _ => std::path::PathBuf::new(),
    };
    
    let rootname: &str = match pwd.as_path().to_str() {
        Some(pwd3) => pwd3,
        None => "",
    };
    
    let root: std::fs::ReadDir = match std::fs::read_dir(rootname) {
        Ok(dir) => dir,
        _ => {
            println!("Could not find dir.");
            return;
        }
    };

    let root2: String = get_root();
    let root3: Root = Root::new();

    for dir_opt in root {
        let dir: std::fs::DirEntry = match dir_opt {
            Ok(dir) => dir,
            _ => continue,
        };

        let stringdir: String = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            _ => continue,
        };

        let githead: String = format!("{}/{}/.git/HEAD", rootname, stringdir);
        let githead: String = match std::fs::read_to_string(&githead) {
            Ok(head) => head.trim().to_string(),
            _ => continue,
        };

        if githead != "ref: refs/heads/master" {
            println!("{: <35} {}", stringdir, githead);
        };
    };
}


fn get_root() -> String {
    let root = match std::env::current_dir() {
        Ok(pwd) => pwd,
        Err(_) => std::path::PathBuf::new(),
    };

    let root = root.to_str();

    let root = match root {
        Some(dir) => dir,
        None => "error...",
    };

    let root = String::from(root);
    root
}