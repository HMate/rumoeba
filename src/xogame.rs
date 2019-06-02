use crate::game_board::{Board, BoardSizeT, Mark};
use crate::Player;

pub struct XOGame<'a> {
    board: Board,
    player1: &'a Player,
    player2: &'a Player,
    winner: Option<&'a Player>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameResult {
    Player1,
    Player2,
    Tie,
}

impl<'a> XOGame<'a> {
    pub fn new(board_size: BoardSizeT, player_uno: &'a Player, player_dos: &'a Player) -> XOGame<'a> {
        return XOGame {
            board: Board::new(board_size),
            player1: player_uno,
            player2: player_dos,
            winner: None,
        };
    }

    pub fn place(&mut self, player: &'a Player, x: BoardSizeT, y: BoardSizeT) -> Result<(), ()> {
        if self.board.read(x, y) != Mark::Unmarked {
            return Err(());
        }
        self.board.mark(x, y, self.get_player_mark(player));

        if self.finished() && !self.is_tie() {
            self.winner = Some(player);
        }

        Ok(())
    }

    fn is_tie(&self) -> bool {
        let bsize = self.board.size();
        for y in 0..bsize {
            for x in 0..bsize {
                if Mark::Unmarked == self.board.read(x, y) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn finished(&self) -> bool {
        if self.is_tie() {
            return true;
        }

        let count_of_marks_to_win = self.get_marks_to_win();

        let count_marks = |x, y, last_mark: &mut Mark, count: &mut i32| -> bool {
            let mark = self.board.read(x, y);
            if mark == *last_mark {
                *count += 1;
            } else {
                *last_mark = mark;
                *count = 1;
            }

            if *last_mark != Mark::Unmarked && *count >= count_of_marks_to_win {
                return true;
            }
            return false;
        };

        let bsize = self.board.size();
        let mut last_mark = Mark::Unmarked;
        let mut count = 0;

        for y in 0..bsize {
            for x in 0..bsize {
                if count_marks(x, y, &mut last_mark, &mut count) {
                    return true;
                }
            }
            count = 0;
        }

        for x in 0..bsize {
            for y in 0..bsize {
                if count_marks(x, y, &mut last_mark, &mut count) {
                    return true;
                }
            }
            count = 0;
        }

        // even diagonal check
        let x_start = 0;
        for y_start in 0..bsize {
            let mut x = x_start;
            let mut y = y_start;
            while x < bsize && y < bsize {
                if count_marks(x, y, &mut last_mark, &mut count) {
                    return true;
                }

                x += 1;
                y += 1;
            }
            count = 0;
        }

        // odd diagonal check
        let x_start = bsize - 1;
        for y_start in 0..bsize {
            let mut x = x_start;
            let mut y = y_start;
            while x >= 0 && y < bsize {
                if count_marks(x, y, &mut last_mark, &mut count) {
                    return true;
                }

                x -= 1;
                y += 1;
            }
            count = 0;
        }
        return false;
    }

    fn get_marks_to_win(&self) -> i32 {
        match self.board.size() {
            3 => 3,
            4 => 4,
            _ => 5
        }
    }

    pub fn winner(&self) -> GameResult {
        match self.winner {
            Some(player) => {
                if player as *const Player == self.player1 as *const Player {
                    return GameResult::Player1;
                } else if player as *const Player == self.player2 as *const Player {
                    return GameResult::Player2;
                }
            }
            None => return GameResult::Tie
        }
        return GameResult::Tie;
    }

    fn get_player_mark(&self, player: &Player) -> Mark {
        if player as *const Player == self.player1 as *const Player {
            return Mark::X;
        } else if player as *const Player == self.player2 as *const Player {
            return Mark::O;
        }
        return Mark::Unmarked;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_game() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 1, 1);
        game.place(&player2, 1, 2);
        game.place(&player1, 0, 1);
        game.place(&player2, 2, 1);
        game.place(&player1, 0, 0);
        game.place(&player2, 2, 2);
        game.place(&player1, 0, 2);
        assert!(game.finished());
        assert_eq!(game.winner(), GameResult::Player1);
    }

    #[test]
    fn play_game_4() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(4, &player1, &player2);

        game.place(&player1, 1, 1);
        game.place(&player2, 1, 2);
        game.place(&player1, 2, 2);
        game.place(&player2, 2, 1);
        game.place(&player1, 0, 0);
        assert!(!game.finished());
        game.place(&player2, 3, 0);
        assert!(!game.finished());
        game.place(&player1, 3, 3);
        assert!(game.finished());
    }

    #[test]
    fn play_game_5() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(15, &player1, &player2);

        game.place(&player1, 7, 7);
        game.place(&player2, 8, 7);
        game.place(&player1, 7, 6);
        game.place(&player2, 7, 8);
        game.place(&player1, 8, 6);
        game.place(&player2, 9, 6);
        game.place(&player1, 6, 9);
        game.place(&player2, 9, 8);
        game.place(&player1, 8, 8);
        game.place(&player2, 9, 7);
        game.place(&player1, 6, 6);
        game.place(&player2, 9, 9);
        game.place(&player1, 5, 6);
        assert!(!game.finished());
        game.place(&player2, 9, 10);
        assert!(game.finished());
        assert_eq!(game.winner(), GameResult::Player2);
    }

    #[test]
    fn play_game_to_tie() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 1, 1);
        game.place(&player2, 2, 1);
        game.place(&player1, 0, 1);
        game.place(&player2, 0, 0);
        game.place(&player1, 2, 0);
        game.place(&player2, 0, 2);
        game.place(&player1, 1, 2);
        game.place(&player2, 1, 0);
        // assert!(game.finished()); //earlies tie detection can be here
        game.place(&player1, 2, 2);
        assert!(game.finished());
        assert_eq!(game.winner(), GameResult::Tie);
    }

    #[test]
    fn game_not_finished_initially() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let game = XOGame::new(3, &player1, &player2);

        assert_eq!(game.finished(), false);
    }

    #[test]
    fn finished_row() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 0, 0);
        game.place(&player1, 1, 0);
        game.place(&player1, 2, 0);
        assert!(game.finished());
    }

    #[test]
    fn finished_row_non_continously() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 1, 0);
        game.place(&player1, 2, 0);
        game.place(&player1, 0, 1);
        assert!(!game.finished());
    }

    #[test]
    fn finished_column() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 0, 0);
        game.place(&player1, 0, 1);
        game.place(&player1, 0, 2);
        assert!(game.finished());
    }

    #[test]
    fn finished_even_diagonal() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 0, 0);
        game.place(&player1, 1, 1);
        game.place(&player1, 2, 2);
        assert!(game.finished());
    }

    #[test]
    fn finished_odd_diagonal() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        game.place(&player1, 2, 0);
        game.place(&player1, 1, 1);
        game.place(&player1, 0, 2);
        assert!(game.finished());
    }

    #[test]
    fn place_to_same_place_gives_error() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        assert!(game.place(&player1, 1, 1).is_ok());
        assert_eq!(game.board.read(1, 1), Mark::X);
        assert!(game.place(&player2, 1, 1).is_err());
        assert_eq!(game.board.read(1, 1), Mark::X);
    }

    #[test]
    fn get_player_mark() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let game = XOGame::new(3, &player1, &player2);

        assert_eq!(game.get_player_mark(&player1), Mark::X);
        assert_eq!(game.get_player_mark(&player2), Mark::O);
        assert_eq!(game.get_player_mark(&Player::new("Moak Doppleganger")), Mark::Unmarked);
    }
}