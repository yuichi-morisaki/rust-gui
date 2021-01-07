use std::hash::{Hash, Hasher};
use std::ops;

#[derive(Debug)]
pub struct Position {
    col: Column,
    row: Row,
}

impl Position {
    pub fn new(col: char, row: usize) -> Position {
        Position {
            col: Column::new(col),
            row: Row::new(row),
        }
    }

    pub fn coordinate(&self) -> (Column, Row) {
        (self.col, self.row)
    }
}

impl From<(Column, Row)> for Position {
    fn from((col, row): (Column, Row)) -> Position {
        Position { col, row }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.col.index().hash(state);
        self.row.index().hash(state);
    }
}

// ---------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Column {
    index: i32,
}

impl Column {
    fn new(index: char) -> Column {
        if index < 'a' || 'h' < index {
            panic!("index out of bounds for Column");
        }
        let index = (index as u8 - b'a') as i32;
        Column { index }
    }

    fn index(&self) -> i32 {
        self.index
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl ops::Add<i32> for Column {
    type Output = Option<Column>;

    fn add(self, rhs: i32) -> Option<Column> {
        let index = self.index + rhs;
        if index < 0 || 8 <= index {
            None
        } else {
            Some(Column { index })
        }
    }
}

// ---------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Row {
    index: i32,
}

impl Row {
    fn new(index: usize) -> Row {
        if index < 1 || 8 < index {
            panic!("index out of bounds for Row");
        }
        let index = (index - 1) as i32;
        Row { index }
    }

    fn index(&self) -> i32 {
        self.index
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl ops::Add<i32> for Row {
    type Output = Option<Row>;

    fn add(self, rhs: i32) -> Option<Row> {
        let index = self.index + rhs;
        if index < 0 || 8 <= index {
            None
        } else {
            Some(Row { index })
        }
    }
}

// =====================================================================

#[cfg(test)]
mod tests {
    use super::Column;
    use super::Position;
    use super::Row;

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_new_under_bound() {
        let c = (b'a' - 1) as char;
        Column::new(c);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn column_nwe_over_bound() {
        let c = (b'h' + 1) as char;
        Column::new(c);
    }

    #[test]
    fn column_index() {
        let mut index = 0;
        for c in 'a'..='h' {
            let col = Column::new(c);
            assert_eq!(col.index(), index);
            index += 1;
        }
    }

    #[test]
    fn column_add_i32() {
        let col = Column::new('a');
        let new_col = col + (-1);
        assert!(new_col.is_none());
        let new_col = col + 0;
        assert_eq!(new_col.unwrap().index(), 0);
        let new_col = col + 1;
        assert_eq!(new_col.unwrap().index(), 1);

        let col = Column::new('h');
        let new_col = col + (-1);
        assert_eq!(new_col.unwrap().index(), 6);
        let new_col = col + 0;
        assert_eq!(new_col.unwrap().index(), 7);
        let new_col = col + 1;
        assert!(new_col.is_none());
    }

    // -----------------------------------------------------------------

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_new_under_bound() {
        Row::new(0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_new_over_bound() {
        Row::new(9);
    }

    #[test]
    fn row_index() {
        let mut index = 0;
        for r in 1..=8 {
            let row = Row::new(r);
            assert_eq!(row.index(), index);
            index += 1;
        }
    }

    #[test]
    fn row_add_i32() {
        let row = Row::new(1);
        let new_row = row + (-1);
        assert!(new_row.is_none());
        let new_row = row + 0;
        assert_eq!(new_row.unwrap().index(), 0);
        let new_row = row + 1;
        assert_eq!(new_row.unwrap().index(), 1);

        let row = Row::new(8);
        let new_row = row + (-1);
        assert_eq!(new_row.unwrap().index(), 6);
        let new_row = row + 0;
        assert_eq!(new_row.unwrap().index(), 7);
        let new_row = row + 1;
        assert!(new_row.is_none());
    }

    // -----------------------------------------------------------------

    #[test]
    fn position_new_and_from() {
        for col in 'a'..='h' {
            for row in 1..=8 {
                let pos1 = Position::new(col, row);
                let col = Column::new(col);
                let row = Row::new(row);
                let pos2 = Position::from((col, row));
                assert_eq!(pos1, pos2);
            }
        }
    }

    #[test]
    fn position_coordinate() {
        for col in 'a'..='h' {
            for row in 1..=8 {
                let pos = Position::new(col, row);
                let col = Column::new(col);
                let row = Row::new(row);
                assert_eq!(pos.coordinate(), (col, row));
            }
        }
    }
}
