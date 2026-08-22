use tictactoe::game::{Board, Game, GameStatus, Player};

#[test]
fn player_can_make_move() {
    let mut board = Board::new();

    let result = board.make_move(0, Player::X);

    assert!(result);
}

#[test]
fn player_cannot_make_move_on_occupied_cell() {
    let mut board = Board::new();

    board.make_move(0, Player::X);
    let result = board.make_move(0, Player::O);

    assert!(!result);
}

#[test]
fn o_can_win_horizontally() {
    let mut board = Board::new();

    board.make_move(0, Player::O);
    board.make_move(1, Player::O);
    board.make_move(2, Player::O);

    assert!(matches!(board.winner(), Some(Player::O)));
}

#[test]
fn x_can_win_vertically() {
    let mut board = Board::new();

    board.make_move(0, Player::X);
    board.make_move(3, Player::X);
    board.make_move(6, Player::X);

    assert!(matches!(board.winner(), Some(Player::X)));
}

#[test]
fn o_can_win_diagonally() {
    let mut board = Board::new();

    board.make_move(0, Player::O);
    board.make_move(4, Player::O);
    board.make_move(8, Player::O);

    assert!(matches!(board.winner(), Some(Player::O)));
}

#[test]
fn nobody_wins() {
    let mut board = Board::new();

    board.make_move(0, Player::O);
    board.make_move(1, Player::X);
    board.make_move(2, Player::O);
    board.make_move(8, Player::X);

    assert!(board.winner().is_none());
    assert!(!board.is_draw());
}

#[test]
fn is_draw() {
    let mut board = Board::new();

    board.make_move(0, Player::O);
    board.make_move(1, Player::X);
    board.make_move(2, Player::O);
    board.make_move(3, Player::X);
    board.make_move(4, Player::O);
    board.make_move(5, Player::X);
    board.make_move(6, Player::X);
    board.make_move(7, Player::O);
    board.make_move(8, Player::X);

    assert!(board.is_draw());
}

#[test]
fn game_can_be_played() {
    let mut game = Game::new();

    assert!(game.make_move(0));
    assert!(game.make_move(1));
    assert!(game.make_move(2));
}

#[test]
fn game_stops_after_win() {
    let mut game = Game::new();

    assert!(game.make_move(0));
    assert!(game.make_move(3));
    assert!(game.make_move(1));
    assert!(game.make_move(4));
    assert!(game.make_move(2));

    assert_eq!(game.status(), &GameStatus::Won(Player::X));
    assert!(!game.make_move(5));
}
