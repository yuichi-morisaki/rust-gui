use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Position {
    col: Column,
    row: Row,
}

impl Position {
    pub fn col(&self) -> i8 {
        self.col.index()
    }

    pub fn row(&self) -> i8 {
        self.row.index()
    }

    pub fn col_and_row(&self) -> (i8, i8) {
        (self.col(), self.row())
    }

    pub fn index(&self) -> usize {
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
    use super::Column;
    use super::Row;

    use super::Position;

    // --------------------------------------------------------

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
}
