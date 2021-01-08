use std::hash::Hash;
use std::ops;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Coordinate {
    col: Column,
    row: Row,
}

impl Coordinate {
    pub fn new(col: char, row: usize) -> Coordinate {
        Coordinate {
            col: Column::new(col),
            row: Row::new(row),
        }
    }
}

impl ops::Add<(i32, i32)> for Coordinate {
    type Output = Result<Coordinate, ()>;

    fn add(self, (delta_col, delta_row): (i32, i32)) -> Self::Output {
        match (self.col + delta_col, self.row + delta_row) {
            (Ok(col), Ok(row)) => Ok(Coordinate { col, row }),
            (_, _) => Err(()),
        }
    }
}

impl Eq for Coordinate {}

// ---------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Column(char);

impl Column {
    fn new(index: char) -> Column {
        if index < 'a' || 'h' < index {
            panic!("index out of bounds for Column");
        }

        Column(index)
    }
}

impl ops::Add<i32> for Column {
    type Output = Result<Column, ()>;

    fn add(self, rhs: i32) -> Self::Output {
        let index = (self.0 as u8) as i32;
        let index = ((index + rhs) as u8) as char;

        if 'a' <= index && index <= 'h' {
            Ok(Column(index))
        } else {
            Err(())
        }
    }
}

// ---------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Row(usize);

impl Row {
    fn new(index: usize) -> Row {
        if index < 1 || 8 < index {
            panic!("index out of bounds for Row");
        }

        Row(index)
    }
}

impl ops::Add<i32> for Row {
    type Output = Result<Row, ()>;

    fn add(self, rhs: i32) -> Self::Output {
        let index = self.0 as i32;
        let index = (index + rhs) as usize;

        if 1 <= index && index <= 8 {
            Ok(Row(index))
        } else {
            Err(())
        }
    }
}

// =====================================================================

#[cfg(test)]
mod tests {
    use super::Column;
    use super::Row;

    use super::Coordinate;

    #[test]
    fn coordinate_ops_add() {
        let d4 = Coordinate::new('d', 4);

        let e4 = (d4 + (1, 0)).unwrap();
        assert_eq!(e4, Coordinate::new('e', 4));
        let c4 = (d4 + (-1, 0)).unwrap();
        assert_eq!(c4, Coordinate::new('c', 4));

        let d5 = (d4 + (0, 1)).unwrap();
        assert_eq!(d5, Coordinate::new('d', 5));
        let d3 = (d4 + (0, -1)).unwrap();
        assert_eq!(d3, Coordinate::new('d', 3));

        assert!((d4 + (5, 0)).is_err());
        assert!((d4 + (-4, 0)).is_err());
        assert!((d4 + (0, 5)).is_err());
        assert!((d4 + (0, -4)).is_err());
    }

    // ---------------------------------------------------------

    #[test]
    #[should_panic]
    fn column_new_under_bound() {
        let c = (b'a' - 1) as char;
        Column::new(c);
    }

    #[test]
    #[should_panic]
    fn column_new_over_bound() {
        Column::new('i');
    }

    #[test]
    fn column_new() {
        for index in 'a'..='h' {
            Column::new(index);
        }
    }

    #[test]
    fn column_ops_add() {
        let col_e = Column::new('e');

        let col_f = (col_e + 1).unwrap();
        assert_eq!(col_f, Column::new('f'));
        let col_h = (col_e + 3).unwrap();
        assert_eq!(col_h, Column::new('h'));
        assert!((col_e + 4).is_err());

        let col_d = (col_e + (-1)).unwrap();
        assert_eq!(col_d, Column::new('d'));
        let col_a = (col_e + (-4)).unwrap();
        assert_eq!(col_a, Column::new('a'));
        assert!((col_e + (-5)).is_err());
    }

    // ---------------------------------------------------------

    #[test]
    #[should_panic]
    fn row_new_under_bound() {
        Row::new(0);
    }

    #[test]
    #[should_panic]
    fn row_new_over_bound() {
        Row::new(9);
    }

    #[test]
    fn row_new() {
        for index in 1..=8 {
            Row::new(index);
        }
    }

    #[test]
    fn row_ops_add() {
        let row4 = Row::new(4);

        let row5 = (row4 + 1).unwrap();
        assert_eq!(row5, Row::new(5));
        let row8 = (row4 + 4).unwrap();
        assert_eq!(row8, Row::new(8));
        assert!((row4 + 5).is_err());

        let row3 = (row4 + (-1)).unwrap();
        assert_eq!(row3, Row::new(3));
        let row1 = (row4 + (-3)).unwrap();
        assert_eq!(row1, Row::new(1));
        assert!((row4 + (-4)).is_err());
    }
}
