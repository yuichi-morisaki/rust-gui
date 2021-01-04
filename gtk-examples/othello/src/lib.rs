use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

pub struct Game {
    root: Rc<State>,
    current: Rc<State>,
}

struct State {
    board: Board,
    turn: Side,
    parent: RefCell<Weak<State>>,
    children: RefCell<HashMap<Position, Rc<State>>>,
}

struct Board {
    disks: [Option<Disk>; 100],
}

impl Board {
    fn new() -> Board {
        let mut disks = [None; 100];

        disks[Position::from(('d', 4)).index()] = Some(Disk::White);
        disks[Position::from(('d', 5)).index()] = Some(Disk::Black);
        disks[Position::from(('e', 4)).index()] = Some(Disk::Black);
        disks[Position::from(('e', 5)).index()] = Some(Disk::White);

        Board { disks }
    }

    fn set_disk(&mut self, pos: Position, disk: Disk) {
        self.disks[pos.index()] = Some(disk);
    }

    fn get_disk(&self, pos: Position) -> Option<Disk> {
        self.disks[pos.index()]
    }

    fn try_place(&self, pos: Position, side: Side) -> Option<Board> {
        if self.get_disk(pos).is_some() {
            return None;
        }

        let (current, opponent) = match side {
            Side::Dark => (Disk::Black, Disk::White),
            Side::Light => (Disk::White, Disk::Black),
        };
        let mut positions = vec![];
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                for f_pos in self.flip_positions(
                    pos.col_and_row(),
                    (dx, dy),
                    (current, opponent),
                ) {
                    positions.push(f_pos);
                }
            }
        }
        if positions.len() == 0 {
            return None;
        }

        let mut board = Board::new();
        for col in 'a'..='h' {
            for row in 1i8..=8 {
                let pos = Position::from((col, row));
                if let Some(disk) = self.get_disk(pos) {
                    board.set_disk(pos, disk);
                }
            }
        }

        board.set_disk(pos, current);
        for pos in positions {
            board.set_disk(pos, current);
        }

        Some(board)
    }

    fn flip_positions(
        &self,
        (mut col, mut row): (i8, i8),
        (dx, dy): (i8, i8),
        (current, opponent): (Disk, Disk),
    ) -> Vec<Position> {
        let mut positions = vec![];
        loop {
            col += dx;
            row += dy;
            let pos = Position::from((col, row));
            if let Some(disk) = self.get_disk(pos) {
                if disk == current {
                    return positions;
                }
                if disk == opponent {
                    positions.push(pos);
                }
            } else {
                return vec![];
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    a   b   c   d   e   f   g   h\n")?;
        for index in 0..100 {
            let row = index / 10;
            let col = index % 10;
            if row == 0 || row == 9 || col == 0 || col == 9 {
                continue;
            }
            if col == 1 {
                write!(f, "  +---+---+---+---+---+---+---+---+\n{} |", row)?;
            }
            let disk = match self.disks[index] {
                None => ' ',
                Some(Disk::Black) => 'x',
                Some(Disk::White) => 'o',
            };
            write!(f, " {} |", disk)?;

            if col == 8 {
                write!(f, "\n")?;
            }
        }
        write!(f, "  +---+---+---+---+---+---+---+---+\n")
    }
}

// --------------------------------------------------------------------

#[repr(u8)]
#[derive(Clone, Copy)]
enum Disk {
    Black = 0,
    White = 1,
}

impl PartialEq for Disk {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Side {
    Dark = 0,
    Light = 1,
}

// --------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Position {
    col: Column,
    row: Row,
}

impl Position {
    fn col(&self) -> i8 {
        self.col.index()
    }

    fn row(&self) -> i8 {
        self.row.index()
    }

    fn col_and_row(&self) -> (i8, i8) {
        (self.col(), self.row())
    }

    fn index(&self) -> usize {
        (self.col() + self.row() * 10) as usize
    }
}

impl From<(i8, i8)> for Position {
    fn from((col, row): (i8, i8)) -> Position {
        let col = Column::from(col);
        let row = Row::from(row);

        Position { col, row }
    }
}

impl From<(char, i8)> for Position {
    fn from((col, row): (char, i8)) -> Position {
        let col = Column::from(col);
        let row = Row::from(row);

        Position { col, row }
    }
}

impl From<usize> for Position {
    fn from(index: usize) -> Position {
        let col = (index % 10) as i8;
        let row = (index / 10) as i8;

        let col = Column::from(col);
        let row = Row::from(row);

        Position { col, row }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.col() == other.col() && self.row() == other.row()
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.col().hash(state);
        self.row().hash(state);
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Column {
    index: i8,
}

impl Column {
    fn index(&self) -> i8 {
        self.index
    }
}

impl From<i8> for Column {
    fn from(index: i8) -> Column {
        if index < 0 || 9 < index {
            panic!("index out of bounds in Column: {}", index);
        }

        Column { index }
    }
}

impl From<char> for Column {
    fn from(index: char) -> Column {
        if index < 'a' || 'h' < index {
            panic!("index out of bounds in Column: {}", index);
        }

        let index = (index as u8 - b'a' + 1) as i8;

        Column { index }
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Row {
    index: i8,
}

impl Row {
    fn index(&self) -> i8 {
        self.index
    }
}

impl From<i8> for Row {
    fn from(index: i8) -> Row {
        if index < 0 || 9 < index {
            panic!("index out of bounds in Row: {}", index);
        }

        Row { index }
    }
}

// ====================================================================

#[cfg(test)]
mod tests {

    // --------------------------------------------------------

    use super::Row;

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn row_from_i8_under_bound() {
        Row::from(-1);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn row_from_i8_over_bound() {
        Row::from(10);
    }

    #[test]
    fn row_from_i8() {
        let row0 = Row::from(0);
        assert_eq!(row0.index(), 0);

        let row9 = Row::from(9);
        assert_eq!(row9.index(), 9);
    }

    // --------------------------------------------------------

    use super::Column;

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn col_from_i8_under_bound() {
        Column::from(-1);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn col_from_i8_over_bound() {
        Column::from(10);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn col_from_char_under_bound() {
        let c = (b'a' - 1) as char;
        Column::from(c);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn col_from_char_over_bound() {
        let c = (b'h' + 1) as char;
        Column::from(c);
    }

    #[test]
    fn col_from_i8() {
        let col0 = Column::from(0);
        assert_eq!(col0.index(), 0);

        let col9 = Column::from(9);
        assert_eq!(col9.index(), 9);
    }

    #[test]
    fn col_from_char() {
        let col1 = Column::from('a');
        assert_eq!(col1.index(), 1);

        let col8 = Column::from('h');
        assert_eq!(col8.index(), 8);
    }

    // --------------------------------------------------------

    use super::Position;

    #[test]
    fn position_from_char_and_i8() {
        let pos = Position::from(('a', 1));
        assert_eq!(pos.col_and_row(), (1, 1));

        let pos = Position::from(('h', 8));
        assert_eq!(pos.col_and_row(), (8, 8));
    }

    #[test]
    fn position_from_i8_and_i8() {
        let pos = Position::from((0, 0));
        assert_eq!(pos.index(), 0);

        let pos = Position::from((9, 9));
        assert_eq!(pos.index(), 99);
    }

    #[test]
    fn position_from_usize() {
        let pos = Position::from(0);
        assert_eq!(pos.col(), 0);
        assert_eq!(pos.row(), 0);

        let pos = Position::from(99);
        assert_eq!(pos.col(), 9);
        assert_eq!(pos.row(), 9);
    }

    // --------------------------------------------------------

    use super::Board;
    use super::Disk;

    #[test]
    fn board_new() {
        let board = Board::new();
        let output = "    \
    a   b   c   d   e   f   g   h
  +---+---+---+---+---+---+---+---+
1 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
2 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
3 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
4 |   |   |   | o | x |   |   |   |
  +---+---+---+---+---+---+---+---+
5 |   |   |   | x | o |   |   |   |
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
7 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
8 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn board_try_place_some() {
        let board = Board::new();
        let pos = Position::from(('c', 4));
        let board = board.try_place(pos, Side::Dark).unwrap();
        let output = "    \
    a   b   c   d   e   f   g   h
  +---+---+---+---+---+---+---+---+
1 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
2 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
3 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
4 |   |   | x | x | x |   |   |   |
  +---+---+---+---+---+---+---+---+
5 |   |   |   | x | o |   |   |   |
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
7 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
8 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn board_try_place_none() {
        let board = Board::new();
        let pos = Position::from(('c', 4));
        let board = board.try_place(pos, Side::Light);
        assert!(board.is_none());
    }

    // --------------------------------------------------------

    use super::Side;

    // --------------------------------------------------------
}
