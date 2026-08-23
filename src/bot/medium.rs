use crate::bot::Bot;
use crate::game::{Cell, Player};

pub struct MediumBot;

impl MediumBot {
    pub fn new() -> Self {
        Self
    }

    fn winning_move(
        &self,
        board: &[Cell; 9],
        player: Player,
    ) -> Option<usize> {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in lines {
            let mut empty = None;
            let mut count = 0;

            for &position in &line {
                match (&board[position], player) {
                    (Cell::Empty, _) => {
                        empty = Some(position);
                    }

                    (Cell::X, Player::X)
                    | (Cell::O, Player::O) => {
                        count += 1;
                    }

                    _ => {}
                }
            }

            if count == 2 {
                if let Some(position) = empty {
                    return Some(position);
                }
            }
        }

        None
    }

    fn opponent(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Bot for MediumBot {
    fn make_move(
        &self,
        board: &[Cell; 9],
        player: Player,
    ) -> Option<usize> {
        // Try to win.
        if let Some(position) = self.winning_move(board, player) {
            return Some(position);
        }

        // Try to block the opponent.
        let opponent = Self::opponent(player);

        if let Some(position) = self.winning_move(board, opponent) {
            return Some(position);
        }

        // Otherwise choose the first empty cell.
        board
            .iter()
            .position(|cell| matches!(cell, Cell::Empty))
    }
}