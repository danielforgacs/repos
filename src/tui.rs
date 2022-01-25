pub const REPO_NAME_WIDTH: usize = 27;
pub const REPO_STATUS_WIDTH: usize = 9;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Tui {
    pub column: u16,
    column_id: u16,
    pub current_column_id: u16,
    pub row_column_counts: Vec<u16>,
    row: u16,
    pub current_row: u16,
    pub row_count: usize,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            column: 0,
            column_id: 0,
            current_column_id: 0,
            row_column_counts: Vec::new(),
            row: 0,
            current_row: 0,
            row_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.row = 0;
        self.column = 0;
    }

    pub fn row(&self) -> u16 {
        // Plus 1 to skip the dev dir path..
        // Plus 1 to skip the header line.
        self.row + 2
    }

    pub fn finished_row(&mut self) {
        self.column = 0;
        self.column_id = 0;
        self.row += 1;
    }

    pub fn go(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Up => self.go_up(),
            MoveDirection::Down => self.go_down(),
            MoveDirection::Left => self.go_left(),
            MoveDirection::Right => self.go_right(),
        }
    }

    fn go_up(&mut self) {
        if self.current_row > 0 {
            self.current_row -= 1;
        }
        self.validate_current_column()
    }

    fn go_down(&mut self) {
        if self.current_row < self.row_count as u16 - 1 {
            self.current_row += 1;
        }
        self.validate_current_column()
    }

    fn go_right(&mut self) {
        self.current_column_id += 1;
        self.validate_current_column()
    }

    fn go_left(&mut self) {
        if self.current_column_id > 0 {
            self.current_column_id -= 1;
        }
    }

    fn validate_current_column(&mut self) {
        if self.current_column_id > self.row_column_counts[self.current_row as usize] - 1 {
            self.current_column_id = self.row_column_counts[self.current_row as usize] - 1
        }
    }

    pub fn column(&mut self) -> u16 {
        match self.column_id {
            0 => {}
            1 => self.column += REPO_NAME_WIDTH as u16 + 1,
            _ => self.column += REPO_STATUS_WIDTH as u16 + 1,
        };
        self.column_id += 1;
        self.column
    }

    pub fn adjust_column_width(&mut self, width: u16) {
        self.column -= 10;
        self.column += width + 1;
    }

    pub fn is_current_cell(&self) -> bool {
        self.column_id == self.current_column_id + 1 && self.row == self.current_row
    }
}
