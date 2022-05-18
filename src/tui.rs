use crate::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Tui {
    // row that's being currently printed in the loop.
    // this is checked against the selected row.
    // If the wip row == the selected row,
    // the wip row is the selected one.
    wip_row: u16,
    selected_row: u16,
    wip_column: u16,
    pub current_column_coord: u16,
    selected_column: u16,
    row_count: u16,
    column_counts: Vec<u16>,
    buff: std::io::BufWriter<std::io::Stdout>,
    previous_branch_width: u16,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            wip_row: 0,
            selected_row: 0,
            wip_column: 0,
            current_column_coord: 0,
            selected_column: 0,
            row_count: 0,
            column_counts: vec![0],
            buff: std::io::BufWriter::new(stdout()),
            previous_branch_width: 0,
        }
    }

    pub fn clear(&mut self) -> ReposResult<()> {
        self.buff
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        self.wip_row = 0;
        self.wip_column = 0;
        self.column_counts = vec![0];
        Ok(())
    }

    fn is_current_cell_selected(&self) -> bool {
        self.wip_row == self.selected_row && self.wip_column == self.selected_column
    }

    pub fn print(&mut self, text: &str) -> ReposResult<()> {
        self.column_counts[self.wip_row as usize] += 1;

        if self.is_current_cell_selected() {
            self.buff
                .queue(SetBackgroundColor(crossterm::style::Color::Red))?;
        }

        match self.wip_column {
            0 => self.current_column_coord = 0,
            1 => self.current_column_coord += 37,
            2 => self.current_column_coord += 15,
            _ => self.current_column_coord += self.previous_branch_width,
        };

        self.previous_branch_width = text.len() as u16 + 1;

        self.buff
            .queue(MoveToColumn(self.current_column_coord))?
            .queue(Print(text))?;

        if self.is_current_cell_selected() {
            self.buff.queue(crossterm::style::ResetColor)?;
        }

        self.wip_column += 1;

        Ok(())
    }

    pub fn flush(&mut self) -> ReposResult<()> {
        self.buff.flush()?;
        Ok(())
    }

    pub fn new_line(&mut self) -> ReposResult<()> {
        self.buff.queue(MoveToNextLine(1))?.queue(MoveToColumn(0))?;
        self.wip_row += 1;
        self.wip_column = 0;
        self.column_counts.push(0);
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
                if self.selected_column < self.column_counts[self.selected_row as usize] - 1 && self.selected_column < 10{
                        self.selected_column += 1;
                }
            }
        };
    }
}
