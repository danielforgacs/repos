mod funcs;

mod prelude {
    pub use crate::funcs::get_root_path;
}

use prelude::*;

fn main() {
    let root_path = get_root_path();
}
