use crate::board::Board;
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

pub struct Engine {
    root: Rc<Node>,
    current: Rc<Node>,
}

impl Engine {
    pub fn new() -> Engine {
        let mut board = Board::new();
        board.init();
        let turn = Side::Dark;

        let node = Node::new(board, turn);
        let root = Rc::new(node);
        let current = Rc::clone(&root);

        Engine { root, current }
    }

    pub fn action(&self, command: Command) {
        match command {
            Command::Init => println!("Debug: init"),
            Command::Quit => println!("Debug: quit"),
            Command::Undo => println!("Debug: undo"),
            Command::Move(row, col) => {
                println!("Debug: move({}, {})", col, row)
            }
        }
    }

    pub fn current_board(&self) -> &Board {
        &self.current.board
    }
}

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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Dark,
    Light,
}
