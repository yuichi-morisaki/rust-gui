mod board;
mod cui_game;
mod position;

pub use board::Side;
pub use cui_game::{Game as CuiGame, GameStatus};
pub use position::Position;

use board::Disk;

use gtk::prelude::*;
use gtk::Image;

use std::cell::Ref;

pub struct Images {
    pub empty: Image,
    black: Image,
    white: Image,
}

impl Images {
    pub fn new() -> Images {
        Images {
            empty: Image::from_file("images/empty.png"),
            black: Image::from_file("images/black.png"),
            white: Image::from_file("images/white.png"),
        }
    }
}

pub struct GuiGame {
    game: CuiGame,
    images: Images,
}

impl GuiGame {
    pub fn new(images: Images) -> GuiGame {
        GuiGame {
            game: CuiGame::new(),
            images,
        }
    }

    pub fn prepare(&mut self) -> GameStatus {
        self.game.prepare()
    }

    pub fn render(&self, disks: Ref<Vec<Image>>) {
        for col in 1..=8 {
            for row in 1..=8 {
                let pos = Position::from((col, row));
                let pixbuf = match self.game.get_disk_at(pos) {
                    None => self.images.empty.get_pixbuf(),
                    Some(Disk::Black) => self.images.black.get_pixbuf(),
                    Some(Disk::White) => self.images.white.get_pixbuf(),
                };
                let index = ((row - 1) * 8 + (col - 1)) as usize;
                disks[index].set_from_pixbuf(pixbuf.as_ref());
            }
        }
    }

    pub fn undo(&mut self) {
        self.game.undo();
    }

    pub fn place(&mut self, pos: Position) -> bool {
        self.game.place(pos)
    }
}
