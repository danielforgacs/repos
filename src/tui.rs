const COL_OFFSET: u16 = 0;
const ROW_OFFSET: u16 = 1;

use crate::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum Column {
    Name,
    Status,
    Branches,
}

#[derive(PartialEq)]
pub struct CellCoord {
    column: u16,
    row: u16,
}

impl CellCoord {
    fn new() -> Self {
        Self {
            column: 0,
            row: 0,
        }
    }

    fn reset(&mut self) {
        self.column = 0;
        self.row = 0;
    }

    pub fn get_column(&self) -> u16 {
        self.column
    }

    pub fn get_row(&self) -> u16 {
        self.row
    }

    fn inc_column(&mut self) {
        self.column += 1;
    }

    fn dec_column(&mut self) {
        self.column -= 1;
    }

    fn inc_row(&mut self) {
        self.row += 1;
    }

    fn dec_row(&mut self) {
        self.row -= 1;
    }

    fn reset_column(&mut self) {
        self.column = 0;
    }

    fn limit_column(&mut self, max: u16) {
        if self.column > max {
            self.column = max;
        }
    }
}

pub trait ToColumn {
    fn to_column(&self) -> Column;
}

impl ToColumn for u16 {
    fn to_column(&self) -> Column {
        match self {
            0 => Column::Name,
            1 => Column::Status,
            _ => Column::Branches,
        }
    }
}

pub enum CellStyle {
    Default,
    CurrentBranch,
    Branch,
    CleanMaster,
    DirtyMaster,
    CleanBranch,
    DirtyBranch,
    Info,
}

pub struct Tui {
    // row that's being currently printed in the loop.
    // this is checked against the selected row.
    // If the wip row == the selected row,
    // the wip row is the selected one.
    wip_cell: CellCoord,
    selected_cell: CellCoord,
    wip_column_coord: u16,
    column_counts: Vec<u16>,
    row_count: u16,
    buff: std::io::BufWriter<std::io::Stdout>,
    previous_column_width: u16,
    pub cell_style: CellStyle,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            wip_cell: CellCoord::new(),
            selected_cell: CellCoord::new(),
            wip_column_coord: 0,
            column_counts: vec![0],
            row_count: 0,
            buff: std::io::BufWriter::new(stdout()),
            previous_column_width: 0,
            cell_style: CellStyle::Default,
        }
    }

    pub fn selected_coord(&self) -> &CellCoord {
        &self.selected_cell
    }

    pub fn clear(&mut self) -> ReposResult<()> {
        self.wip_cell.reset();
        self.buff
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(self.wip_cell.get_row(), self.wip_cell.get_column()))?;
        self.row_count = 0;
        self.column_counts = vec![0];
        self.buff.queue(crossterm::cursor::MoveToNextLine(ROW_OFFSET))?;
        Ok(())
    }

    fn calc_wip_column_coord<'a>(&self, text: &'a str) -> (u16, &'a str) {
        match self.wip_cell.get_column().to_column() {
            Column::Name => (0 + COL_OFFSET, text),
            Column::Status => (self.wip_column_coord + REPO_NAME_WIDTH + COL_OFFSET, text),
            Column::Branches => {
                let (width, _) = terminal::size().unwrap();
                let test_column_coord = self.wip_column_coord + self.previous_column_width as u16;
                let test_column_coord = test_column_coord + COL_OFFSET;
                if test_column_coord > width - (text.len() as u16) {
                    (width - 5, " >>>")
                } else {
                    (test_column_coord, text)
                }
            }
        }
    }

    pub fn print(&mut self, mut text: &str) -> ReposResult<()> {
        (self.wip_column_coord, text) = self.calc_wip_column_coord(text);
        self.previous_column_width = text.len() as u16;
        let cell_gap = 1;
        self.wip_column_coord += cell_gap;
        self.buff.queue(MoveToColumn(self.wip_column_coord))?;
        self.apply_cell_style()?;
        if self.wip_cell == self.selected_cell {
            self.buff.queue(SetBackgroundColor(Color::Rgb { r: 90, g: 15, b: 0 }))?;
        }
        self.buff
            .queue(Print(text))?
            // Just to fill the gap between columns
            .queue(Print(" "))?
            .queue(ResetColor)?;
        self.wip_cell.inc_column();
        self.column_counts[usize::from(self.wip_cell.get_row())] += 1;
        Ok(())
    }

    pub fn set_cell_style(&mut self, style: CellStyle) {
        self.cell_style = style;
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
            CellStyle::Branch => {
                self.buff.queue(SetForegroundColor(Color::Rgb { r: 95, g: 85, b: 80 }))?;
                self.cell_style = CellStyle::Default;
            }
            CellStyle::CleanMaster => {
                if self.wip_cell.get_column() < 2 {
                    self.buff.queue(SetForegroundColor(Color::Green))?;
                }
            }
            CellStyle::DirtyMaster => {
                if self.wip_cell.get_column() < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 255, g: 205, b: 0 }))?;
                }
            }
            CellStyle::CleanBranch => {
                if self.wip_cell.get_column() < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 0, g: 200, b: 255 }))?;
                }
            }
            CellStyle::DirtyBranch => {
                if self.wip_cell.get_column() < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }))?;
                }
            }
            CellStyle::Info => {
                if self.wip_cell.get_column() < 2 {
                    self.buff.queue(SetForegroundColor(Color::Rgb { r: 80, g: 80, b: 80 }))?;
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
        self.wip_cell.inc_row();
        self.wip_cell.reset_column();
        self.wip_column_coord = 0;
        self.column_counts.push(0);
        self.row_count += 1;
        Ok(())
    }

    pub fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.selected_cell.get_row() > 0 {
                    self.selected_cell.dec_row();
                }
            }
            Direction::Down => {
                if self.selected_cell.get_row() < self.row_count - 1 {
                    self.selected_cell.inc_row()
                }
            }
            Direction::Left => {
                if self.selected_cell.get_column() > 0 {
                    self.selected_cell.dec_column()
                }
            }
            Direction::Right => {
                if self.selected_cell.get_column() < self.column_counts[self.selected_cell.get_row() as usize] - 1 {
                    self.selected_cell.inc_column();
                }
            }
        };
        self.set_max_selected_column(self.column_counts[self.selected_cell.get_row() as usize] - 1);
    }

    pub fn set_max_selected_column(&mut self, max: u16) {
        if self.selected_cell.get_column() > max {
            self.selected_cell.limit_column(max);
        }
    }

    pub fn print_status(&mut self, repo_name: &str, current_branch: &str, selected_cell_branch: &str) -> ReposResult<()> {
        self.buff
            .queue(MoveTo(0, self.row_count + 1))?
            .queue(Print(repo_name))?
            .queue(MoveToColumn(REPO_NAME_WIDTH + COL_OFFSET + 2))?
            .queue(Print("|"))?
            .queue(Print(current_branch))?
            .queue(Print("|"))?
            .queue(Print(selected_cell_branch))?;
        Ok(())
    }

    pub fn print_dev_dir(&mut self, path: &str) -> ReposResult<()> {
        self.set_cell_style(CellStyle::Info);
        self.apply_cell_style()?;
        self.buff
            .queue(MoveTo(0, 0))?
            .queue(Print(path))?;
        Ok(())
    }
}
