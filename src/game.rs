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
pub struct Board {
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
    pub fn new() -> Board {
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
        }
    }
    pub fn make_move(&mut self, position: usize, player: Player) -> bool {
        if position >= 9 {
            return false;
        }
        match self.cells[position] {
            Cell::Empty => {
                self.cells[position] = match player {
                    Player::X => Cell::X,
                    Player::O => Cell::O,
                };
                true
            }
            _ => false,
        }
    }
    pub fn winner(&self) -> Option<Player> {
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
            let a = &self.cells[line[0]];
            let b = &self.cells[line[1]];
            let c = &self.cells[line[2]];

            match (a, b, c) {
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
    pub fn is_draw(&self) -> bool {
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
