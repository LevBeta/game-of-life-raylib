pub struct GameOfLife {
    pub board: Vec<Vec<bool>>
}

impl GameOfLife {
    pub fn new(widht: usize, height: usize) -> Self {
        Self {
            board: vec![vec![false; widht]; height]
        }
    }

    pub fn random_cells(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rng.gen::<bool>();
            }
        }
    }

    pub fn get_live_neighbors(&self, row: usize, col: usize, board: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
        let mut live_neighbors = Vec::new();
        
        let max_row = board.len();
        let max_col = board[0].len();
        for d_row in -1..=1 {
            for d_col in -1..=1 {
                if d_row == 0 && d_col == 0 {
                    continue;
                }
    
                let new_row = (row as isize) + d_row;
                let new_col = (col as isize) + d_col;
                
                if new_row >= 0 && new_row < max_row as isize &&
                   new_col >= 0 && new_col < max_col as isize &&
                   board[new_row as usize][new_col as usize] {
                    live_neighbors.push((new_row as usize, new_col as usize));
                }
            }
        }
        
        live_neighbors
    }

    pub fn next_seed(&mut self) {
        let board_copy = self.board.clone();
        for x in 0..board_copy.len() {
            for y in 0..board_copy[0].len() {
                let live_neighbors = self.get_live_neighbors(x, y, &board_copy);
                let should_live = match (board_copy[x][y], live_neighbors.len()) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                self.board[x][y] = should_live;
            }
        }
    }
}