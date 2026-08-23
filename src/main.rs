use eframe::egui;

use tictactoe::game::{Game,Player, GameStatus,GameMode};

struct TicTacToeApp {
    game: Game,
    mode: GameMode,
}
impl Default for TicTacToeApp {
    fn default() -> Self {
        let mode = GameMode::HumanVsHuman;

        Self {
            game: Game::new_with_mode(mode),
            mode,
        }
    }
}
impl TicTacToeApp {
    fn new() -> Self {
        let mode = GameMode::HumanVsHuman;

        Self {
            game: Game::new_with_mode(mode),
            mode,
        }
    }

    fn draw_status(&self, ui: &mut egui::Ui) {
        match self.game.status() {
            GameStatus::InProgress => {
                ui.heading(format!("Player {:?}'s turn", self.game.current_player()));
            }

            GameStatus::Won(player) => {
                ui.heading(format!("Player {:?} wins!", player));
            }

            GameStatus::Draw => {
                ui.heading("It's a draw!");
            }
        }
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let symbols: Vec<char> = self
            .game
            .board()
            .iter()
            .map(|cell| cell.symbol())
            .collect();

        let x_last_move = self.game.first_move(Player::X);
        let o_last_move = self.game.first_move(Player::O);
        let winner_positions = self.game.winner_position();

        let game_over = self.game.status() != &GameStatus::InProgress;

        egui::Grid::new("board")
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                for position in 0..9 {
                    let mut button = egui::Button::new(
                        egui::RichText::new(symbols[position].to_string())
                            .size(40.0),
                    )
                        .min_size(egui::vec2(80.0, 80.0));

                    if let Some(positions) = winner_positions {
                        if positions.contains(&position) {
                            button = button.fill(egui::Color32::LIGHT_GREEN);
                        }
                    } else if x_last_move == Some(position) {
                        button = button.fill(egui::Color32::LIGHT_BLUE);
                    } else if o_last_move == Some(position) {
                        button = button.fill(egui::Color32::LIGHT_YELLOW);
                    }

                    let response = ui.add_enabled(
                        !game_over && symbols[position] == ' ',
                        button,
                    );

                    if response.clicked() {
                        self.game.make_move(position);
                    }

                    if position % 3 == 2 {
                        ui.end_row();
                    }
                }
            });
    }
}

impl eframe::App for TicTacToeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.label("Game mode:");

                if ui
                    .selectable_label(
                        self.mode == GameMode::HumanVsHuman,
                        "2 Players",
                    )
                    .clicked()
                {
                    self.mode = GameMode::HumanVsHuman;
                    self.game = Game::new_with_mode(self.mode);
                }

                if ui
                    .selectable_label(
                        self.mode == GameMode::HumanVsBot,
                        "Vs Bot",
                    )
                    .clicked()
                {
                    self.mode = GameMode::HumanVsBot;
                    self.game = Game::new_with_mode(self.mode);
                }
            });

            ui.add_space(15.0);

            self.draw_status(ui);

            ui.add_space(15.0);

            self.draw_board(ui);

            ui.add_space(10.0);

            let reset_button = egui::Button::new(
                egui::RichText::new("Reset").size(15.0),
            )
                .min_size(egui::vec2(160.0, 45.0));

            if ui.add(reset_button).clicked() {
                self.game = Game::new_with_mode(self.mode);
            }
        });
    }
}

const WINDOW_WIDTH: f32 = 250.0;
const WINDOW_HEIGHT: f32 = 400.0;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Ok(Box::new(TicTacToeApp::new()))),
    )
}
