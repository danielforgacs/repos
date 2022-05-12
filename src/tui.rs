use crate::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Tui {
    current_row: u16,
    current_column: u16,
    row_count: u16,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            current_row: 0,
            current_column: 0,
            row_count: 0,
        }
    }

    pub fn clear(&mut self) -> ReposError<()> {
        stdout()
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        Ok(())
    }

    pub fn print(&mut self, text: &String, index: u16, column: u16) -> ReposError<()> {
        if index == self.current_row {
            stdout()
                .queue(SetBackgroundColor(crossterm::style::Color::Red))?;
        }

        stdout()
            .queue(MoveToColumn(column * 40))?
            .queue(Print(text))?;

        if index == self.current_row {
            stdout()
                .queue(crossterm::style::ResetColor)?;
        }
        Ok(())
    }

    pub fn flush(&self) -> ReposError<()> {
        stdout().flush()?;
        Ok(())
    }

    pub fn new_line(&mut self) -> ReposError<()> {
        stdout()
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?;
        Ok(())
    }

    pub fn set_row_count(&mut self, count: u16) {
        if self.current_row > count {
            self.current_row = count;
        }
        self.row_count = count;
    }

    pub fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.current_row > 0 {
                    self.current_row -= 1;
                }
            }
            Direction::Down => {
                if self.current_row < self.row_count - 1{
                    self.current_row += 1
                }
            },
            Direction::Left => {
                if self.current_column > 0 {
                    self.current_column -= 1;
                }
            }
            Direction::Right => {
                if self.current_column < 10 {
                    self.current_column += 1;
                }
            }
        };
    }
}
