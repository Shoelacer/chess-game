use chess::{ChessMove, File, Game, Rank, Square};

fn main() {
    let mut game = Game::new();

    println!("{}", game.current_position().to_string());

    game.make_move(ChessMove::from_san(&game.current_position(), "e4").expect("Not valid"));

    println!("{}", game.current_position().to_string());
}
