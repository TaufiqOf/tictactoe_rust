use crate::bot::Bot;
use crate::game::{Cell, Player};

pub struct EasyBot;

impl EasyBot {
    pub fn new() -> Self {
        Self
    }
}

impl Bot for EasyBot {
    fn make_move(
        &self,
        board: &[Cell; 9],
        _player: Player,
    ) -> Option<usize> {
        board
            .iter()
            .position(|cell| matches!(cell, Cell::Empty))
    }
}