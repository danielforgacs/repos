fn main() {
    let rootname: &str = "/home/ford/storage/dev/";
    let root = match std::fs::read_dir(rootname) {
        Ok(dir) => { dir },
        Err(_) => {
            println!("Could not find dir.");
            return
        },
    };

    for dir_opt in root {
        let dir = match dir_opt {
            Ok(dir) => dir,
            Err(_) => continue,
        };
        // println!("{:?}", dir.file_name());

        let stringdir = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            Err(_) => continue,
        };
        // dbg!(&stringdir);
        let fullpath: String = format!("{}{}/.git", rootname, stringdir);
        // dbg!(fullpath);
        println!("{:?}", fullpath);
    };
}
