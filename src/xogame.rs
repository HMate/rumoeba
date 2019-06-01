use crate::game_board::{BoardSizeT,Board,Mark};
use crate::Player;

pub struct XOGame<'a> {
    board: Board,
    player1: &'a Player,
    player2: &'a Player,
}

impl <'a> XOGame<'a>  {
    pub fn new(board_size: BoardSizeT, player_uno: &'a Player, player_dos: &'a Player) -> XOGame<'a> {
        return XOGame {
            board: Board::new(board_size),
            player1: player_uno,
            player2: player_dos
        };
    }

    pub fn place(&mut self, player: &Player, x: BoardSizeT, y: BoardSizeT) -> Result<(), ()>{
        if self.board.read(x,y) != Mark::Unmarked {
            return Err(());
        }
        self.board.mark(x, y, self.get_player_mark(player));
        Ok(())
    }

    pub fn finished(&self) -> bool {
        let bsize = self.board.size();
//        let count_of_marks_to_win = match self.board.size() {
//            3 => 3,
//            4 => 4,
//            _ => 5
//        };
        let count_of_marks_to_win = 3;

        let mut last_mark = Mark::Unmarked;
        let mut count = 0;

        for y in 0..bsize {
            for x in 0..bsize{
                let mark = self.board.read(x, y);
                if mark == last_mark {
                    count += 1;
                }else {
                    last_mark = mark;
                    count = 1;
                }

                if last_mark != Mark::Unmarked && count >= count_of_marks_to_win {
                    return true;
                }
            }
        }
        return false;
    }

    fn get_player_mark(&self, player: &Player) -> Mark {
        if player as *const Player == self.player1 as *const Player {
            return Mark::X;
        }
        else if player as *const Player == self.player2 as *const Player {
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
    }

    #[test]
    fn game_not_finished_initially() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let game = XOGame::new(3, &player1, &player2);

        assert_eq!(false, game.finished());
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
    fn place_to_same_place_gives_error() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let mut game = XOGame::new(3, &player1, &player2);

        assert!(game.place(&player1, 1, 1).is_ok());
        assert_eq!(Mark::X, game.board.read(1, 1));
        assert!(game.place(&player2, 1, 1).is_err());
        assert_eq!(Mark::X, game.board.read(1, 1));
    }

    #[test]
    fn get_player_mark() {
        let player1 = Player::new("SeeZee");
        let player2 = Player::new("Moak");
        let game = XOGame::new(3, &player1, &player2);

        assert_eq!(Mark::X, game.get_player_mark(&player1));
        assert_eq!(Mark::O, game.get_player_mark(&player2));
        assert_eq!(Mark::Unmarked, game.get_player_mark(&Player::new("Moak Doppleganger")));
    }

// TODO: error when placing mark where somebody already placed
}