pub type BoardSizeT = i32;

pub struct Board {
    pub size: BoardSizeT
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Mark {
    Unmarked,
    X,
    O
}

impl Board {

    /// Returns whether the given position on the board is
    /// marked with X/O or unmarked
    ///
    /// # Examples
    ///
    /// ```
    /// use rumoeba::game_board::*;
    /// let b = Board{size:3};
    /// assert_eq!(b.read(2, 1), Mark::Unmarked);
    /// ```
    pub fn read(&self, row: BoardSizeT, col: BoardSizeT) -> Mark {
        return Mark::Unmarked;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_size() {
        let b = Board { size: 5 };
        assert_eq!(b.size, 5);
    }

    #[test]
    fn get_mark() {
        let b = Board { size: 5 };
        assert_eq!(b.read(3, 3), Mark::Unmarked);
    }
}