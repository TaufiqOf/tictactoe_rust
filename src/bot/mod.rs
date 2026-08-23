mod easy;
mod medium;
mod hard;

pub use easy::EasyBot;
pub use medium::MediumBot;
pub use hard::HardBot;

use crate::game::{Cell, Player};

pub trait Bot {
    fn make_move(
        &self,
        board: &[Cell; 9],
        player: Player,
    ) -> Option<usize>;
}