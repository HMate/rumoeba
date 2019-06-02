pub type BoardSizeT = i32;

pub struct Board {
    size: BoardSizeT,
    slots: Vec<Mark>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mark {
    Unmarked,
    X,
    O,
}

impl Board {
    pub fn new(size: BoardSizeT) -> Board {
        Board {
            size,
            slots: vec![Mark::Unmarked; (size * size) as usize],
        }
    }

    pub fn size(&self) -> BoardSizeT {
        self.size
    }

    /// Returns whether the given position on the board is
    /// marked with X/O or unmarked
    ///
    /// # Examples
    ///
    /// ```
    /// # use ::game_board::*;
    /// let b = Board::new(3);
    /// assert_eq!(b.read(2, 1), Mark::Unmarked);
    /// ```
    pub fn read(&self, x: BoardSizeT, y: BoardSizeT) -> Mark {
        return self.slots[self.ref_mark_on_pos(x, y)];
    }

    /// Marks the given position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ::game_board::*;
    /// let mut b = Board::new(3);
    /// b.mark(2,1,Mark::O);
    /// assert_eq!(b.read(2, 1), Mark::O);
    /// ```
    pub fn mark(&mut self, x: BoardSizeT, y: BoardSizeT, mark: Mark) {
        let index = self.ref_mark_on_pos(x, y);
        self.slots[index] = mark;
    }

    fn ref_mark_on_pos(&self, x: BoardSizeT, y: BoardSizeT) -> usize {
        (y * self.size + x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_size() {
        let b = Board::new(5);
        assert_eq!(b.size(), 5);
    }

    #[test]
    fn get_mark() {
        let b = Board::new(5);
        assert_eq!(b.read(3, 3), Mark::Unmarked);
    }

    #[test]
    fn get_mark_x() {
        let mut b = Board::new(7);
        b.mark(3, 6, Mark::X);
        assert_eq!(b.read(3, 6), Mark::X);
    }

    #[test]
    #[should_panic]
    fn panic_when_read_out_of_board() {
        let b = Board::new(5);
        let mark = b.read(3, 5);
    }

    #[test]
    #[should_panic]
    fn panic_when_mark_out_of_board() {
        let mut b = Board::new(5);
        b.mark(3, 5, Mark::O);
    }
}