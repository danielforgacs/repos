use crate::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum CellStyle {
    Default,
    CurrentBranch,
    CleanMaster,
    DirtyMaster,
    CleanBranch,
    DirtyBranch,
}

pub struct Tui {
    // row that's being currently printed in the loop.
    // this is checked against the selected row.
    // If the wip row == the selected row,
    // the wip row is the selected one.
    wip_row: u16,
    selected_row: u16,
    wip_column: u16,
    wip_column_coord: u16,
    selected_column: u16,
    column_counts: Vec<u16>,
    row_count: u16,
    buff: std::io::BufWriter<std::io::Stdout>,
    previous_column_width: u16,
    pub cell_style: CellStyle,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            wip_row: 0,
            selected_row: 0,
            wip_column: 0,
            wip_column_coord: 0,
            selected_column: 0,
            column_counts: vec![0],
            row_count: 0,
            buff: std::io::BufWriter::new(stdout()),
            previous_column_width: 0,
            cell_style: CellStyle::Default,
        }
    }

    pub fn clear(&mut self) -> ReposResult<()> {
        self.buff
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        self.wip_row = 0;
        self.wip_column = 0;
        self.row_count = 0;
        self.column_counts = vec![0];
        Ok(())
    }

    pub fn print(&mut self, mut text: &str) -> ReposResult<()> {
        match self.wip_column {
            0 => self.wip_column_coord = 0,
            1 => self.wip_column_coord += REPO_NAME_WIDTH,
            _ => {
                let (width, _) = terminal::size()?;
                let test_column_coord = self.wip_column_coord + self.previous_column_width as u16;
                if test_column_coord > width - (text.len() as u16) {
                    self.wip_column_coord = width - 4;
                    text = " >>>";
                } else {
                    self.wip_column_coord = test_column_coord;
                }
            }
        };
        self.previous_column_width = text.len() as u16;
        let cell_gap = 1;
        self.wip_column_coord += cell_gap;
        self.buff.queue(MoveToColumn(self.wip_column_coord))?;
        self.apply_cell_style()?;
        if self.is_cell_selected() {
            self.buff.queue(SetBackgroundColor(Color::Red))?;
        }
        self.buff
            .queue(Print(text))?
            .queue(ResetColor)?;
        self.wip_column += 1;
        self.column_counts[self.wip_row as usize] += 1;
        Ok(())
    }

    pub fn set_cell_style(&mut self, style: CellStyle) {
        self.cell_style = style;
    }

    fn is_cell_selected(&self) -> bool {
        self.wip_column == self.selected_column && self.wip_row == self.selected_row
    }

    fn apply_cell_style(&mut self) -> ReposResult<()> {
        self.buff.queue(ResetColor)?;
        match self.cell_style {
            CellStyle::Default => {
                self.buff.queue(ResetColor)?;
            }
            CellStyle::CurrentBranch => {
                self.buff.queue(SetForegroundColor(Color::Green))?;
                self.cell_style = CellStyle::Default;
            }
            CellStyle::CleanMaster => {
                if self.wip_column < 2 {
                    self.buff.queue(SetForegroundColor(Color::Green))?;
                }
            }
            CellStyle::DirtyMaster => {
                if self.wip_column < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 255, g: 180, b: 0 }))?;
                }
            }
            CellStyle::CleanBranch => {
                if self.wip_column < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 0, g: 200, b: 255 }))?;
                }
            }
            CellStyle::DirtyBranch => {
                if self.wip_column < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }))?;
                }
            }
        };
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
        self.wip_column_coord = 0;
        self.column_counts.push(0);
        self.row_count += 1;
        Ok(())
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
                if self.selected_column < self.column_counts[self.selected_row as usize] - 1
                    && self.selected_column < 10
                {
                    self.selected_column += 1;
                }
            }
        };
        if self.selected_column > self.column_counts[self.selected_row as usize] - 1 {
            self.selected_column = self.column_counts[self.selected_row as usize] - 1;
        }
    }
}
