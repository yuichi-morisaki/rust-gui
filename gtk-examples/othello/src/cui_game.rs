use crate::board::{self, Board, Disk, Side};
use crate::position::Position;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub enum GameStatus {
    GameOver(u8, u8),
    PassBack(u8, u8, Side),
    Continue(u8, u8, Side),
}

pub struct Game {
    root: Rc<State>,
    current: Rc<State>,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();
        let side = Side::Dark;
        let state = State::new(board, side);

        let root = Rc::new(state);
        let current = Rc::clone(&root);

        Game { root, current }
    }

    pub fn new_game(&mut self) {
        self.current = Rc::clone(&self.root);
    }

    pub fn prepare(&mut self) -> GameStatus {
        self.gen_next_moves();

        let (black, white) = self.count();
        if black + white == 64 {
            return GameStatus::GameOver(black, white);
        }

        if self.pass_back() {
            self.gen_next_moves();
            if self.pass_back() {
                GameStatus::GameOver(black, white)
            } else {
                let side = self.current.side();
                GameStatus::PassBack(black, white, side)
            }
        } else {
            let side = self.current.side();
            GameStatus::Continue(black, white, side)
        }
    }

    pub fn render(&self) {
        let board = self.current.board();
        println!("{}", board);
    }

    fn count(&self) -> (u8, u8) {
        let board = self.current.board();
        let mut black = 0;
        let mut white = 0;

        for col in 'a'..='h' {
            for row in 1..=8 {
                let pos = Position::from((col, row));
                match board.get_disk(pos) {
                    None => (),
                    Some(Disk::Black) => black += 1,
                    Some(Disk::White) => white += 1,
                }
            }
        }

        (black, white)
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.current.get_parent() {
            self.current = state;
        }
    }

    fn gen_next_moves(&self) {
        if self.current.has_children() {
            return;
        }

        let board = self.current.board();
        let side = self.current.side();
        let next_side = board::change_turn(side);

        for col in 'a'..='h' {
            for row in 1i8..=8 {
                let pos = Position::from((col, row));
                if let Some(board) = board.try_place(pos, side) {
                    self.current.insert_child(
                        Some(pos),
                        Rc::new(State::new(board, next_side)),
                    );
                    self.current
                        .get_child(Some(pos))
                        .unwrap()
                        .set_parent(Rc::clone(&self.current));
                }
            }
        }

        if !self.current.has_children() {
            let board = *self.current.board();
            self.current
                .insert_child(None, Rc::new(State::new(board, next_side)));
            self.current
                .get_child(None)
                .unwrap()
                .set_parent(Rc::clone(&self.current));
        }
    }

    pub fn place(&mut self, pos: Position) -> bool {
        if let Some(state) = self.current.get_child(Some(pos)) {
            self.current = state;
            true
        } else {
            false
        }
    }

    fn pass_back(&mut self) -> bool {
        if let Some(state) = self.current.get_child(None) {
            self.current = state;
            true
        } else {
            false
        }
    }

    pub fn get_disk_at(&self, pos: Position) -> Option<Disk> {
        self.current.board().get_disk(pos)
    }
}

// --------------------------------------------------------------------

struct State {
    board: Board,
    side: Side,
    parent: RefCell<Weak<State>>,
    children: RefCell<HashMap<Option<Position>, Rc<State>>>,
}

impl State {
    fn new(board: Board, side: Side) -> State {
        State {
            board,
            side,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn side(&self) -> Side {
        self.side
    }

    fn set_parent(&self, state: Rc<State>) {
        *self.parent.borrow_mut() = Rc::downgrade(&state);
    }

    fn get_parent(&self) -> Option<Rc<State>> {
        self.parent.borrow().upgrade()
    }

    fn insert_child(&self, pos: Option<Position>, state: Rc<State>) {
        self.children.borrow_mut().insert(pos, state);
    }

    fn get_child(&self, pos: Option<Position>) -> Option<Rc<State>> {
        let children = self.children.borrow();
        let state = children.get(&pos);
        if state.is_none() {
            None
        } else {
            Some(Rc::clone(state.as_ref().unwrap()))
        }
    }

    fn has_children(&self) -> bool {
        self.children.borrow().len() > 0
    }
}

// ====================================================================

#[cfg(test)]
mod tests {
    use super::State;

    use crate::board::{self, Board, Side};
    use crate::position::Position;

    use std::rc::Rc;

    #[test]
    fn state_new() {
        let board = Board::new();
        let side = Side::Dark;
        let state = State::new(board, side);

        let root = Rc::new(state);
        let current = Rc::clone(&root);

        let pos = Position::from(('f', 5));
        let board = current.board().try_place(pos, side).unwrap();
        let side = board::change_turn(side);

        current.insert_child(Some(pos), Rc::new(State::new(board, side)));
        current
            .get_child(Some(pos))
            .unwrap()
            .set_parent(Rc::clone(&current));

        let current = current.get_child(Some(pos)).unwrap();

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
        assert_eq!(root.board().to_string(), output);

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
5 |   |   |   | x | x | x |   |   |
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
7 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
8 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
";
        assert_eq!(current.board().to_string(), output);
    }
}
