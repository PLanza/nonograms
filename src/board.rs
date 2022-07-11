#[derive(Clone, Copy, Debug)]
pub enum CellState {
    BLANK = 0,
    FILLED = 1,
    CROSSED = 2,
}

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    state: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Board {
        let mut state = Vec::new();
        state.resize(rows, Vec::new());

        for i in 0..state.len() {
            state[i].resize(cols, CellState::BLANK);
        }

        Board {
            rows,
            cols,
            state: state.clone(),
        }
    }

    pub fn get(&self, col: usize, row: usize) -> CellState {
        self.state[row][col]
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn toggle_fill_at(&mut self, position: (usize, usize)) {
        let (col, row) = position;
        self.state[row][col] = match self.state[row][col] {
            CellState::FILLED => CellState::BLANK,
            _ => CellState::FILLED,
        };
    }

    pub fn toggle_cross_at(&mut self, position: (usize, usize)) {
        let (col, row) = position;
        self.state[row][col] = match self.state[row][col] {
            CellState::CROSSED => CellState::BLANK,
            _ => CellState::CROSSED,
        };
    }
}
