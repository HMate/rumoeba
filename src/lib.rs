use crate::game_board::BoardSizeT;
pub use crate::player::Player;
use crate::xogame::GameResult;
pub use crate::xogame::XOGame;

pub mod ui;
pub mod player;
pub mod xogame;
pub mod awesome_mode;
mod game_board;

const MIN_BOARD_SIZE: BoardSizeT = 3;
const MAX_BOARD_SIZE: BoardSizeT = 25;


pub fn start_game() {
    ui::show_message("What dimensions are we talking about?");
    let board_size = read_board_size();

    ui::show_message(&format!("Let the game begin! {}", board_size));

    let player1 = Player::new("SeeZee");
    let player2 = Player::new("LaGente");
    let mut game = XOGame::new(board_size, &player1, &player2);

    let mut cur_player = &player1;
    while !game.finished() {
        let board = ui::draw_board(game.board());
        ui::show_message(&board);

        ui::show_message(&format!("{}s turn to play!", cur_player.name()));
        ui::show_message_cursor_at_end(&format!("give X (1-{}): ", board_size));
        let x = read_move(board_size);
        ui::show_message_cursor_at_end(&format!("give Y (1-{}): ", board_size));
        let y = read_move(board_size);

        if game.place(cur_player, x, y).is_ok() {
            if cur_player == &player1 {
                cur_player = &player2;
            } else {
                cur_player = &player1;
            }
        }
    }

    let winner = game.winner();
    if winner == GameResult::Player1 {
        ui::show_message(&format!("{} is the champion of RUMOEBA!", player1.name()));
    } else if winner == GameResult::Player2 {
        ui::show_message(&format!("{} is the champion of RUMOEBA!", player2.name()));
    } else {
        ui::show_message(&format!("Todays result is a very close TIE between {} and {}!",
                                  player1.name(), player2.name()));
    }
}

fn read_board_size() -> BoardSizeT {
    loop {
        let mut user_input = String::new();
        if std::io::stdin().read_line(&mut user_input).is_err() {
            continue;
        }
        if let Ok(parsed_number) = user_input.trim().parse::<BoardSizeT>() {
            if MIN_BOARD_SIZE <= parsed_number && parsed_number < MAX_BOARD_SIZE {
                return parsed_number;
            }
        }
    }
}

fn read_move(board_size: BoardSizeT) -> BoardSizeT {
    loop {
        let mut user_input = String::new();
        if std::io::stdin().read_line(&mut user_input).is_err() {
            continue;
        }
        if let Ok(parsed_number) = user_input.trim().parse::<BoardSizeT>() {
            if 1 <= parsed_number && parsed_number <= board_size {
                return parsed_number - 1;
            }
        }
    }
}
