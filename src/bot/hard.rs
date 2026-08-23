use crate::bot::Bot;
use crate::game::{Cell, Player};

pub struct HardBot;

impl HardBot {
    pub fn new() -> Self {
        Self
    }

    fn opponent(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn winner(board: &[Cell; 9]) -> Option<Player> {
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
            match (
                &board[line[0]],
                &board[line[1]],
                &board[line[2]],
            ) {
                (Cell::X, Cell::X, Cell::X) => {
                    return Some(Player::X);
                }

                (Cell::O, Cell::O, Cell::O) => {
                    return Some(Player::O);
                }

                _ => {}
            }
        }

        None
    }

    fn minimax(
        board: &mut [Cell; 9],
        bot: Player,
        current: Player,
        depth: i32,
    ) -> i32 {
        if let Some(winner) = Self::winner(board) {
            if winner == bot {
                return 10 - depth;
            }

            return depth - 10;
        }

        let empty_positions: Vec<usize> = board
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| {
                if matches!(cell, Cell::Empty) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        if empty_positions.is_empty() {
            return 0;
        }

        let opponent = Self::opponent(current);

        if current == bot {
            let mut best = i32::MIN;

            for position in empty_positions {
                board[position] = match bot {
                    Player::X => Cell::X,
                    Player::O => Cell::O,
                };

                let score =
                    Self::minimax(board, bot, opponent, depth + 1);

                board[position] = Cell::Empty;

                best = best.max(score);
            }

            best
        } else {
            let mut best = i32::MAX;

            for position in empty_positions {
                board[position] = match current {
                    Player::X => Cell::X,
                    Player::O => Cell::O,
                };

                let score =
                    Self::minimax(board, bot, opponent, depth + 1);

                board[position] = Cell::Empty;

                best = best.min(score);
            }

            best
        }
    }
}

impl Bot for HardBot {
    fn make_move(
        &self,
        board: &[Cell; 9],
        player: Player,
    ) -> Option<usize> {
        let mut board = board.clone();

        let mut best_position = None;
        let mut best_score = i32::MIN;

        for position in 0..9 {
            if !matches!(board[position], Cell::Empty) {
                continue;
            }

            board[position] = match player {
                Player::X => Cell::X,
                Player::O => Cell::O,
            };

            let score = Self::minimax(
                &mut board,
                player,
                Self::opponent(player),
                0,
            );

            board[position] = Cell::Empty;

            if score > best_score {
                best_score = score;
                best_position = Some(position);
            }
        }

        best_position
    }
}