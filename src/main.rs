mod funcs;

mod prelude {
    pub use clap::{Arg, Command};

    pub use crate::funcs::get_root_path;
}

use prelude::*;

fn main() {
    let root_path = get_root_path();
    dbg!(root_path);
}
