/*
CURRENT
INDEX_NEW
INDEX_MODIFIED
INDEX_DELETED
INDEX_RENAMED
INDEX_TYPECHANGE
WT_NEW
WT_MODIFIED
WT_DELETED
WT_TYPECHANGE
WT_RENAMED
IGNORED
CONFLICTED
*/

pub struct Status;

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[.......]")
    }
}
