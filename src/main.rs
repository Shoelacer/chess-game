use std::{panic::PanicHookInfo, str::FromStr};

use chess::{ChessMove, Color, File, Game, Rank, Square};
use tokio::{self};

#[tokio::main]
async fn main() {
    let mut game = Game::new();

    print_board_pretty(&game.current_position());
    while game.result().is_none() {
        print_current_move_info(&game);
        game.make_move(request_move(&game));
        print_board_pretty(&game.current_position());
    }
    println!("{:?}", game.result().unwrap());
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
        } else if i == ' ' {
            println!();
            return;
        } else {
            print!("{i}");
        }
    }
}

fn print_current_move_info(game: &Game) {
    if game.side_to_move() == Color::White {
        println!("White's Turn");
    } else {
        println!("Black's Turn");
    }
    if (game.current_position().checkers().popcnt() > 0) {
        println!("Currently in check");
    }
}

fn request_move(game: &Game) -> ChessMove {
    let mut input = String::new();
    let mut move_coords = "";

    while ChessMove::from_san(&game.current_position(), move_coords).is_err() {
        input.clear();
        println!("Enter a valid move: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read the move");
        move_coords = input.trim_end();
    }
    println!(
        "Move: {}",
        ChessMove::from_san(&game.current_position(), move_coords).unwrap()
    );

    ChessMove::from_san(&game.current_position(), &move_coords).unwrap()
}
