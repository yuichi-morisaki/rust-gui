use crate::position::Coordinate;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Disk {
    Black,
    White,
}

fn flip_disk(disk: &Disk) -> Disk {
    match disk {
        Disk::Black => Disk::White,
        Disk::White => Disk::Black,
    }
}

// ---------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum MoveErr {
    NotEmpty,
    NoDiskFlipped,
}

// ---------------------------------------------------------------------

#[derive(Clone)]
pub struct Board {
    disks: HashMap<Coordinate, Disk>,
    stack: Vec<Coordinate>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            disks: HashMap::with_capacity(64),
            stack: Vec::with_capacity(18),
        }
    }

    pub fn init(&mut self) {
        self.disks.clear();
        self.stack.clear();

        self.place(Coordinate::new('d', 5), Disk::Black);
        self.place(Coordinate::new('e', 4), Disk::Black);
        self.place(Coordinate::new('d', 4), Disk::White);
        self.place(Coordinate::new('e', 5), Disk::White);
    }

    pub fn get_disk(&self, coord: Coordinate) -> Option<Disk> {
        match self.disks.get(&coord) {
            None => None,
            Some(&disk) => Some(disk),
        }
    }

    pub fn try_move(
        &self,
        coord: Coordinate,
        disk: Disk,
    ) -> Result<Board, MoveErr> {
        if self.get_disk(coord).is_some() {
            return Err(MoveErr::NotEmpty);
        }

        let mut board = self.clone();
        let mut num_flip = 0;
        let (current, opponent) = match disk {
            Disk::Black => (Disk::Black, Disk::White),
            Disk::White => (Disk::White, Disk::Black),
        };

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                for offset in 1.. {
                    if let Ok(coord) = coord + (dx * offset, dy * offset) {
                        if let Some(disk) = board.get_disk(coord) {
                            if disk == current {
                                num_flip += board.commit();
                                break;
                            }
                            if disk == opponent {
                                board.flip(coord);
                                continue;
                            }
                        }
                    }
                    board.abort();
                    break;
                }
            }
        }

        if num_flip > 0 {
            board.place(coord, disk);
            Ok(board)
        } else {
            Err(MoveErr::NoDiskFlipped)
        }
    }

    fn abort(&mut self) {
        if let Some(_) = self.undo_flip() {
            self.abort();
        }
    }

    fn commit(&mut self) -> usize {
        let size_of_stack = self.stack.len();
        self.stack.clear();

        size_of_stack
    }

    fn place(&mut self, coord: Coordinate, disk: Disk) {
        if let Some(_) = self.disks.insert(coord, disk) {
            panic!("can't place - not empty");
        }
    }

    fn flip(&mut self, coord: Coordinate) {
        if let Some(disk) = self.disks.get_mut(&coord) {
            *disk = flip_disk(disk);
            self.stack.push(coord);
        } else {
            panic!("can't flip - no disk");
        }
    }

    fn undo_flip(&mut self) -> Option<Coordinate> {
        if let Some(coord) = self.stack.pop() {
            let disk = self.disks.get_mut(&coord).unwrap();
            *disk = flip_disk(disk);
            Some(coord)
        } else {
            None
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 1..=8 {
            for col in 'a'..='h' {
                let coord = Coordinate::new(col, row);
                let symbol = match self.get_disk(coord) {
                    None => '.',
                    Some(Disk::Black) => 'x',
                    Some(Disk::White) => 'o',
                };
                write!(f, "{}", symbol)?;
                if col == 'h' {
                    write!(f, " ")?;
                }
            }
        }

        Ok(())
    }
}

// =====================================================================

#[cfg(test)]
mod tests {
    use super::flip_disk;
    use super::Disk;

    use super::Board;
    use super::Coordinate;
    use super::MoveErr;

    #[test]
    fn flip_disk_and_disk_eq() {
        let black = Disk::Black;
        let white = Disk::White;
        assert_eq!(flip_disk(&black), white);
        assert_eq!(flip_disk(&white), black);
    }

    #[test]
    fn board_display() {
        let mut board = Board::new();
        board.init();
        let output = "\
........ ........ ........ ...ox... ...xo... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    #[should_panic]
    fn board_place_not_empty() {
        let mut board = Board::new();
        board.init();
        board.place(Coordinate::new('d', 4), Disk::Black);
    }

    #[test]
    #[should_panic]
    fn board_flip_at_empty() {
        let mut board = Board::new();
        board.flip(Coordinate::new('d', 4));
    }

    #[test]
    fn board_flip() {
        let mut board = Board::new();
        board.init();

        board.flip(Coordinate::new('d', 4));
        board.flip(Coordinate::new('e', 4));
        board.flip(Coordinate::new('d', 5));
        board.flip(Coordinate::new('e', 5));

        let output = "\
........ ........ ........ ...xo... ...ox... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn board_abort() {
        let mut board = Board::new();
        board.init();

        board.flip(Coordinate::new('d', 4));
        board.flip(Coordinate::new('e', 4));
        board.flip(Coordinate::new('d', 5));
        board.flip(Coordinate::new('e', 5));
        assert_eq!(board.stack.len(), 4);
        let output = "\
........ ........ ........ ...xo... ...ox... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);

        board.abort();
        assert_eq!(board.stack.len(), 0);
        let output = "\
........ ........ ........ ...ox... ...xo... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn board_commit() {
        let mut board = Board::new();
        board.init();

        board.flip(Coordinate::new('d', 4));
        board.flip(Coordinate::new('e', 4));
        board.flip(Coordinate::new('d', 5));
        board.flip(Coordinate::new('e', 5));
        assert_eq!(board.stack.len(), 4);
        let output = "\
........ ........ ........ ...xo... ...ox... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);

        board.commit();
        assert_eq!(board.stack.len(), 0);
        let output = "\
........ ........ ........ ...xo... ...ox... ........ ........ ........ ";
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn board_try_move() {
        let mut board = Board::new();
        board.init();

        let result = board.try_move(Coordinate::new('f', 5), Disk::Black);
        let board = result.unwrap();
        let output = "\
........ ........ ........ ...ox... ...xxx.. ........ ........ ........ ";
        assert_eq!(board.to_string(), output);

        let result = board.try_move(Coordinate::new('f', 5), Disk::White);
        assert_eq!(result.err(), Some(MoveErr::NotEmpty));

        let result = board.try_move(Coordinate::new('f', 4), Disk::Black);
        assert_eq!(result.err(), Some(MoveErr::NoDiskFlipped));
    }
}
