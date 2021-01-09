use crate::board::{Board, Disk};
use crate::position::Coordinate;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub enum Command {
    Init,
    Quit,
    Undo,
    Move(char, usize),
}

// ---------------------------------------------------------------------

pub struct Engine {
    root: Rc<Node>,
    current: Rc<Node>,
    pub prompt: String,
}

impl Engine {
    pub fn new() -> Engine {
        let mut board = Board::new();
        board.init();
        let turn = Side::Dark;

        let root = Rc::new(Node::new(board, turn));
        let current = Rc::clone(&root);

        Engine {
            root,
            current,
            prompt: String::with_capacity(1024),
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
            Command::Move(row, col) => {
                let coord = Coordinate::new(row, col);
                self.try_move(coord);
            }
        }
    }

    fn init(&mut self) {
        self.current = Rc::clone(&self.root);
        self.extend_tree();

        self.update_status(Some("Game start!"));
    }

    fn try_move(&mut self, coord: Coordinate) {
        if !self.current.has_any_child() {
            return;
        }

        if let Some(node) = self.current.get_child(Some(coord)) {
            self.current = node;
            self.extend_tree();

            if let Some(node) = self.current.get_child(None) {
                self.current = node;
                self.extend_tree();

                if let Some(_node) = self.current.get_child(None) {
                    self.current.remove_child(None);
                    self.current = self.current.get_parent().unwrap();
                    self.current.remove_child(None);
                    self.update_status(None);
                    return;
                }
            }
            self.update_status(None);
        } else {
            self.update_status(Some("Can't place there!"));
        }
    }

    fn quit(&self) {
        () // do nothing now
    }

    fn undo(&mut self) {
        if let Some(parent) = self.current.get_parent() {
            self.current = parent;
            if let Some(_node) = self.current.get_child(None) {
                let parent = self.current.get_parent().unwrap();
                self.current = parent;
            }
            self.update_status(Some("Undo, and "));
        } else {
            self.update_status(Some("Can't undo!"));
        }
    }

    fn update_status(&mut self, msg: Option<&str>) {
        self.prompt.clear();

        if let Some(msg) = msg {
            self.prompt += msg;
            self.prompt += " ";
        }

        if !self.current.has_any_child() {
            self.prompt += "Game is over!";
        } else {
            if let Some(parent) = self.current.get_parent() {
                if let Some(_node) = parent.get_child(None) {
                    self.prompt += match parent.turn {
                        Side::Dark => "Black passed, and ",
                        Side::Light => "White passed, and ",
                    };
                }
            }
            self.prompt += match self.current.turn {
                Side::Dark => "Black's turn.",
                Side::Light => "White's turn.",
            };
        }
    }

    fn extend_tree(&self) {
        if self.current.has_any_child() {
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

        if !self.current.has_any_child() {
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

    fn has_any_child(&self) -> bool {
        self.num_of_children() > 0
    }

    fn num_of_children(&self) -> usize {
        self.children.borrow().len()
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
        assert!(parent.has_any_child());
        assert!(!child.has_any_child());
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
