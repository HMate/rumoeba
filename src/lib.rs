use crate::game_board::BoardSizeT;

pub mod ui;
mod game_board;

const MIN_BOARD_SIZE: BoardSizeT = 3;
const MAX_BOARD_SIZE: BoardSizeT = 25;

pub struct XOGame {

}

impl XOGame {
    pub fn new(board_size: BoardSizeT, player_uno: &Player, player_dos: &Player) -> XOGame {
        return XOGame{};
    }

    pub fn place(&self, player: &Player, x: BoardSizeT, y: BoardSizeT) {

    }

    pub fn finished(&self) -> bool {
        true
    }
}

pub struct Player {

}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player{};
    }
}


pub fn start_game() {
    ui::show_message("What dimensions are we talking about?");
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
            if MIN_BOARD_SIZE <= parsed_number && parsed_number < MAX_BOARD_SIZE {
                return parsed_number;
            }
        }
    }
}
