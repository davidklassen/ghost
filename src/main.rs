extern crate ghost;
use ghost::board;
use ghost::game;

fn main() {
    let mut g = game::Game::new(19);

    g.make_move(board::Stone::Black, board::Coords{ x: 15, y: 3 });
    g.make_move(board::Stone::White, board::Coords{ x: 3, y: 15 });
    g.make_move(board::Stone::Black, board::Coords{ x: 15, y: 16 });
    g.make_move(board::Stone::White, board::Coords{ x: 3, y: 3 });

    println!("{}", g.get_board());
}
