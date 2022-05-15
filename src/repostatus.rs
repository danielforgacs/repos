/*
                            CURRENT             = raw::GIT_STATUS_CURRENT as u32;
is_index_new                INDEX_NEW           = raw::GIT_STATUS_INDEX_NEW as u32;
is_index_modified           INDEX_MODIFIED      = raw::GIT_STATUS_INDEX_MODIFIED as u32;
is_index_deleted            INDEX_DELETED       = raw::GIT_STATUS_INDEX_DELETED as u32;
is_index_renamed            INDEX_RENAMED       = raw::GIT_STATUS_INDEX_RENAMED as u32;
is_index_typechange         INDEX_TYPECHANGE    = raw::GIT_STATUS_INDEX_TYPECHANGE as u32;
is_wt_new                   WT_NEW              = raw::GIT_STATUS_WT_NEW as u32;
is_wt_modified              WT_MODIFIED         = raw::GIT_STATUS_WT_MODIFIED as u32;
is_wt_deleted               WT_DELETED          = raw::GIT_STATUS_WT_DELETED as u32;
is_wt_typechange            WT_TYPECHANGE       = raw::GIT_STATUS_WT_TYPECHANGE as u32;
is_wt_renamed               WT_RENAMED          = raw::GIT_STATUS_WT_RENAMED as u32;
is_ignored                  IGNORED             = raw::GIT_STATUS_IGNORED as u32;
is_conflicted               CONFLICTED          = raw::GIT_STATUS_CONFLICTED as u32;

*/

pub struct Status {
    current: bool,
    index_new: bool,
    index_modified: bool,
    index_deleted: bool,
    index_renamed: bool,
    index_typechange: bool,
    wt_new: bool,
    wt_modified: bool,
    wt_deleted: bool,
    wt_typechange: bool,
    wt_renamed: bool,
    ignored: bool,
    conflicted: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            current: true,
            index_new: false,
            index_modified: false,
            index_deleted: false,
            index_renamed: false,
            index_typechange: false,
            wt_new: false,
            wt_modified: false,
            wt_deleted: false,
            wt_typechange: false,
            wt_renamed: false,
            ignored: false,
            conflicted: false,
        }
    }

    pub fn to_status_vec(mut self, statuses: Vec<git2::Status>) -> Self {
        for item in statuses {
            if item.is_index_new() { self.index_new = true };
            if item.is_index_modified() { self.index_modified = true };
            if item.is_index_deleted() { self.index_deleted = true };
            if item.is_index_renamed() { self.index_renamed = true };
            if item.is_index_typechange() { self.index_typechange = true };
            if item.is_wt_new() { self.wt_new = true };
            if item.is_wt_modified() { self.wt_modified = true };
            if item.is_wt_deleted() { self.wt_deleted = true };
            if item.is_wt_typechange() { self.wt_typechange = true };
            if item.is_wt_renamed() { self.wt_renamed = true };
            if item.is_ignored() { self.ignored = true };
            if item.is_conflicted() { self.conflicted = true };
        }
        self
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[.......]")
    }
}
