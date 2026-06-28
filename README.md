# Chess Game — Rust with WebSockets
A two-player chess server built in Rust with WebSocket communication. Players connect from any WebSocket client (browser, terminal, or custom client) and play a full game of chess with move validation and check detection.

## Features
* Full chess rules (move validation, check, checkmate, stalemate)
* WebSocket communication
* Move updates on both sides
* Game result notification (win/loss/draw)
* Compatible with terminal clients using SAN.


## Running the Server
The server can be run with cargo run and the clients can be accessed after hosting them on python or some other platform. A custom terminal client can also be used if the user wishes to play from the terminal (for example websocat).

This project uses the chess, tokio, serde, tokio_tungstenite, and futures_util crates along with teh standard library. 

## Skills Learned
* Building a WebSocket server with Tokio
* Building the frontend with vanilla Javascript
* Validating moves using a chess library
* Communicating between two clients through a server

