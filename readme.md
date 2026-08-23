# Tic Tac Toe

A Tic Tac Toe game written in Rust with a shared game engine that runs on both the Linux desktop and the web.

The project supports:

- Linux desktop gameplay using `eframe` / `egui`
- Browser gameplay using WebAssembly
- Player vs Player
- Player vs Bot
- Easy, Medium, and Hard bot difficulties
- Shared game logic between desktop and WebAssembly
- Last-move highlighting
- Winning-line highlighting
- Game reset

## Project Structure

```text
src/
├── lib.rs
├── game.rs
├── bot/
│   ├── mod.rs
│   ├── easy.rs
│   ├── medium.rs
│   └── hard.rs
├── main.rs
└── wasm.rs

web/
├── index.html
├── app.js
└── style.css
```

The game engine lives in `game.rs`. Each bot difficulty has its own implementation under `src/bot/`.

Both the desktop application and the WebAssembly frontend use the same game engine.

## Prerequisites

Install Rust:

https://www.rust-lang.org/tools/install

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

Verify:

```bash
wasm-pack --version
```

Python 3 is required for the local web development server.

Verify:

```bash
python3 --version
```

## Development

### Check the project

```bash
cargo check
```

### Run tests

```bash
cargo test
```

### Run the Linux desktop application

```bash
cargo run
```

The native application uses `eframe` and `egui`.

## Web Version

The web version uses the same Rust game engine compiled to WebAssembly.

### Build WebAssembly

```bash
wasm-pack build --target web
```

This generates the WebAssembly package in:

```text
pkg/
```

The generated package contains the JavaScript bindings and WebAssembly binary used by the browser.

### Run the Web Version

From the project root:

```bash
python3 -m http.server 8080
```

Then open:

```text
http://localhost:8080/web/
```

The browser loads the generated WebAssembly package from:

```text
../pkg/tictactoe.js
```

## Game Modes

### Player vs Player

Both players make moves manually:

```text
Player X
   vs
Player O
```

### Player vs Bot

The human player plays as X and the bot plays as O:

```text
Player X
   vs
Bot O
```

The game engine automatically makes the bot's move after the human player moves.

The UI does not contain bot logic. This allows the same game and bot implementation to work with both the desktop and WebAssembly versions.

## Bot Difficulties

Bots are separated into their own modules:

```text
src/bot/
├── mod.rs
├── easy.rs
├── medium.rs
└── hard.rs
```

### Easy

The Easy bot chooses the first available position.

### Medium

The Medium bot can:

- Make a winning move
- Block the opponent
- Otherwise choose an available position

### Hard

The Hard bot uses a stronger game strategy to select its moves.

## Shared Game Engine

The UI does not manage the game rules.

The architecture is:

```text
              ┌─────────────────┐
              │   Game Engine   │
              │                 │
              │    game.rs      │
              └────────┬────────┘
                       │
          ┌────────────┴────────────┐
          │                         │
          ▼                         ▼
 ┌─────────────────┐       ┌─────────────────┐
 │ Linux Desktop   │       │ WebAssembly     │
 │                 │       │                 │
 │ eframe / egui   │       │ JavaScript      │
 └─────────────────┘       └─────────────────┘
```

The game engine handles:

- Board state
- Players
- Game modes
- Game status
- Move validation
- Winning conditions
- Draw conditions
- Bot execution
- Bot difficulty

The UI is responsible for displaying the game and sending player input.

## Bot Architecture

Bots implement a common trait:

```rust
pub trait Bot {
    fn make_move(
        &self,
        board: &[Cell; 9],
        player: Player,
    ) -> Option<usize>;
}
```

This allows the game to select a bot dynamically:

```rust
match difficulty {
    BotDifficulty::Easy => Box::new(EasyBot::new()),
    BotDifficulty::Medium => Box::new(MediumBot::new()),
    BotDifficulty::Hard => Box::new(HardBot::new()),
}
```

New bot implementations can therefore be added without changing the desktop or web UI.

## Rebuilding After Rust Changes

Whenever you change the Rust game engine and want to test the web version, rebuild WebAssembly:

```bash
wasm-pack build --target web
```

Then start the web server:

```bash
python3 -m http.server 8080
```

Open:

```text
http://localhost:8080/web/
```

## Desktop Development

Run the desktop application:

```bash
cargo run
```

Check compilation:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

## Web Development

Build WebAssembly:

```bash
wasm-pack build --target web
```

Start the local server:

```bash
python3 -m http.server 8080
```

Open:

```text
http://localhost:8080/web/
```

## License

This project is free to use, modify, and distribute for learning and educational purposes.

Feel free to use the code as a reference for learning Rust, WebAssembly, `egui`, and game development.