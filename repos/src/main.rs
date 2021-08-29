fn main() {
    let root = match std::fs::read_dir("/home/ford/storage/dev/") {
        Ok(dir) => { dir },
        Err(_) => {
            println!("Could not find dir.");
            return
        },
    };

    for dir_opt in root {
        // dbg!(&dir_opt);
        // dir_opt.Ok;
        let dir = match dir_opt {
            Ok(dir) => {dir},
            Err(_) => {continue},
        };
        // dbg!(&dir);
        println!("{:?}", dir.file_name());
    };
}
