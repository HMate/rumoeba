pub use crate::xogame::XOGame;
use crate::game_board::BoardSizeT;

pub mod ui;
pub mod xogame;
mod game_board;

const MIN_BOARD_SIZE: BoardSizeT = 3;
const MAX_BOARD_SIZE: BoardSizeT = 25;

#[derive(Debug, PartialEq)]
pub struct Player {
    name: String
}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player{name: String::from(name)};
    }

    pub fn name(&self) -> &String {
        return &self.name;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_player_name() {
        let p = Player::new("Kose");
        assert_eq!("Kose", p.name());
    }
}