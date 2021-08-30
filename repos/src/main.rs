fn main() {
    let pwd = match std::env::current_dir() {
        Ok(pwd) => pwd,
        _ => std::path::PathBuf::new(),
    };
    
    let rootname: &str = match pwd.as_path().to_str() {
        Some(pwd3) => pwd3,
        None => "",
    };
    // println!("..rootname: {}", rootname);
    
    let root = match std::fs::read_dir(rootname) {
        Ok(dir) => dir,
        _ => {
            println!("Could not find dir.");
            return;
        }
    };

    // dbg!(&root);

    for dir_opt in root {
        // dbg!(&dir_opt);
        let dir = match dir_opt {
            Ok(dir) => dir,
            _ => continue,
        };

        let stringdir = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            _ => continue,
        };

        let githead: String = format!("{}/{}/.git/HEAD", rootname, stringdir);
        // dbg!(&githead);
        let githead = match std::fs::read_to_string(&githead) {
            Ok(head) => head.trim().to_string(),
            _ => continue,
        };
        // dbg!(&githead);

        if githead != "ref: refs/heads/master" {
            println!("{: <35} {}", stringdir, githead);
        };
    };
}
