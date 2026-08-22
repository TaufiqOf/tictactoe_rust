#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Debug, PartialEq)]
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
            o_moves : Vec::new(),
            x_moves : Vec::new(),
        }
    }
    fn make_move(&mut self, position: usize, player: Player) -> bool {
        if position >= 9 {
            return false;
        }

        let moves = match player {
            Player::X => &self.x_moves,
            Player::O => &self.o_moves,
        };

        // If the cell is occupied by the opponent, reject the move.
        let occupied_by_opponent = match (&self.cells[position], player) {
            (Cell::X, Player::O) => true,
            (Cell::O, Player::X) => true,
            _ => false,
        };

        if occupied_by_opponent {
            return false;
        }

        // Remove oldest move if this is the player's 4th move.
        if moves.len() >= 3 {
            let oldest_move = moves[0];

            match player {
                Player::X => {
                    self.x_moves.remove(0);
                }
                Player::O => {
                    self.o_moves.remove(0);
                }
            }

            self.cells[oldest_move] = Cell::Empty;
        }

        // Place new move.
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
        for cell in &self.cells {
            if matches!(cell, Cell::Empty) {
                return false;
            }
        }
        true
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
        Game {
            board: Board::new(),
            current_player: Player::X,
            status: GameStatus::InProgress,
        }
    }

    pub fn winner_position(&self) -> Option<[usize; 3]> {
        self.board.winner_position()
    }

    pub fn last_move(&self, player: Player) -> Option<usize> {
        match player {
            Player::X => {
                if self.board.x_moves.is_empty() || self.board.x_moves.len() < 3 {
                    None
                } else {
                    self.board.x_moves.first().copied()
                }
            },
            Player::O => {
                if self.board.o_moves.is_empty() || self.board.o_moves.len() < 3 {
                    None
                } else {
                    self.board.o_moves.first().copied()
                }
            },
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
        if self.status != GameStatus::InProgress {
            return false;
        }
        if !self.board.make_move(position, self.current_player) {
            return false;
        }
        if let Some(player) = self.board.winner() {
            self.status = GameStatus::Won(player);
            return true;
        }
        if self.board.is_draw() {
            self.status = GameStatus::Draw;
            return true;
        }
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        true
    }
}
