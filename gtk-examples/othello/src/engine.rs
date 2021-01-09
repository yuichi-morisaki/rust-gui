use crate::board::{Board, Disk};
use crate::position::Coordinate;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub enum Command {
    Init,
    Quit,
    Undo,
    Pass,
    Move(char, usize),
}

// ---------------------------------------------------------------------

pub struct Engine {
    root: Rc<Node>,
    current: Rc<Node>,
    pub status: String,
    is_over: bool,
}

impl Engine {
    pub fn new() -> Engine {
        let mut board = Board::new();
        board.init();
        let turn = Side::Dark;

        let node = Node::new(board, turn);
        let root = Rc::new(node);
        let current = Rc::clone(&root);

        let status = String::with_capacity(1024);

        Engine {
            root,
            current,
            status,
            is_over: false,
        }
    }

    pub fn current_board(&self) -> &Board {
        &self.current.board
    }

    pub fn action(&mut self, command: Command) {
        match command {
            Command::Init => self.init(),
            Command::Quit => self.quit(),
            Command::Undo => self.undo(),
            Command::Pass => self.pass(),
            Command::Move(row, col) => {
                let coord = Coordinate::new(row, col);
                self.try_move(coord);
            }
        }
    }

    fn init(&mut self) {
        self.current = Rc::clone(&self.root);
        self.extend_tree();

        self.status.clear();
        self.status += "Game start! ";
        self.append_turn_to_status();
        self.is_over = false;
    }

    fn quit(&self) {
        () // do nothing now
    }

    fn undo(&mut self) {
        self.is_over = false;
        self.status.clear();
        if let Some(node) = self.current.get_parent() {
            self.current = node;
            self.status += "Undid! ";
            if self.current.has_none_key() {
                self.append_pass_to_status();
                return;
            }
        } else {
            self.status += "Couldn't undo! ";
        }
        self.append_turn_to_status();
    }

    fn pass(&mut self) {
        if self.is_over {
            return;
        }

        self.status.clear();
        if let Some(node) = self.current.get_child(None) {
            self.current = node;
            self.extend_tree();
            if self.current.has_none_key() {
                self.current.remove_child(None);
                self.is_over = true;
                self.status += "Game is over! ";
                return;
            }
        } else {
            self.status += "Can't pass! ";
        }
        self.append_turn_to_status();
    }

    fn try_move(&mut self, coord: Coordinate) {
        if self.is_over {
            return;
        }

        self.status.clear();
        if let Some(node) = self.current.get_child(Some(coord)) {
            self.current = node;
            self.extend_tree();
            if self.current.has_none_key() {
                self.append_pass_to_status();
                return;
            }
        } else {
            self.status += "Can't place there! ";
        }
        self.append_turn_to_status();
    }

    fn append_turn_to_status(&mut self) {
        self.status += match self.current.turn {
            Side::Dark => "Black's turn",
            Side::Light => "White's turn",
        };
    }

    fn append_pass_to_status(&mut self) {
        self.status += match self.current.turn {
            Side::Dark => "Black must pass",
            Side::Light => "White must pass",
        }
    }

    fn extend_tree(&self) {
        if self.current.any_child() {
            return;
        }

        let board = &self.current.board;
        let disk = self.current.turn.to_disk();
        let next_turn = change_turn(self.current.turn);

        for col in 'a'..='h' {
            for row in 1..=8 {
                let coord = Coordinate::new(col, row);
                if let Ok(board) = board.try_move(coord, disk) {
                    self.current.insert_child(
                        Some(coord),
                        Rc::new(Node::new(board, next_turn)),
                    );
                    self.current
                        .get_child(Some(coord))
                        .unwrap()
                        .set_parent(Rc::clone(&self.current));
                }
            }
        }

        if !self.current.any_child() {
            let board = self.current.board.clone();
            self.current
                .insert_child(None, Rc::new(Node::new(board, next_turn)));
            self.current
                .get_child(None)
                .unwrap()
                .set_parent(Rc::clone(&self.current));
        }
    }
}

