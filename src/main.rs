use chess::{ChessMove, Color, File, Game, GameResult, Rank, Square};
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::{panic::PanicHookInfo, str::FromStr};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::{self, net::TcpListener};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on ws://{}", addr);

    let (stream1, addr1) = listener.accept().await?;
    println!("Player 1 connected: {}", addr1);

    let mut ws1 = accept_async(stream1).await?;
    ws1.send(WsMessage::text(
        json!({"type": "welcome","color":"white"}).to_string(),
    ))
    .await
    .unwrap();
    ws1.send(WsMessage::text(
        json!({"type": "status","text":"Waiting for Player Two..."}).to_string(),
    ))
    .await
    .unwrap();

    let (stream2, addr2) = listener.accept().await?;
    println!("Player 2 connected: {}", addr2);

    let mut ws2 = accept_async(stream2).await?;
    ws2.send(WsMessage::text(
        json!({"type": "welcome","color":"black"}).to_string(),
    ))
    .await
    .unwrap();

    play_game(ws1, ws2).await;

    Ok(())
}

async fn play_game(
    mut player_one: WebSocketStream<TcpStream>,
    mut player_two: WebSocketStream<TcpStream>,
) {
    let mut game = Game::new();
    print_board_pretty(&game.current_position());
    while game.result().is_none() {
        print_current_move_info(&game);
        if game.side_to_move() == Color::White {
            game.make_move(request_move(&game, &mut player_one).await);
        } else {
            game.make_move(request_move(&game, &mut player_two).await);
        }
        print_board_pretty(&game.current_position());
        player_one
            .send(WsMessage::text(
                json!({"type": "board", "fen": game.current_position().to_string()}).to_string(),
            ))
            .await
            .unwrap();
        player_two
            .send(WsMessage::text(
                json!({"type": "board", "fen": game.current_position().to_string()}).to_string(),
            ))
            .await
            .unwrap();
    }

    let mut game_result = String::new();
    if game.result().unwrap() == GameResult::WhiteCheckmates
        || game.result().unwrap() == GameResult::BlackResigns
    {
        game_result.push_str("White Wins!");
    } else if game.result().unwrap() == GameResult::BlackCheckmates
        || game.result().unwrap() == GameResult::WhiteResigns
    {
        game_result.push_str("Black Wins!");
    } else {
        game_result.push_str("Draw!");
    }

    println!("{:?}", game.result().unwrap());

    player_one
        .send(WsMessage::text(
            json!({"type": "result", "result":game_result}).to_string(),
        ))
        .await
        .unwrap();
    player_two
        .send(WsMessage::text(
            json!({"type": "result", "result":game_result}).to_string(),
        ))
        .await
        .unwrap();
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

async fn request_move(game: &Game, player: &mut WebSocketStream<TcpStream>) -> ChessMove {
    while let Some(msg) = player.next().await {
        let msg = msg.unwrap();

        if let WsMessage::Text(text) = msg {
            println!("Received move: {}", text);
            if let Ok(mv) = ChessMove::from_san(&game.current_position(), text.trim()) {
                return mv;
            }
            if let Ok(mv) = ChessMove::from_str(text.trim()) {
                return mv;
            }

            player
                .send(WsMessage::text(
                    json!({"type": "error", "message": "Illegal move"}).to_string(),
                ))
                .await
                .unwrap();
        }
    }

    panic!("Client disconnected");
}
