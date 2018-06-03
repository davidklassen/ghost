extern crate ghost;
use ghost::board;

fn make_move(b: &mut board::Board, stone: board::Stone, coords: board::Coords) {
    b.set_slot(board::SlotState::Stone(stone), coords);
}

fn main() {
    let b = &mut board::Board::new(19);

    make_move(b, board::Stone::Black, board::Coords{ x: 15, y: 3 });
    make_move(b, board::Stone::White, board::Coords{ x: 3, y: 15 });
    make_move(b, board::Stone::Black, board::Coords{ x: 15, y: 16 });
    make_move(b, board::Stone::White, board::Coords{ x: 3, y: 3 });

    println!("{}", b);
}
