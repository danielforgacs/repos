fn main() {
    let rootname = "/home/ford/storage/dev/";
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
        println!("{:?}", dir.file_name());
    };
}
