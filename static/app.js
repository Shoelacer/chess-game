const FILES = "abcdefgh";

for (let i = 0; i < 8; i++) {
  for (let j = 1; j < 8; j++) {
    let cell = document
      .getElementById("chessboard")
      .children.item(0)
      .children.item(j - 1)
      .children.item(i);
    cell.id = FILES.charAt(i) + (8 - j + 1);
    console.log(FILES.charAt(8 - i - 1) + j);
    cell.addEventListener("click", () => {
      console.log(cell.id);
    });
  }
}
