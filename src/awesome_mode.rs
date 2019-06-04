extern crate console;
use console::Term;
use crate::xogame;
use crate::xogame::{XOGame, GameResult};
use crate::player::Player;
use crate::game_board;
use crate::game_board::BoardSizeT;


pub fn start_awesome() {
    let term = Term::stdout();

    let board_size = choose_dimension(&term);
    term.write_line(&format!("See you in dimension {}!", board_size)).unwrap();

    term.clear_line().unwrap();

    let player1 = Player::new("SeeZee");
    let player2 = Player::new("LaGente");
    let mut game = XOGame::new(board_size, &player1, &player2);

    let mut board_pos: (BoardSizeT, BoardSizeT) = (0, 0);
    let mut cur_player = &player1;
    while !game.finished() {
        term.clear_screen().unwrap();
        let board = draw_board(game.board(), &board_pos);
        term.write_line(&board).unwrap();
        term.write_line(&format!("{}s turn!", cur_player.name())).unwrap();

        let mut hit_enter = false;
        match term.read_key().unwrap() {
            console::Key::Enter => hit_enter = true,
            console::Key::ArrowLeft => {
                let x = board_pos.0;
                if x == 0 {
                    board_pos.0 = board_size -1;
                } else {
                    board_pos.0 -= 1;
                }
            },
            console::Key::ArrowRight => {
                board_pos.0 += 1;
                if board_pos.0 >= board_size {
                    board_pos.0 = 0;
                }
            },
            console::Key::ArrowDown => {
                let y = board_pos.1;
                if y == 0 {
                    board_pos.1 = board_size -1;
                } else {
                    board_pos.1 -= 1;
                }
            },
            console::Key::ArrowUp => {
                board_pos.1 += 1;
                if board_pos.1 >= board_size {
                    board_pos.1 = 0;
                }
            },
            _ => ()
        };

        term.clear_screen().unwrap();
        let board = draw_board(game.board(), &board_pos);
        term.write_line(&board).unwrap();
        if hit_enter && game.place(cur_player, board_pos.0, board_pos.1).is_ok() {
            if cur_player == &player1 {
                cur_player = &player2;
            } else {
                cur_player = &player1;
            }
        }
    }

    let winner = game.winner();
    if winner == GameResult::Player1 {
        term.write_line(&format!("{} is the champion of RUMOEBA!", player1.name())).unwrap();
    } else if winner == GameResult::Player2 {
        term.write_line(&format!("{} is the champion of RUMOEBA!", player2.name())).unwrap();
    } else {
        term.write_line(&format!("Todays result is a very close TIE between {} and {}!",
                                  player1.name(), player2.name())).unwrap();
    }
}

fn draw_board(board: &game_board::Board, cursor_pos: &(BoardSizeT, BoardSizeT)) -> String {
    let size = board.size();
    let mut result = String::with_capacity(((size + 3) * (size + 4)) as usize);
    result.push_str("Y\\X ");
    for x in 0..size {
        result.push_str(&format!("{:^2}", x + 1));
    }
    result.push('\n');

    for y in (0..size).rev() {
        result.push_str(&format!("{:>3}", y + 1));
        for x in 0..size {
            if &(x, y) == cursor_pos {
                match board.read(x, y) {
                    game_board::Mark::X => result.push_str("|X*"),
                    game_board::Mark::O => result.push_str("|O*"),
                    game_board::Mark::Unmarked => result.push_str("|*")
                };
            }
            else {
                match board.read(x, y) {
                    game_board::Mark::X => result.push_str("|X"),
                    game_board::Mark::O => result.push_str("|O"),
                    game_board::Mark::Unmarked => result.push_str("|_")
                };
            }
        }
        result.push_str(&format!("|{}\n", y + 1));
    }
    result.push_str("    ");
    for x in 0..size {
        result.push_str(&format!("{:^2}", x + 1));
    }

    result
}

fn choose_dimension(term: &Term) -> BoardSizeT {

    let dimension_range = xogame::MIN_BOARD_SIZE..xogame::MAX_BOARD_SIZE;
    let dimensions: Vec<BoardSizeT> = dimension_range.collect();
    let mut selected_index = 0;

    let mut waiting_enter = true;
    while waiting_enter {
        term.clear_screen().unwrap();
        term.write_line("The dimensions Mason! What are they?").unwrap();

        let dim_chooser_line = draw_dimension_chooser(&dimensions, selected_index);
        term.write_line(&dim_chooser_line).unwrap();

        match term.read_key().unwrap() {
            console::Key::Enter => waiting_enter = false,
            console::Key::ArrowLeft => {
                if selected_index == 0 {
                    selected_index = dimensions.len()-1;
                } else {
                    selected_index -= 1;
                }
            },
            console::Key::ArrowRight => {
                selected_index += 1;
                if selected_index >= dimensions.len() {
                    selected_index = 0;
                }
            },
            _ => ()
        };
    }
    return dimensions[selected_index];
}

fn draw_dimension_chooser(dimensions: &Vec<BoardSizeT>, selected_index: usize) ->String {
    let mut dim_chooser_line = String::with_capacity(dimensions.len() * 4);
    for (index, dim) in dimensions.iter().enumerate() {
        if index == selected_index {
            dim_chooser_line.push_str(&format!("*{:}*", dim));
        } else {
            dim_chooser_line.push_str(&format!("{:^4}", dim));
        }
    }
    dim_chooser_line
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_board_empty_with_x() {
        let b = game_board::Board::new(3);
        let expected =  String::from("Y\\X 1 2 3 ") + "
  3|_|_|_|3
  2|_|_|_|2
  1|*|_|_|1" + "
    1 2 3 ";
        let actual = draw_board(&b, &(0, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn show_board_xo() {
        let mut b = game_board::Board::new(3);
        b.mark(0, 0, game_board::Mark::X);
        b.mark(0, 1, game_board::Mark::O);
        b.mark(1, 1, game_board::Mark::X);
        b.mark(2, 2, game_board::Mark::O);
        let expected = String::from("Y\\X 1 2 3 ") + "
  3|_|_|O|3
  2|O|X|_|2
  1|X*|_|_|1" + "
    1 2 3 ";
        let actual = draw_board(&b, &(0, 0));
        assert_eq!(actual, expected);
    }
}