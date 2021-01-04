#[repr(u8)]
enum Side {
    Dark = 0,
    Light = 1,
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Disk {
    Black = 0,
    White = 1,
}

struct Position {
    row: usize,
    col: char,
}

impl Position {
    fn new(col: char, row: usize) -> Position {
        if col < 'a' || 'h' < col {
            panic!("out of bound for position col: {}", col);
        }
        if row < 1 || 9 < row {
            panic!("out of bound for position row: {}", row);
        }

        Position { col, row }
    }

    fn col(&self) -> usize {
        (self.col as u8 - b'a') as usize
    }

    fn row(&self) -> usize {
        (self.row - 1) as usize
    }

    fn to_index(&self) -> usize {
        self.row() * 8 + self.col()
    }

    fn from_index(index: usize) -> Position {
        if index >= 64 {
            panic!("out of bound for position index");
        }

        let col = index % 8;
        let col = (col as u8 + 'a' as u8) as char;

        let row = index / 8 + 1;

        Position { col, row }
    }
}

struct Board {
    board: [Option<Disk>; 64],
}

impl Board {
    fn new() -> Board {
        Board { board: [None; 64] }
    }

    fn place_disk(disk: Disk, pos: Position) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "col")]
    fn position_col_under_bound() {
        Position::new('Z', 1);
    }

    #[test]
    #[should_panic(expected = "col")]
    fn position_col_over_bound() {
        Position::new('(', 1);
    }

    #[test]
    #[should_panic(expected = "row")]
    fn position_row_under_bound() {
        Position::new('a', 0);
    }

    #[test]
    fn position_getter_row() {
        let pos = Position::new('d', 4);
        assert_eq!(pos.row(), 3);
    }

    #[test]
    fn position_getter_col() {
        let pos = Position::new('d', 4);
        assert_eq!(pos.col(), 3);
    }

    #[test]
    fn position_to_index() {
        let idx1 = Position::new('a', 1).to_index();
        let idx2 = Position::new('d', 4).to_index();
        let idx3 = Position::new('h', 8).to_index();
        assert_eq!((idx1, idx2, idx3), (0, 27, 63));
    }

    #[test]
    #[should_panic]
    fn position_from_index_over_bound() {
        Position::from_index(64);
    }

    #[test]
    fn position_from_index() {
        let pos = Position::from_index(27);
        assert_eq!((pos.col(), pos.row()), (3, 3));
    }
}
