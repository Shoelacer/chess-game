use chess::{ChessMove, File, Game, Rank, Square};

fn main() {
    let mut game = Game::new();

    println!("{}", game.current_position().to_string());

    game.make_move(ChessMove::from_san(&game.current_position(), "e4").expect("Not valid"));

    print_board_pretty(&game.current_position());
}

fn print_board_pretty(board: &chess::Board) {
    let board = board.to_string();
    for i in board.chars() {
        if let Ok(i) = i.to_string().parse() {
            for _ in 0..i {
                print!("*");
            }
        } else if i == '/' {
            println!();
        } else {
            print!("{i}");
        }
    }
}
