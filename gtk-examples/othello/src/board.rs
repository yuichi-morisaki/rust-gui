use crate::position::Position;

use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Disk {
    Black = 0,
    White = 1,
}

impl PartialEq for Disk {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

// --------------------------------------------------------------------

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Side {
    Dark = 0,
    Light = 1,
}

pub fn change_turn(current: Side) -> Side {
    match current {
        Side::Dark => Side::Light,
        Side::Light => Side::Dark,
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Board {
    disks: [Option<Disk>; 100],
}

impl Board {
    pub fn new() -> Board {
        let mut disks = [None; 100];

        disks[Position::from(('d', 4)).index()] = Some(Disk::White);
        disks[Position::from(('d', 5)).index()] = Some(Disk::Black);
        disks[Position::from(('e', 4)).index()] = Some(Disk::Black);
        disks[Position::from(('e', 5)).index()] = Some(Disk::White);

        Board { disks }
    }

    pub fn set_disk(&mut self, pos: Position, disk: Disk) {
        self.disks[pos.index()] = Some(disk);
    }

    pub fn get_disk(&self, pos: Position) -> Option<Disk> {
        self.disks[pos.index()]
    }

    pub fn try_place(&self, pos: Position, side: Side) -> Option<Board> {
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

        let mut board = *self;
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

// ====================================================================

#[cfg(test)]
mod tests {
    use crate::position::Position;

    use super::Side;

    use super::Board;

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
}
