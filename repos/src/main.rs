fn main() {
    let pwd = match std::env::current_dir() {
        Ok(pwd) => pwd,
        _ => std::path::PathBuf::new(),
    };
    dbg!(pwd);
    let rootname: &str = "/home/ford/storage/dev/";
    let root = match std::fs::read_dir(rootname) {
        Ok(dir) => { dir },
        _ => {
            println!("Could not find dir.");
            return
        },
    };

    for dir_opt in root {
        let dir = match dir_opt {
            Ok(dir) => dir,
            _ => continue,
        };

        let stringdir = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            _ => continue,
        };

        let githead: String = format!("{}{}/.git/HEAD", rootname, stringdir);
        let githead = match std::fs::read_to_string(&githead) {
            Ok(head) => head,
            _ => continue ,
        };
        if githead != "ref: refs/heads/master\n" {
            println!("[DIR]: {: <20} [HEAD]: {}", stringdir, githead);
        };
    };
}
