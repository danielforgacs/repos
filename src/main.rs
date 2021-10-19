const MAX_STATUS_LINES: usize = 5;
const STATUS_MARKER_LENGTH: usize = 2;

struct DevDir {
    path: String,
    repos: Vec<Repo>,
}

struct Repo {
    path: String,
    pbuf: std::path::PathBuf,
}

impl DevDir {
    fn new() -> Self {
        let env_devdir: String = match std::env::var("DEVDIR") {
            Ok(devd) => { devd },
            Err(_) => { "-".to_string() }
        };
        // println!("env-devdir: {}", env_devdir);
        let mut repos: Vec<Repo> = Vec::new();
        for root in std::fs::read_dir(&env_devdir) {
            // println!(";;;{:?}", root);
            for sudir in root {
                let direntry = sudir.unwrap();
                // println!("{:?}", direntry);

                let dirtype = match direntry.file_type() {
                    Ok(dirt) => {
                        // println!("{:?}", dirt.is_dir());
                        dirt.is_dir()
                    }
                    _ => { false }
                };
                // println!("{:?}", dirtype);
                if !dirtype {
                    // println!("{:?}", dirtype);
                    continue
                }
                // println!("{:?}", direntry.path());
                let dirstring = direntry.path().to_str().unwrap().to_string();
                let rep = {&dirstring}.to_string() + "/.git";
                let mut git_dir = std::path::PathBuf::new();
                git_dir.push(rep);
                // let s:()=git_dir;
                if !git_dir.is_dir() {
                    continue
                }
                // println!("{:?}", git_dir);

                let repo = Repo {
                    // path: git_dir.as_path().to_str().unwrap().to_string(),
                    path: dirstring,
                    pbuf: git_dir,
                };
                repos.push(repo);
                // let xyz: () = git_dir;
                // println!("{}", git_dir.is_dir());


                // println!("{:?}", sudir.unwrap().path());

            }
        }
        // match readdir {
        //     Ok(rd) => {println!("{:?}", rd);
        //     // let qw:()=rd;
        //     for k in rd {
        //         // println!("{:?}", k);
        //         match k {
        //             Ok(d) => {
        //                 println!("{:?}", d);
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        //     _ => {}
        // }
        // let q:()=readdir;
        // for k in readdir {
        //     println!("{:?}", k);
        // }
        // println!("++{:?}", readdir);
        // for item in std::fs::read_dir(&devdir) {
        //     println!("{:?}", item);
        // }
        Self {
            path: env_devdir,
            repos,
        }
    }
}

mod root {
    use std::fs::ReadDir;

    pub struct Root {
        pub name: String,
        pub dirs: ReadDir,
    }

    pub enum Devdir {
        Some(String),
        None,
    }

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
    // diagnose_repos();
    check_repos();
}

fn check_repos() {
    let devdir = DevDir::new();
    for repo in devdir.repos {
        let mut repotext = "\n------------------------------".to_string();
        repotext += format!("\npath: {}", repo.path).as_str();
        print!("{}", repotext)
        // repotext += format!("path: {}", repo.path).to_string();
        // println!("{}, {}", repo.path, repo.pbuf);
        // let xyz: () = repo.path;
        // let xyz: () = repo.pbuf;
    }
}

fn check_status(dir: &str) -> String {
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
    let mut newresponse: String = String::new();

    if response != "" {
        let mut linecount = 0;

        for line in response.split('\n') {
            // println!("line: {}, len: {}", &line, line.len());

            if line.len() < STATUS_MARKER_LENGTH + 1 {
                continue;
            }

            linecount += 1;

            if linecount > MAX_STATUS_LINES {
                newresponse.push_str("    (more...)\n");

                break;
            };
            let statusname = match &line[..STATUS_MARKER_LENGTH] {
                "??" => "untracked:",
                " D" => "deleted:",
                "D " => "deleted staged:",
                "M " => "staged:",
                " M" => "modified:",
                "A " => "new file:",
                "AM" => "new file 2:",
                _ => "(unknown)",
            };
            let newline = format!(
                "    {: <15} {}\n",
                statusname,
                &line[STATUS_MARKER_LENGTH + 1..]
            );
            // let newline: String;
            // if statusname == "deleted:" {
            //     newline = format!("    {: <12} {}\n", statusname, &line[2..]);
            // } else {
            //     newline = format!("    {: <12} {}\n", statusname, &line[3..]);
            // };
            newresponse.push_str(newline.as_str());
        }

        newresponse = newresponse[..newresponse.len() - 1].to_string()
    };

    newresponse
}

fn diagnose_repos() {
    let parms = Parms::new();
    let root = match Root::new(parms.devdir) {
        Ok(root) => root,
        Err(_) => {
            println!("Could not read path.");
            return;
        }
    };

    for dir_opt in root.dirs {
        let dir: DirEntry = match dir_opt {
            Ok(dir) => {
                let is_dir = match dir.file_type() {
                    Ok(isdir2) => isdir2.is_dir(),
                    Err(_error) => false,
                };
                if is_dir == false {
                    continue;
                }
                dir
            }
            _ => continue,
        };

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
            Ok(head) => {
                let branch = head.trim().to_string();
                let branch = get_branch(branch);
                branch
            }
            _ => continue,
        };

        let mut do_print = false;

        if status != "" {
            do_print = true;
        };

        if githead != "master" {
            do_print = true;
        };

        if do_print {
            let stralign = format!("{}", stringdir.trim());
            println!("___________________________________________________________");

            if githead.trim() == "master" {
                println!("{: <35} {}", stralign, githead.trim());
            } else {
                println!("{: <35} {: <15} *", stralign, githead.trim());
            }

            if status != "" {
                println!("{}", status);
            }
        }
    }
}

fn get_branch(head: String) -> String {
    let branch = match head.split("/").last() {
        Some(element) => element,
        None => "",
    };
    branch.to_string()
}
