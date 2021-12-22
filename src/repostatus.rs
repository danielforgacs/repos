#[derive(Debug)]
pub struct RepoStatus {
    pub untracked: bool,
    pub deleted: bool,
    pub deleted_staged: bool,
    pub staged: bool,
    pub modified: bool,
    pub new_file: bool,
    pub new_file_2: bool,
}

impl RepoStatus {
    pub fn new() -> Self {
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

    pub fn is_ok(&self) -> bool {
        let has_bad_stuff = self.untracked
            || self.deleted
            || self.deleted_staged
            || self.staged
            || self.modified
            || self.new_file
            || self.new_file_2;
        !has_bad_stuff
    }
}

impl ToString for RepoStatus {
    fn to_string(&self) -> String {
        let empty_status = " ";
        let status_text = format!(
            "{}{}{}{}{}{}{}",
            if self.untracked { "U" } else { empty_status },
            if self.deleted { "D" } else { empty_status },
            if self.deleted_staged { "d" } else { empty_status },
            if self.staged { "S" } else { empty_status },
            if self.modified { "M" } else { empty_status },
            if self.new_file { "N" } else { empty_status },
            if self.new_file_2 { "n" } else { empty_status },
        );
        status_text
    }
}
