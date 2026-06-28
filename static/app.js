const FILES = "abcdefgh";
const ws = new WebSocket("ws://10.0.0.52:8080");

let move = "";

ws.onopen = () => {
  console.log("Connected to Rust Server!");
};
ws.onmessage = (event) => {
  console.log(event.data);
  /*  if (event.data == "Enter Username") {
    let username = prompt("Enter Username to continue: ");
    ws.send(JSON.stringify({ username }));
  } else {
    const data = JSON.parse(event.data);
    addMessage(data.user, data.text);
  }*/
  const data = JSON.parse(event.data);
  if (data.type == "board") {
    let board = "";
    let fen = data.fen;
    for (let i = 0; i < fen.length; i++) {
      if (fen.charAt(i) >= "0" && fen.charAt(i) <= "9") {
        board += " ".repeat(parseInt(fen.charAt(i)));
      } else if (fen.charAt(i) === "/") {
      } else if (fen.charAt(i) === " ") {
        i = fen.length;
      } else {
        board += fenPieceToUnicode(fen.charAt(i));
      }
    }

    console.log(fen);
    console.log(board);
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        let cell = document
          .getElementById("chessboard")
          .children.item(0)
          .children.item(j)
          .children.item(i);
        cell.setHTMLUnsafe(board.charAt(j * 8 + i));
      }
    }
  }
};

for (let i = 0; i < 8; i++) {
  for (let j = 1; j <= 8; j++) {
    let cell = document
      .getElementById("chessboard")
      .children.item(0)
      .children.item(j - 1)
      .children.item(i);
    cell.id = FILES.charAt(i) + (8 - j + 1);
    console.log(FILES.charAt(8 - i - 1) + j);
    cell.addEventListener("click", () => {
      console.log(cell.id);
      if (move != "") {
        if (
          document.getElementById(move).getHTML() == "♟" ||
          document.getElementById(move).getHTML() == "♙"
        ) {
          if (cell.id.substring(1) == "8" || cell.id.substring(1) == "1") {
            let promote = prompt("What piece you want? (r/n/b/q)");
            move += cell.id + promote;
          } else {
            move += cell.id;
          }
        } else {
          move += cell.id;
        }
      } else {
        move += cell.id;
      }
      console.log(move);
      if (move.length >= 4) {
        ws.send(move);
        move = "";
      }
    });
  }
}

function fenPieceToUnicode(piece) {
  if (piece === "K") return "♔";
  if (piece === "Q") return "♕";
  if (piece === "R") return "♖";
  if (piece === "B") return "♗";
  if (piece === "N") return "♘";
  if (piece === "P") return "♙";
  if (piece === "k") return "♚";
  if (piece === "q") return "♛";
  if (piece === "r") return "♜";
  if (piece === "b") return "♝";
  if (piece === "n") return "♞";
  if (piece === "p") return "♟";
}
