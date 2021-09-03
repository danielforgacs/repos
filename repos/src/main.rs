mod root {
    use std::fs::ReadDir;

    pub struct Root {
        pub name: String,
        pub dirs: ReadDir,
        // alldirs_iter: std::fs::ReadDir,
    }

    #[derive(Debug)]
    pub enum Devdir {
        Some(String),
        None,
    }
    #[derive(Debug)]
    pub struct Parms {
        pub showdot: bool,
        pub devdir: Devdir,
    }
}

use root::Parms;
use root::Root;
use std::env::{args, current_dir};
use std::fs::{read_dir, read_to_string, DirEntry};
use std::path::PathBuf;
use std::process::Command;

impl Parms {
    fn new() -> Self {
        let args: Vec<String> = args().skip(1).collect();
        let showdot = if args.iter().any(|i| i == "-dot") {
            true
        } else {
            false
        };

        let mut count = 0;
        let mut devdir = root::Devdir::None;
        for item in args.iter() {
            count += 1;

            if item == "-d" {
                let dirstr = match args.get(count) {
                    Some(dstr) => dstr,
                    None => "",
                };

                if dirstr != "" {
                    devdir = root::Devdir::Some(args[count].as_str().to_string());
                } else {
                    println!("Missing dev dir after \"-d\" arg.");
                };
            }
        }

        Parms { showdot, devdir }
    }
}

impl Root {
    fn new(devdir: root::Devdir) -> Result<Self, std::io::Error> {
        let is_startdir: bool = match devdir {
            root::Devdir::Some(ref _dir) => true,
            _ => false,
        };

        let pwd: PathBuf = match current_dir() {
            Ok(pwd) => pwd,
            Err(error) => return Result::Err(error),
        };

        let mut name: String = match pwd.as_path().to_str() {
            Some(pwd3) => String::from(pwd3),
            None => String::from(""),
        };

        if is_startdir {
            name = match devdir {
                root::Devdir::Some(dir) => dir,
                _ => String::from(""),
            }
        }

        let dirs = match read_dir(&name) {
            Ok(dirs) => dirs,
            Err(error) => return Result::Err(error),
        };

        Result::Ok(Root { name, dirs })
    }
}

fn main() {
    // check_status("/home/ford/storage/dev/Rust101/");
    // check_status("/home/ford/storage/dev/__cpython3.9.6");
    list_non_master_repos();
}

fn check_status(dir: &str) -> String {
    // println!("-- status checking dir: {}", dir);
    // let rawoutput = Command::new("git").arg("status").arg("--porcelain").output();
    let rawoutput = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(dir)
        .output();
    let response: String = match rawoutput {
        Ok(resp) => {
            let stdout = match String::from_utf8(resp.stdout) {
                Ok(text) => text,
                Err(error) => error.to_string(),
            };
            stdout
        }
        Err(error) => error.to_string(),
    };
    // println!("{}", response);
    response
}

fn list_non_master_repos() {
    let parms = Parms::new();
    // println!("{:?}", parms);

    let root = match Root::new(parms.devdir) {
        Ok(root) => root,
        Err(_) => {
            println!("Could not read path.");
            return;
        }
    };

    for dir_opt in root.dirs {
        // println!("{:?}", &dir_opt);
        let dir: DirEntry = match dir_opt {
            Ok(dir) => {
                let is_dir = match dir.file_type() {
                    Ok(isdir2) => isdir2.is_dir(),
                    Err(_error) => false,
                };
                if is_dir == false {
                    continue;
                }
                // println!("###{}, {:?}", "OK", dir);
                dir
            }
            _ => continue,
        };
        // println!("{:?}", dir);
        
        // println!(":{:?}", &dir_opt);
        
        // check_status(&format!("{}/{}", root.name, stringdir));
        let stringdir: String = match dir.file_name().into_string() {
            Ok(dirn) => dirn,
            _ => continue,
        };
        
        if stringdir.chars().nth(0) == Some('.') {
            if parms.showdot == false {
                continue;
            }
        };
        let status = check_status(&format!("{}/{}", root.name, stringdir));

        let githead: String = format!("{}/{}/.git/HEAD", root.name, stringdir);
        let githead: String = match read_to_string(&githead) {
            Ok(head) => head.trim().to_string(),
            _ => continue,
        };

        println!("____________________________________________________________");
        
        if status != "" {
            let stralign = format!("[{}]", stringdir);
            println!("{: <35}", stralign);
            println!("{}", status);
        };
        
        // println!("................................................");
        if githead != "ref: refs/heads/master" {
            let stralign = format!("[{}]", stringdir);
            println!("{: <35} {}", stralign, githead);
        };

    }
}
