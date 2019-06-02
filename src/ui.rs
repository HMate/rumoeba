extern crate std;

use std::io;
use std::io::prelude::*;

use super::game_board;

pub fn show_message(msg: &str) {
    println!("{}", msg);
}

pub fn show_message_cursor_at_end(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().ok().expect("Could not flush stdout");
}

pub fn draw_board(board: &game_board::Board) -> String {
    let size = board.size();
    let mut result = String::with_capacity(((size + 3) * (size + 4)) as usize);
    result.push_str("Y\\X");
    for x in 0..size {
        result.push_str(&format!("{:^3}", x+1));
    }
    result.push('\n');

    for y in (0..size).rev() {
        result.push_str(&format!("{:>3}", y+1));
        for x in 0..size {
            match board.read(x, y) {
                game_board::Mark::X => result.push_str("|X|"),
                game_board::Mark::O => result.push_str("|O|"),
                game_board::Mark::Unmarked => result.push_str("|_|")
            };
        }
        result.push_str(&format!("{}\n", y+1));
    }
    result.push_str("   ");
    for x in 0..size {
        result.push_str(&format!("{:^3}", x+1));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_board_empty() {
        let b = game_board::Board::new(3);
        let expected =  String::from("Y\\X 1  2  3 ") + "
  3|_||_||_|3
  2|_||_||_|2
  1|_||_||_|1" + "
    1  2  3 ";
        let actual = draw_board(&b);
        assert_eq!(actual, expected);
    }

    #[test]
    fn show_board_xo() {
        let mut b = game_board::Board::new(3);
        b.mark(0, 0, game_board::Mark::X);
        b.mark(0, 1, game_board::Mark::O);
        b.mark(1, 1, game_board::Mark::X);
        b.mark(2, 2, game_board::Mark::O);
        let expected = String::from("Y\\X 1  2  3 ") + "
  3|_||_||O|3
  2|O||X||_|2
  1|X||_||_|1" + "
    1  2  3 ";
        let actual = draw_board(&b);
        assert_eq!(actual, expected);
    }
}