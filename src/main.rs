use std::path::PathBuf;
use std::fs::{read_to_string};
use std::process::Command;

const REPO_NAME_WIDTH: usize = 25;
const BRANCH_NAME_WIDTH: usize = 50;

struct DevDir {
    _path: PathBuf,
    repos: Vec<Repo>,
}

struct Repo {
    name: String,
    path: PathBuf,
}

#[derive(Debug)]
struct RepoStatus {
    untracked: bool,
    deleted: bool,
    deleted_staged: bool,
    staged: bool,
    modified: bool,
    new_file: bool,
    new_file_2: bool,
}

impl DevDir {
    fn new(rootdir: String) -> Self {
        let mut devdir = PathBuf::new();
        devdir.push(rootdir);
        let mut repos: Vec<Repo> = Vec::new();
        for entry in devdir.read_dir().unwrap() {
            let entry = match entry {
                Ok(entry) => { entry.path() },
                Err(_) => { PathBuf::new() },
            };
            let entry_git = entry.to_str().unwrap().to_string() + "/.git";
            if !std::path::Path::new(&entry_git).is_dir() {
                continue
            }
            let repo = Repo::new(entry);
            repos.push(repo);
        }
        repos.sort_by(|repo_a, repo_b| repo_a.name.to_lowercase().cmp(&repo_b.name.to_lowercase()));
        DevDir {
            _path: devdir,
            repos,
        }
    }
}

impl Repo {
    fn new(path: PathBuf) -> Self {
        let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
        if name.len() > REPO_NAME_WIDTH {
            name = String::from(&name[..REPO_NAME_WIDTH-1]);
            name += "~";
        }
        Self { name, path }
    }

    fn branch(&self) -> String {
        let mut head_file = PathBuf::new();
        head_file.push(self.path.to_str().unwrap().to_string() + "/.git/HEAD");
        let githead: String = read_to_string(&head_file).unwrap();
        let githead = githead.trim().to_string();
        let mut branch = githead.split("/").last().unwrap().to_string();
        if branch.len() > BRANCH_NAME_WIDTH {
            branch = branch[..BRANCH_NAME_WIDTH-1].to_string();
            branch += "~";
        }
        branch
    }

    fn status(&self) -> RepoStatus {
        let status_stdout = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .current_dir(&self.path)
            .output().unwrap().stdout;
        let status_stdout = String::from_utf8(status_stdout).unwrap();
        let mut status = RepoStatus::new();
        let status_mark_width = 2;
        for line in status_stdout.lines() {
            match &line[..status_mark_width] {
                "??" => status.untracked = true,
                " D" => status.deleted = true,
                "D " => status.deleted_staged = true,
                "M " => status.staged = true,
                " M" => status.modified = true,
                "A " => status.new_file = true,
                "AM" => status.new_file_2 = true,
                _ => (),
            };
        }
        status
    }
}

impl RepoStatus {
    fn new() -> Self {
        Self {
            untracked: false,
            deleted: false,
            deleted_staged: false,
            staged: false,
            modified: false,
            new_file: false,
            new_file_2: false,
        }
    }
}

fn main() {
    // diagnose_repos();
    check_repos();
}

fn check_repos() {
    let devdir_env: String = std::env::var("DEVDIR").unwrap();
    let devdir = DevDir::new(devdir_env);
    let header = format!("{:>re$} |{:^st$}| {:br$}",
        "repo", "status", "branch", re=REPO_NAME_WIDTH, st=7, br=BRANCH_NAME_WIDTH);
    let empty_status = " ";
    let mut print_text = "".to_string();
    print_text.push_str(&header);
    for repo in devdir.repos {
        let branch = if repo.branch() == "master" { "".to_string() } else { repo.branch() };
        let status = repo.status();
        let status_text = format!("[{}{}{}{}{}{}{}]",
            if status.untracked { "U" } else { empty_status },
            if status.deleted { "D" } else { empty_status },
            if status.deleted_staged { "d" } else { empty_status },
            if status.staged { "S" } else { empty_status },
            if status.modified { "M" } else { empty_status },
            if status.new_file { "N" } else { empty_status },
            if status.new_file_2 { "n" } else { empty_status },
        );
        print_text += format!("\n{:>rw$} {} {:bw$}", repo.name, status_text, branch, rw=REPO_NAME_WIDTH, bw=BRANCH_NAME_WIDTH).as_str();
    }
    print!("{}\n", print_text);
}

// const MAX_STATUS_LINES: usize = 5;
// const STATUS_MARKER_LENGTH: usize = 2;

// mod root {
//     use std::fs::ReadDir;
//
//     pub struct Root {
//         pub name: String,
//         pub dirs: ReadDir,
//     }
//
//     pub enum Devdir {
//         Some(String),
//         None,
//     }
//
//     pub struct Parms {
//         pub showdot: bool,
//         pub devdir: Devdir,
//     }
// }

// use root::Parms;
// use root::Root;
// use std::env::{args, current_dir};
// use std::fs::{read_dir, read_to_string, DirEntry};
// use std::process::Command;

