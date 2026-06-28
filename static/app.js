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
      console.log("next" + i);
      if (fen.charAt(i) >= "0" && fen.charAt(i) <= "9") {
        board += " ".repeat(parseInt(fen.charAt(i)));
        console.log("space");
      } else if (fen.charAt(i) === "/") {
        console.log("line");
      } else if (fen.charAt(i) === " ") {
        console.log("done");
        i = fen.length;
      } else {
        console.log("piece");
        board += fen.charAt(i);
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
      move += cell.id;
      console.log(move);
      if (move.length == 4) {
        ws.send(move);
        move = "";
      }
    });
  }
}