// ---------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Dark,
    Light,
}

impl Side {
    fn to_disk(&self) -> Disk {
        match self {
            Side::Dark => Disk::Black,
            Side::Light => Disk::White,
        }
    }
}

fn change_turn(side: Side) -> Side {
    match side {
        Side::Dark => Side::Light,
        Side::Light => Side::Dark,
    }
}

// ---------------------------------------------------------------------

struct Node {
    pub board: Board,
    pub turn: Side,
    parent: RefCell<Weak<Node>>,
    children: RefCell<HashMap<Option<Coordinate>, Rc<Node>>>,
}

impl Node {
    fn new(board: Board, turn: Side) -> Node {
        Node {
            board,
            turn,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    fn set_parent(&self, parent: Rc<Node>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }

    fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }

    fn insert_child(&self, coord: Option<Coordinate>, node: Rc<Node>) {
        self.children.borrow_mut().insert(coord, node);
    }

    fn get_child(&self, coord: Option<Coordinate>) -> Option<Rc<Node>> {
        if let Some(node) = self.children.borrow().get(&coord) {
            Some(Rc::clone(node))
        } else {
            None
        }
    }

    fn remove_child(&self, coord: Option<Coordinate>) {
        self.children.borrow_mut().remove(&coord);
    }

    fn any_child(&self) -> bool {
        self.num_of_children() > 0
    }

    fn num_of_children(&self) -> usize {
        self.children.borrow().len()
    }

    fn has_none_key(&self) -> bool {
        self.children.borrow().contains_key(&None)
    }
}

// =====================================================================

#[cfg(test)]
mod tests {
    use super::change_turn;
    use super::{Board, Coordinate, Disk};
    use super::{Engine, Node, Side};
    use std::rc::Rc;

    #[test]
    fn node_operations() {
        let mut board = Board::new();
        board.init();
        let turn = Side::Dark;
        let parent = Node::new(board, turn);
        let parent = Rc::new(parent);

        let coord = Coordinate::new('f', 5);
        let board = parent.board.try_move(coord, Disk::Black).unwrap();
        let turn = change_turn(parent.turn);
        let child = Node::new(board, turn);

        parent.insert_child(Some(coord), Rc::new(child));
        parent
            .get_child(Some(coord))
            .unwrap()
            .set_parent(Rc::clone(&parent));

        let child = parent.get_child(Some(coord)).unwrap();
        let output = "\
........ ........ ........ ...ox... ...xxx.. ........ ........ ........ ";
        assert_eq!(child.board.to_string(), output);

        let parent = child.get_parent().unwrap();
        let output = "\
........ ........ ........ ...ox... ...xo... ........ ........ ........ ";
        assert_eq!(parent.board.to_string(), output);

        assert!(parent.get_parent().is_none());
        assert!(parent.any_child());
        assert!(!child.any_child());
    }

    #[test]
    fn engine_extend_tree() {
        let engine = Engine::new();
        engine.extend_tree();
        assert_eq!(engine.current.num_of_children(), 4);

        let coord = Coordinate::new('c', 4);
        let node = engine.current.get_child(Some(coord)).unwrap();
        let output = "\
........ ........ ........ ..xxx... ...xo... ........ ........ ........ ";
        assert_eq!(node.board.to_string(), output);

        let coord = Coordinate::new('d', 3);
        let node = engine.current.get_child(Some(coord)).unwrap();
        let output = "\
........ ........ ...x.... ...xx... ...xo... ........ ........ ........ ";
        assert_eq!(node.board.to_string(), output);

        let coord = Coordinate::new('e', 6);
        let node = engine.current.get_child(Some(coord)).unwrap();
        let output = "\
........ ........ ........ ...ox... ...xx... ....x... ........ ........ ";
        assert_eq!(node.board.to_string(), output);

        let coord = Coordinate::new('f', 5);
        let node = engine.current.get_child(Some(coord)).unwrap();
        let output = "\
........ ........ ........ ...ox... ...xxx.. ........ ........ ........ ";
        assert_eq!(node.board.to_string(), output);
    }
}
