use crate::prelude::*;


pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clear(&self) -> ReposError<()> {
        stdout().queue(Clear(ClearType::All))?;
        Ok(())
    }
}
