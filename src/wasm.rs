use wasm_bindgen::prelude::*;

use crate::game::{Game, GameMode, GameStatus, Player};

#[wasm_bindgen]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new(mode: String) -> WasmGame {
        let game_mode = match mode.as_str() {
            "human_vs_bot" => GameMode::HumanVsBot,
            "human_vs_human" => GameMode::HumanVsHuman,
            _ => GameMode::HumanVsHuman,
        };

        WasmGame {
            game: Game::new_with_mode(game_mode),
        }
    }

    pub fn mode(&self) -> String {
        match self.game.mode() {
            GameMode::HumanVsBot => "human_vs_bot".to_string(),
            GameMode::HumanVsHuman => "human_vs_human".to_string(),
        }
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
            GameStatus::Won(Player::X) => "x_won".to_string(),
            GameStatus::Won(Player::O) => "o_won".to_string(),
            GameStatus::Draw => "draw".to_string(),
        }
    }

    pub fn first_move(&self) -> Option<usize> {
        self.game.first_move(self.game.current_player())
    }

    pub fn winner_position(&self) -> Option<Vec<usize>> {
        self.game
            .winner_position()
            .map(|positions| positions.to_vec())
    }
}