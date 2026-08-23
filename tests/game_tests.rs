use tictactoe::game::{Game, GameStatus, Player};

#[test]
fn game_starts_with_x() {
    let game = Game::new();

    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game.status(), &GameStatus::InProgress);
}

#[test]
fn players_alternate_turns() {
    let mut game = Game::new();

    assert_eq!(game.current_player(), Player::X);

    assert!(game.make_move(0));
    assert_eq!(game.current_player(), Player::O);

    assert!(game.make_move(1));
    assert_eq!(game.current_player(), Player::X);

    assert!(game.make_move(2));
    assert_eq!(game.current_player(), Player::O);
}

#[test]
fn nobody_wins() {
    let mut game = Game::new();

    game.make_move(0);
    game.make_move(1);
    game.make_move(2);
    game.make_move(8);

    assert_eq!(game.status(), &GameStatus::InProgress);
}

#[test]
fn game_can_end_in_draw() {
    let mut game = Game::new();

    game.make_move(0);
    game.make_move(1);
    game.make_move(2);
    game.make_move(4);
    game.make_move(3);
    game.make_move(5);
    game.make_move(7);
    game.make_move(6);
    game.make_move(8);
    game.display();
    assert_ne!(game.status(), &GameStatus::Draw);
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

#[test]
fn occupied_position_is_rejected() {
    let mut game = Game::new();

    assert!(game.make_move(0));
    assert!(!game.make_move(0));

    assert_eq!(game.current_player(), Player::O);
}

#[test]
fn invalid_position_is_rejected() {
    let mut game = Game::new();

    assert!(!game.make_move(9));

    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game.status(), &GameStatus::InProgress);
}
