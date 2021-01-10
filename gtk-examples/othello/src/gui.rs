use crate::board::Disk;
use crate::engine::{Command, Engine};
use crate::position::Coordinate;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Image;
use gtk::TextBuffer;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run() -> Result<(), &'static str> {
    let app_id = Some("othello.gtk.rust");
    let application = match Application::new(app_id, Default::default()) {
        Ok(app) => app,
        Err(_) => return Err("Failed to initialize GTK application."),
    };

    application.connect_activate(|app| {
        let window = create_application_window(app);
        let images = Images::new();
        let ui = build_ui(&window, &images.empty);

        let disks = Rc::clone(&ui.disks);
        let text = Rc::clone(&ui.text);
        let game = Rc::new(RefCell::new(Game::new(disks, text, images)));
        {
            let mut game = game.borrow_mut();
            game.engine.action(Command::Init);
            game.render();
        }

        let game_clone = Rc::clone(&game);
        ui.init_button.connect_clicked(move |_| {
            let mut game = game_clone.borrow_mut();
            game.engine.action(Command::Init);
            game.render();
        });

        let game_clone = Rc::clone(&game);
        ui.undo_button.connect_clicked(move |_| {
            let mut game = game_clone.borrow_mut();
            game.engine.action(Command::Undo);
            game.render();
        });

        let game_clone = Rc::clone(&game);
        ui.move_button.connect_button_press_event(move |_, button| {
            let mut game = game_clone.borrow_mut();
            let (x, y) = button.get_position();
            let col = b"abcdefgh"[x as usize / 32] as char;
            let row = y as usize / 32 + 1;
            let coord = Coordinate::new(col, row);
            game.engine.action(Command::Move(coord));
            game.render();
            Inhibit(true)
        });

        window.show_all();
    });

    application.run(&[]);

    Ok(())
}

pub struct Game {
    engine: Engine,
    disks: Rc<RefCell<HashMap<Coordinate, Image>>>,
    status_line: Rc<TextBuffer>,
    images: Images,
    buffer: String,
}

impl Game {
    pub fn new(
        disks: Rc<RefCell<HashMap<Coordinate, Image>>>,
        text: Rc<TextBuffer>,
        images: Images,
    ) -> Game {
        Game {
            engine: Engine::new(),
            disks,
            status_line: text,
            images,
            buffer: String::with_capacity(1024),
        }
    }

    pub fn render(&mut self) {
        let board = self.engine.current_board();
        let mut black = 0;
        let mut white = 0;

        for col in 'a'..='h' {
            for row in 1..=8 {
                let coord = Coordinate::new(col, row);
                let image = match board.get_disk(coord) {
                    None => &self.images.empty,
                    Some(Disk::Black) => {
                        black += 1;
                        &self.images.black
                    }
                    Some(Disk::White) => {
                        white += 1;
                        &self.images.white
                    }
                };
                let pixbuf = image.get_pixbuf();
                let disks = self.disks.borrow();
                if let Some(image) = disks.get(&coord) {
                    image.set_from_pixbuf(pixbuf.as_ref());
                }
            }
        }

        self.buffer.clear();
        self.buffer += format!("Black={}, White={}\n", black, white).as_str();
        self.buffer += &self.engine.prompt;
        self.status_line.set_text(&self.buffer);
    }
}

pub struct Images {
    pub empty: Image,
    pub black: Image,
    pub white: Image,
}

impl Images {
    fn new() -> Images {
        Images {
            empty: Image::from_file("images/empty.png"),
            black: Image::from_file("images/black.png"),
            white: Image::from_file("images/white.png"),
        }
    }
}

struct UiParts {
    disks: Rc<RefCell<HashMap<Coordinate, Image>>>,
    init_button: Button,
    undo_button: Button,
    move_button: Button,
    text: Rc<TextBuffer>,
}

fn create_application_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title("Othello");
    window.set_default_size(400, 300);

    window
}

fn build_ui(window: &ApplicationWindow, img_empty: &Image) -> UiParts {
    let frame = gtk::Fixed::new();
    frame.set_margin_top(4);
    window.add(&frame);

    let move_button = Button::new();
    move_button.set_size_request(257, 257);
    frame.put(&move_button, 2, 2);

    let pixbuf = img_empty.get_pixbuf();
    let mut disks = HashMap::with_capacity(64);
    for col in 'a'..='h' {
        for row in 1..=8 {
            let coord = Coordinate::new(col, row);
            let image = Image::from_pixbuf(pixbuf.as_ref());
            let x_pos = (col as u8 - b'a') as i32 * 32;
            let y_pos = (row - 1) as i32 * 32;
            frame.put(&image, x_pos, y_pos);
            disks.insert(coord, image);
        }
    }

    let init_button = Button::with_label("new game");
    frame.put(&init_button, 290, 0);

    let undo_button = Button::with_label("undo");
    frame.put(&undo_button, 290, 50);

    let text_view = gtk::TextView::new();
    let text_buf = text_view.get_buffer().unwrap();
    frame.put(&text_view, 0, 270);

    UiParts {
        disks: Rc::new(RefCell::new(disks)),
        init_button,
        undo_button,
        move_button,
        text: Rc::new(text_buf),
    }
}
