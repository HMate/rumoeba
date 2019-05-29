use crate::game_board::BoardSizeT;

pub mod ui;
pub mod game_board;

pub fn start_game() {
    ui::show_message("Pick a board size!");
    let board_size = read_board_size();

    ui::show_message(&format!("Let the game begin! {}", board_size));

    let board = game_board::Board::new(board_size);
    ui::show_board(board);
    //loop {}
}

fn read_board_size() -> BoardSizeT {
    loop {
        let mut user_input = String::new();
        if std::io::stdin().read_line(&mut user_input).is_err() {
            continue;
        }
        if let Ok(parsed_number) = user_input.trim().parse::<BoardSizeT>() {
            if 2 < parsed_number && parsed_number < 25 {
                return parsed_number;
            }
        }
    }
}
