use crate::bot::{Bot, EasyBot, HardBot, MediumBot};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    HumanVsHuman,
    HumanVsBot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    InProgress,
    Draw,
    Won(Player),
}

#[derive(Debug)]
struct Board {
    x_moves: Vec<usize>,
    o_moves: Vec<usize>,
    cells: [Cell; 9],
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    current_player: Player,
    status: GameStatus,
    mode: GameMode,
    bot_difficulty: BotDifficulty,
}

impl Cell {
    pub fn symbol(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            o_moves: Vec::new(),
            x_moves: Vec::new(),
        }
    }

    fn make_move(&mut self, position: usize, player: Player) -> bool {
        if position >= 9 {
            return false;
        }

        let occupied_by_opponent = match (&self.cells[position], player) {
            (Cell::X, Player::O) => true,
            (Cell::O, Player::X) => true,
            _ => false,
        };

        if occupied_by_opponent {
            return false;
        }

        let oldest_move = match player {
            Player::X if self.x_moves.len() >= 3 => {
                Some(self.x_moves.remove(0))
            }
            Player::O if self.o_moves.len() >= 3 => {
                Some(self.o_moves.remove(0))
            }
            _ => None,
        };

        if let Some(oldest_move) = oldest_move {
            self.cells[oldest_move] = Cell::Empty;
        }

        self.cells[position] = match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };

        match player {
            Player::X => self.x_moves.push(position),
            Player::O => self.o_moves.push(position),
        }

        true
    }

    fn winner(&self) -> Option<Player> {
        let positions = self.winner_position()?;

        match self.cells[positions[0]] {
            Cell::X => Some(Player::X),
            Cell::O => Some(Player::O),
            Cell::Empty => None,
        }
    }

    fn winner_position(&self) -> Option<[usize; 3]> {
        let winning_lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in winning_lines {
            match (
                &self.cells[line[0]],
                &self.cells[line[1]],
                &self.cells[line[2]],
            ) {
                (Cell::X, Cell::X, Cell::X)
                | (Cell::O, Cell::O, Cell::O) => {
                    return Some(line);
                }
                _ => {}
            }
        }

        None
    }

    fn is_draw(&self) -> bool {
        if self.winner().is_some() {
            return false;
        }

        self.cells
            .iter()
            .all(|cell| !matches!(cell, Cell::Empty))
    }

    fn display(&self) {
        println!(
            "{} | {} | {}",
            self.cells[0].symbol(),
            self.cells[1].symbol(),
            self.cells[2].symbol()
        );

        println!("---+---+---");

        println!(
            "{} | {} | {}",
            self.cells[3].symbol(),
            self.cells[4].symbol(),
            self.cells[5].symbol()
        );

        println!("---+---+---");

        println!(
            "{} | {} | {}",
            self.cells[6].symbol(),
            self.cells[7].symbol(),
            self.cells[8].symbol()
        );
    }
}

impl Game {
    pub fn new() -> Game {
        Self::new_with_mode_and_difficulty(
            GameMode::HumanVsHuman,
            BotDifficulty::Hard,
        )
    }

    pub fn new_with_mode(mode: GameMode) -> Game {
        Self::new_with_mode_and_difficulty(
            mode,
            BotDifficulty::Hard,
        )
    }

    pub fn new_with_mode_and_difficulty(
        mode: GameMode,
        bot_difficulty: BotDifficulty,
    ) -> Game {
        Game {
            board: Board::new(),
            current_player: Player::X,
            status: GameStatus::InProgress,
            mode,
            bot_difficulty,
        }
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    pub fn bot_difficulty(&self) -> BotDifficulty {
        self.bot_difficulty
    }

    pub fn winner_position(&self) -> Option<[usize; 3]> {
        self.board.winner_position()
    }

    pub fn first_move(&self, player: Player) -> Option<usize> {
        match player {
            Player::X => {
                if self.board.x_moves.len() < 3 {
                    None
                } else {
                    self.board.x_moves.first().copied()
                }
            }

            Player::O => {
                if self.board.o_moves.len() < 3 {
                    None
                } else {
                    self.board.o_moves.first().copied()
                }
            }
        }
    }

    pub fn last_move(&self, player: Player) -> Option<usize> {
        match player {
            Player::X => self.board.x_moves.last().copied(),
            Player::O => self.board.o_moves.last().copied(),
        }
    }

    pub fn board(&self) -> &[Cell; 9] {
        &self.board.cells
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn display(&self) {
        self.board.display();
    }

    pub fn make_move(&mut self, position: usize) -> bool {
        if !self.make_player_move(position) {
            return false;
        }

        self.play_bot_if_needed();

        true
    }

    fn make_player_move(&mut self, position: usize) -> bool {
        if self.status != GameStatus::InProgress {
            return false;
        }

        if !self.board.make_move(position, self.current_player) {
            return false;
        }

        self.update_status();

        if self.status != GameStatus::InProgress {
            return true;
        }

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        true
    }

    fn update_status(&mut self) {
        if let Some(player) = self.board.winner() {
            self.status = GameStatus::Won(player);
            return;
        }

        if self.board.is_draw() {
            self.status = GameStatus::Draw;
        }
    }

    fn play_bot_if_needed(&mut self) {
        if self.mode != GameMode::HumanVsBot {
            return;
        }

        if self.current_player != Player::O {
            return;
        }

        if self.status != GameStatus::InProgress {
            return;
        }

        let bot = Self::create_bot(self.bot_difficulty);

        let Some(position) = bot.make_move(self.board(), self.current_player) else {
            return;
        };

        self.make_player_move(position);
    }

    fn create_bot(difficulty: BotDifficulty) -> Box<dyn Bot> {
        match difficulty {
            BotDifficulty::Easy => Box::new(EasyBot::new()),
            BotDifficulty::Medium => Box::new(MediumBot::new()),
            BotDifficulty::Hard => Box::new(HardBot::new()),
        }
    }

    pub fn find_winning_move(&self, player: Player) -> Option<usize> {
        let winning_lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in winning_lines {
            let mut player_count = 0;
            let mut empty_position = None;

            for &position in &line {
                match (&self.board.cells[position], player) {
                    (Cell::Empty, _) => {
                        empty_position = Some(position);
                    }

                    (Cell::X, Player::X)
                    | (Cell::O, Player::O) => {
                        player_count += 1;
                    }

                    _ => {}
                }
            }

            if player_count == 2 {
                if let Some(position) = empty_position {
                    return Some(position);
                }
            }
        }

        None
    }
}