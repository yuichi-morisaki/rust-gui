use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Fixed, Image};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::rc::Rc;

use othello::{Disk, Game, GameStatus, Position, Side};

fn main() {
    let application =
        Application::new(Some("othello.gtk.rust"), Default::default())
            .expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        let container = Fixed::new();
        window.add(&container);

        let handler = Button::new();
        handler.set_size_request(257, 257);
        container.put(&handler, 2, 2);

        let img_empty = Image::from_file("images/empty.png");
        let img_black = Image::from_file("images/black.png");
        let img_white = Image::from_file("images/white.png");

        let pixbuf_empty = img_empty.get_pixbuf();

        let mut disks = Vec::with_capacity(64);
        for row in 0..8 {
            for col in 0..8 {
                let image = Image::from_pixbuf(pixbuf_empty.as_ref());
                container.put(&image, 32 * col, 32 * row);
                disks.push(image);
            }
        }

        let mut game = GuiGame::new(disks, img_empty, img_black, img_white);
        game.prepare();
        game.render();
        println!("Black's turn");

        let game = Rc::new(RefCell::new(game));
        let game = Rc::clone(&game);

        handler.connect_button_press_event(move |_, button| {
            let (x, y) = button.get_position();
            let col = b"abcdefgh"[x as usize / 32] as char;
            let row = (y as usize / 32 + 1) as i8;
            let pos = Position::from((col, row));

            let mut game = game.borrow_mut();

            if game.place(pos) {
                match game.prepare() {
                    GameStatus::GameOver(black, white) => {
                        game.render();
                        println!("Game is over");
                        println!("Black = {}, White = {}", black, white);
                    }
                    GameStatus::PassBack(black, white, side) => {
                        game.render();
                        println!("Black = {}, White = {}", black, white);
                        println!("Pass back");
                        match side {
                            Side::Dark => println!("Black's turn"),
                            Side::Light => println!("White's turn"),
                        }
                    }
                    GameStatus::Continue(black, white, side) => {
                        game.render();
                        println!("Black = {}, White = {}", black, white);
                        match side {
                            Side::Dark => println!("Black's turn"),
                            Side::Light => println!("White's turn"),
                        }
                    }
                }
            }

            Inhibit(true)
        });

        window.show_all();
    });

    application.run(&[]);
}

struct GuiGame {
    game: Game,
    gui_disks: Vec<Image>,
    img_empty: Image,
    img_black: Image,
    img_white: Image,
}

impl GuiGame {
    fn new(
        gui_disks: Vec<Image>,
        img_empty: Image,
        img_black: Image,
        img_white: Image,
    ) -> GuiGame {
        GuiGame {
            game: Game::new(),
            gui_disks,
            img_empty,
            img_black,
            img_white,
        }
    }

    fn prepare(&mut self) -> GameStatus {
        self.game.prepare()
    }

    fn render(&mut self) {
        for col in 1..=8 {
            for row in 1..=8 {
                let pos = Position::from((col, row));
                let pixbuf = match self.game.get_disk_at(pos) {
                    None => self.img_empty.get_pixbuf(),
                    Some(Disk::Black) => self.img_black.get_pixbuf(),
                    Some(Disk::White) => self.img_white.get_pixbuf(),
                };
                let index = ((row - 1) * 8 + (col - 1)) as usize;
                self.gui_disks[index].set_from_pixbuf(pixbuf.as_ref());
            }
        }
    }

    fn undo(&mut self) {
        self.game.undo();
    }

    fn place(&mut self, pos: Position) -> bool {
        self.game.place(pos)
    }
}

#[allow(dead_code)]
fn cui_main() {
    let mut game = Game::new();

    loop {
        match game.prepare() {
            GameStatus::GameOver(black, white) => {
                game.render();
                println!("Game is over");
                println!("Black = {}, White = {}", black, white);
                match black.cmp(&white) {
                    Ordering::Greater => println!("Black won!"),
                    Ordering::Less => println!("White won!"),
                    Ordering::Equal => println!("Draw"),
                }
                break; // or game.new_game();
            }
            GameStatus::PassBack(black, white, current_side) => {
                game.render();
                println!("Pass back\n");
                println!("Black = {}, White = {}", black, white);
                match current_side {
                    Side::Dark => println!("Black's turn [x]"),
                    Side::Light => println!("White's turn [o]"),
                }
            }
            GameStatus::Continue(black, white, current_side) => {
                game.render();
                println!("Black = {}, White = {}", black, white);
                match current_side {
                    Side::Dark => println!("Black's turn [x]"),
                    Side::Light => println!("White's turn [o]"),
                }
            }
        }

        loop {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");

            let buffer = buffer.trim().as_bytes();

            if buffer == b"undo" {
                game.undo();
                break;
            }

            if buffer.len() != 2 {
                println!("Enter next move again");
                continue;
            }

            let col = buffer[0] as char;
            if col < 'a' || 'h' < col {
                println!("Enter next move again");
                continue;
            }

            let row = buffer[1] as char;
            if row < '1' || '8' < row {
                println!("Enter next move again");
                continue;
            }
            let row = (row as u8 - b'1' + 1) as i8;

            println!("Next move is {}{}", col, row);
            let pos = Position::from((col, row));
            if game.place(pos) {
                break;
            } else {
                println!("Can't place your disk there");
                println!("Enter next move again");
            }
        }
    }
}