// impl Parms {
//     fn new() -> Self {
//         let args: Vec<String> = args().skip(1).collect();
//         let showdot = if args.iter().any(|i| i == "-dot") {
//             true
//         } else {
//             false
//         };
//
//         let mut count = 0;
//         let mut devdir = root::Devdir::None;
//
//         for item in args.iter() {
//             count += 1;
//
//             if item == "-d" {
//                 let dirstr = match args.get(count) {
//                     Some(dstr) => dstr,
//                     None => "",
//                 };
//
//                 if dirstr != "" {
//                     devdir = root::Devdir::Some(args[count].as_str().to_string());
//                 } else {
//                     println!("Missing dev dir after \"-d\" arg.");
//                 };
//             }
//         }
//
//         Parms { showdot, devdir }
//     }
// }

// impl Root {
//     fn new(devdir: root::Devdir) -> Result<Self, std::io::Error> {
//         let is_startdir: bool = match devdir {
//             root::Devdir::Some(ref _dir) => true,
//             _ => false,
//         };
//
//         let pwd: PathBuf = match current_dir() {
//             Ok(pwd) => pwd,
//             Err(error) => return Result::Err(error),
//         };
//
//         let mut name: String = match pwd.as_path().to_str() {
//             Some(pwd3) => String::from(pwd3),
//             None => String::from(""),
//         };
//
//         if is_startdir {
//             name = match devdir {
//                 root::Devdir::Some(dir) => dir,
//                 _ => String::from(""),
//             }
//         }
//
//         let dirs = match read_dir(&name) {
//             Ok(dirs) => dirs,
//             Err(error) => return Result::Err(error),
//         };
//
//         Result::Ok(Root { name, dirs })
//     }
// }

// fn check_status(dir: &str) -> String {
//     let rawoutput = Command::new("git")
//         .arg("status")
//         .arg("--porcelain")
//         .current_dir(dir)
//         .output();
//
//     let response: String = match rawoutput {
//         Ok(resp) => {
//             let stdout = match String::from_utf8(resp.stdout) {
//                 Ok(text) => text,
//                 Err(error) => error.to_string(),
//             };
//             stdout
//         }
//         Err(error) => error.to_string(),
//     };
//     let mut newresponse: String = String::new();
//
//     if response != "" {
//         let mut linecount = 0;
//
//         for line in response.split('\n') {
//             // println!("line: {}, len: {}", &line, line.len());
//
//             if line.len() < STATUS_MARKER_LENGTH + 1 {
//                 continue;
//             }
//
//             linecount += 1;
//
//             if linecount > MAX_STATUS_LINES {
//                 newresponse.push_str("    (more...)\n");
//
//                 break;
//             };
//             let statusname = match &line[..STATUS_MARKER_LENGTH] {
//                 "??" => "untracked:",
//                 " D" => "deleted:",
//                 "D " => "deleted staged:",
//                 "M " => "staged:",
//                 " M" => "modified:",
//                 "A " => "new file:",
//                 "AM" => "new file 2:",
//                 _ => "(unknown)",
//             };
//             let newline = format!(
//                 "    {: <15} {}\n",
//                 statusname,
//                 &line[STATUS_MARKER_LENGTH + 1..]
//             );
//             // let newline: String;
//             // if statusname == "deleted:" {
//             //     newline = format!("    {: <12} {}\n", statusname, &line[2..]);
//             // } else {
//             //     newline = format!("    {: <12} {}\n", statusname, &line[3..]);
//             // };
//             newresponse.push_str(newline.as_str());
//         }
//
//         newresponse = newresponse[..newresponse.len() - 1].to_string()
//     };
//
//     newresponse
// }

// fn diagnose_repos() {
//     let parms = Parms::new();
//     let root = match Root::new(parms.devdir) {
//         Ok(root) => root,
//         Err(_) => {
//             println!("Could not read path.");
//             return;
//         }
//     };
//
//     for dir_opt in root.dirs {
//         let dir: DirEntry = match dir_opt {
//             Ok(dir) => {
//                 let is_dir = match dir.file_type() {
//                     Ok(isdir2) => isdir2.is_dir(),
//                     Err(_error) => false,
//                 };
//                 if is_dir == false {
//                     continue;
//                 }
//                 dir
//             }
//             _ => continue,
//         };
//
//         let stringdir: String = match dir.file_name().into_string() {
//             Ok(dirn) => dirn,
//             _ => continue,
//         };
//
//         if stringdir.chars().nth(0) == Some('.') {
//             if parms.showdot == false {
//                 continue;
//             }
//         };
//
//         let status = check_status(&format!("{}/{}", root.name, stringdir));
//         let githead: String = format!("{}/{}/.git/HEAD", root.name, stringdir);
//         let githead: String = match read_to_string(&githead) {
//             Ok(head) => {
//                 let branch = head.trim().to_string();
//                 let branch = get_branch(branch);
//                 branch
//             }
//             _ => continue,
//         };
//
//         let mut do_print = false;
//
//         if status != "" {
//             do_print = true;
//         };
//
//         if githead != "master" {
//             do_print = true;
//         };
//
//         if do_print {
//             let stralign = format!("{}", stringdir.trim());
//             println!("___________________________________________________________");
//
//             if githead.trim() == "master" {
//                 println!("{: <35} {}", stralign, githead.trim());
//             } else {
//                 println!("{: <35} {: <15} *", stralign, githead.trim());
//             }
//
//             if status != "" {
//                 println!("{}", status);
//             }
//         }
//     }
// }

// fn get_branch(head: String) -> String {
//     let branch = match head.split("/").last() {
//         Some(element) => element,
//         None => "",
//     };
//     branch.to_string()
// }
