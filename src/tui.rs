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
    selected_row: u16,
    current_column: u16,
    selected_column: u16,
    row_count: u16,
    buff: std::io::BufWriter<std::io::Stdout>,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            current_row: 0,
            selected_row: 0,
            current_column: 0,
            selected_column: 0,
            row_count: 0,
            buff: std::io::BufWriter::new(stdout()),
        }
    }

    pub fn clear(&mut self) -> ReposError<()> {
        self.buff
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        self.current_row = 0;
        self.current_column = 0;
        Ok(())
    }

    fn is_current_cell_selected(&self) -> bool {
        self.current_row == self.selected_row && self.current_column == self.selected_column
    }

    pub fn print(&mut self, text: &str) -> ReposError<()> {
        if self.is_current_cell_selected() {
            self.buff
                .queue(SetBackgroundColor(crossterm::style::Color::Red))?;
        }

        let colum_width = 30;

        self.buff
            .queue(MoveToColumn(self.current_column * colum_width))?
            .queue(Print(text))?;

        if self.is_current_cell_selected() {
            self.buff.queue(crossterm::style::ResetColor)?;
        }

        self.current_column += 1;

        Ok(())
    }

    pub fn flush(&mut self) -> ReposError<()> {
        self.buff.flush()?;
        Ok(())
    }

    pub fn new_line(&mut self) -> ReposError<()> {
        self.buff.queue(MoveToNextLine(1))?.queue(MoveToColumn(0))?;
        self.current_row += 1;
        self.current_column = 0;
        Ok(())
    }

    pub fn set_row_count(&mut self, count: u16) {
        if self.selected_row > count {
            self.selected_row = count;
        }
        self.row_count = count;
    }

    pub fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
            }
            Direction::Down => {
                if self.selected_row < self.row_count - 1 {
                    self.selected_row += 1
                }
            }
            Direction::Left => {
                if self.selected_column > 0 {
                    self.selected_column -= 1;
                }
            }
            Direction::Right => {
                if self.selected_column < 10 {
                    self.selected_column += 1;
                }
            }
        };
    }
}
