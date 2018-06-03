use super::board;

pub struct Game {
    board: board::Board,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game{ board: board::Board::new(size) }
    }

    pub fn make_move(&mut self, stone: board::Stone, coords: board::Coords) {
        self.board.set_slot(board::SlotState::Stone(stone), coords);
    }

    pub fn get_board(&self) -> &board::Board {
        &self.board
    }
}
