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
    let mut result = String::with_capacity((size * (size + 3)) as usize);
    for y in (0..size).rev() {
        for x in 0..size {
            match board.read(x, y) {
                game_board::Mark::X => result.push_str("|X|"),
                game_board::Mark::O => result.push_str("|O|"),
                game_board::Mark::Unmarked => result.push_str("|_|")
            };
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_board_empty() {
        let b = game_board::Board::new(3);
        let expected = "\
|_||_||_|
|_||_||_|
|_||_||_|
";
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
        let expected = "\
|_||_||O|
|O||X||_|
|X||_||_|
";
        let actual = draw_board(&b);
        assert_eq!(actual, expected);
    }
}