use crate::position::Coordinate;
use std::collections::HashMap;

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

pub enum MoveResult {
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
    ) -> Result<Board, MoveResult> {
        if self.get_disk(coord).is_some() {
            return Err(MoveResult::NotEmpty);
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
                    if let Ok(coord) = coord + (dx, dy) {
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
            Ok(board)
        } else {
            Err(MoveResult::NoDiskFlipped)
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
