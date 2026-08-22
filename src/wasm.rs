use wasm_bindgen::prelude::*;

use crate::game::{Game, GameStatus, Player};

#[wasm_bindgen]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame {
            game: Game::new(),
        }
    }
    pub fn last_move(&self) -> Option<usize> {
        self.game.last_move(self.game.current_player())
    }

    pub fn winner_position(&self) -> Option<Vec<u32>> {
        self.game.winner_position()
            .map(|positions| {
                positions
                    .into_iter()
                    .map(|position| position as u32)
                    .collect()
            })
    }
    
    pub fn make_move(&mut self, position: usize) -> bool {
        self.game.make_move(position)
    }

    pub fn board(&self) -> String {
        self.game
            .board()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    pub fn current_player(&self) -> String {
        match self.game.current_player() {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }

    pub fn status(&self) -> String {
        match self.game.status() {
            GameStatus::InProgress => "in_progress".to_string(),
            GameStatus::Draw => "draw".to_string(),
            GameStatus::Won(Player::X) => "x_won".to_string(),
            GameStatus::Won(Player::O) => "o_won".to_string(),
        }
    }
}