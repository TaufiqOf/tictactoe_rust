# Tic Tac Toe

A Tic Tac Toe game written in Rust, compiled to WebAssembly for browser gameplay.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.org/wasm-pack/installer/)
- Python 3 (for local development server)

## Installation

Install wasm-pack:
```bash
cargo install wasm-pack
```

Verify installation:
```bash
wasm-pack --version
```

## Development

Check the code compiles:
```bash
cargo check
```

Run tests:
```bash
cargo test
```

Run the native binary:
```bash
cargo run
```

## Building for Web

Build the WebAssembly package for the browser:
```bash
wasm-pack build --target web
```

This generates the compiled WebAssembly files in the `pkg/` directory.

## Running Locally

Start a local development server:
```bash
python3 -m http.server 8080
```

Then open your browser to `http://localhost:8080`
