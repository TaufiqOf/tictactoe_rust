import init, { WasmGame } from "../pkg/tictactoe.js";

let game;

async function main() {
    await init();

    game = new WasmGame();

    render();
}

function render() {
    renderBoard();
    renderStatus();
}

function renderBoard() {
    const boardElement = document.getElementById("board");

    boardElement.innerHTML = "";

    const board = game.board();

    for (let position = 0; position < 9; position++) {
        const cell = document.createElement("button");

        cell.className = "cell";

        cell.textContent = board[position];

        cell.addEventListener("click", () => {
            makeMove(position);
        });

        boardElement.appendChild(cell);
    }
}

function renderStatus() {
    const statusElement = document.getElementById("status");

    const status = game.status();
    const player = game.current_player();

    if (status === "in_progress") {
        statusElement.textContent = `Player ${player}'s turn`;
    } else if (status === "x_won") {
        statusElement.textContent = "Player X wins!";
    } else if (status === "o_won") {
        statusElement.textContent = "Player O wins!";
    } else if (status === "draw") {
        statusElement.textContent = "It's a draw!";
    }
}

function makeMove(position) {
    const success = game.make_move(position);

    if (!success) {
        return;
    }

    render();
}

document.getElementById("reset").addEventListener("click", () => {
    game = new WasmGame();

    render();
});

main();