import init, { WasmGame } from "../pkg/tictactoe.js";

let game;
let mode = "human_vs_bot";

async function main() {
    await init();

    createGame();
    setupEvents();
    render();
}

function createGame() {
    game = new WasmGame(mode);
}

function setupEvents() {
    document
        .getElementById("reset")
        .addEventListener("click", resetGame);

    document
        .querySelectorAll('input[name="game-mode"]')
        .forEach((input) => {
            input.addEventListener("change", () => {
                mode = input.value;

                createGame();
                render();
            });
        });
}

function resetGame() {
    createGame();
    render();
}

function render() {
    renderMode();
    renderBoard();
    renderStatus();
}

function renderMode() {
    const modeElement = document.getElementById("selected-mode");

    if (game.mode() === "human_vs_bot") {
        modeElement.textContent = "Mode: Player vs Bot";
    } else {
        modeElement.textContent = "Mode: Player vs Player";
    }
}

function renderBoard() {
    const boardElement = document.getElementById("board");

    boardElement.innerHTML = "";

    const board = game.board();
    const lastMove = game.first_move();
    const winnerPositions = game.winner_position() ?? [];
    const gameOver = game.status() !== "in_progress";

    for (let position = 0; position < 9; position++) {
        const cell = document.createElement("button");

        cell.className = "cell";

        if (winnerPositions.includes(position)) {
            cell.classList.add("winner");
        } else if (position === lastMove) {
            cell.classList.add("last-move");
        }

        cell.textContent = board[position];

        cell.disabled =
            gameOver ||
            board[position] !== " ";

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
    if (!game.make_move(position)) {
        return;
    }

    render();
}

main();