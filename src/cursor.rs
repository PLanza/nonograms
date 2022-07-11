pub struct Cursor {
    pub col: usize,
    pub row: usize,
    max_col: usize,
    max_row: usize,
}

impl Cursor {
    pub fn new(board_size: (usize, usize)) -> Cursor {
        Cursor {
            col: 0,
            row: 0,
            max_col: board_size.0,
            max_row: board_size.1,
        }
    }

    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.row < self.max_row - 1 {
            self.row += 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.col < self.max_col - 1 {
            self.col += 1;
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.col, self.row)
    }
}
