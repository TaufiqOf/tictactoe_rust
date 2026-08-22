use tictactoe::game::Game;

fn main() {
    println!("Welcome to Tic Tac Toe!");

    let mut game = Game::new();

    game.make_move(0);
    game.make_move(1);
    game.make_move(2);

    game.display();
}
