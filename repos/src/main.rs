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

        let stringdir = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            Err(_) => continue,
        };

        let githead: String = format!("{}{}/.git/HEAD", rootname, stringdir);
        let githead = match std::fs::read_to_string(&githead) {
            Ok(head) => head,
            Err(error) => format!("[ERROR] {}: {}", error, githead),
        };
        println!("{}", githead);
    };
}
