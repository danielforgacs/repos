use crate::prelude::*;


pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clear(&self) -> ReposError<()> {
        stdout()
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        Ok(())
    }

    pub fn print(&self, text: &String) -> ReposError<()> {
        stdout().queue(Print(text))?;
        Ok(())
    }

    pub fn flush(&self) -> ReposError<()> {
        stdout().flush()?;
        Ok(())
    }

    pub fn new_line(&self) -> ReposError<()> {
        stdout().queue(MoveToNextLine(1))?;
        Ok(())
    }
}
