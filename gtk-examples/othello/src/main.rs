use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) if arg == "cui" => cui::main(),
        Some(arg) if arg == "gui" => gui::main(),
        Some(arg) => panic!("Unknown argument: {}", arg),
        None => gui::main(),
    }
}

mod cui {
    use othello::{CuiGame, GameStatus, Position, Side};

    use std::cmp::Ordering;
    use std::io::{self, Write};

    pub fn main() {
        let mut game = CuiGame::new();

        loop {
            match game.prepare() {
                GameStatus::GameOver(black, white) => {
                    game.render();
                    write_result(black, white);

                    if play_again() {
                        game.new_game();
                        game.render();
                    } else {
                        break;
                    }
                }
                GameStatus::PassBack(black, white, side) => {
                    game.render();
                    write_status(black, white, side, true);
                }
                GameStatus::Continue(black, white, side) => {
                    game.render();
                    write_status(black, white, side, false);
                }
            }

            loop {
                match read_input() {
                    Input::Undo => {
                        game.undo();
                        break;
                    }
                    Input::Place(pos) => {
                        if game.place(pos) {
                            break;
                        } else {
                            println!("Can't place there!");
                        }
                    }
                }
            }
        }
    }

    enum Input {
        Undo,
        Place(Position),
    }

    fn read_input() -> Input {
        loop {
            print!("\nEnter next move => ");
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");

            let bytes = buffer.trim().as_bytes();

            if bytes == b"undo" {
                return Input::Undo;
            }

            if bytes.len() != 2 {
                println!("Invalid input!");
                continue;
            }

            let col = bytes[0] as char;
            let row = bytes[1] as char;
            let pos = match position_from(col, row) {
                Some(pos) => pos,
                None => continue,
            };
            println!("Next move is {}{}", col, row);

            return Input::Place(pos);
        }
    }

    fn position_from(col: char, row: char) -> Option<Position> {
        if col < 'a' || 'h' < col {
            println!("column must be 'a' to 'h'!");
            return None;
        }
        if row < '1' || '8' < row {
            println!("row must be '1' to '8'!");
            return None;
        }
        let row = (row as u8 - b'0') as i8;

        Some(Position::from((col, row)))
    }

    fn write_score(black: u8, white: u8) {
        println!("Black = {}, White = {}", black, white);
    }

    fn write_result(black: u8, white: u8) {
        println!("Game is over!");
        write_score(black, white);
        match black.cmp(&white) {
            Ordering::Greater => println!("Black won!"),
            Ordering::Less => println!("White won!"),
            Ordering::Equal => println!("Draw"),
        }
    }

    fn write_status(black: u8, white: u8, side: Side, pass: bool) {
        write_score(black, white);
        if pass {
            print!("Pass back, and ");
        }
        match side {
            Side::Dark => println!("Black's turn [x]"),
            Side::Light => println!("White's turn [o]"),
        }
    }

    fn play_again() -> bool {
        print!("Do you want to play again? [y/n] => ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        if buffer.len() > 0 {
            let c = buffer.as_bytes()[0];
            if c == b'y' || c == b'Y' {
                return true;
            }
        }

        false
    }
}

mod gui {
    use othello::{GameStatus, GuiGame, Images, Position, Side};

    use gio::prelude::*;
    use gtk::prelude::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn main() {
        let application =
            gtk::Application::new(Some("othello.gtk.rust"), Default::default())
                .expect("Failed to initialize GTK application");

        application.connect_activate(|app| {
            let images = Images::new();
            let ui = build_ui(app, &images.empty);
            let text = Rc::clone(&ui.text);
            let game = Rc::new(RefCell::new(GuiGame::new(images)));

            {
                let mut game = game.borrow_mut();
                let disks = ui.disks.borrow();
                game.prepare();
                game.render(disks);
                text.set_text("Black's turn");
            }

            let game1 = Rc::clone(&game);
            let disks1 = Rc::clone(&ui.disks);

            ui.handler.connect_button_press_event(move |_, button| {
                let (x, y) = button.get_position();
                let col = b"abcdefgh"[x as usize / 32] as char;
                let row = (y as usize / 32 + 1) as i8;
                let pos = Position::from((col, row));

                let mut game = game1.borrow_mut();
                let disks = disks1.borrow();

                if game.place(pos) {
                    match game.prepare() {
                        GameStatus::GameOver(black, white) => {
                            game.render(disks);
                            let msg = format!(
                                "Black = {}, White = {}\nGame is over.",
                                black, white
                            );
                            text.set_text(&msg);
                        }
                        GameStatus::PassBack(black, white, side) => {
                            game.render(disks);
                            let mut msg = format!(
                                "Black = {}, White = {}\nPass back, and ",
                                black, white
                            );
                            match side {
                                Side::Dark => msg += "Black's turn",
                                Side::Light => msg += "White's turn",
                            }
                            text.set_text(&msg);
                        }
                        GameStatus::Continue(black, white, side) => {
                            game.render(disks);
                            let mut msg = format!(
                                "Black = {}, White = {}\n",
                                black, white
                            );
                            text.set_text(&msg);
                            match side {
                                Side::Dark => msg += "Black's turn",
                                Side::Light => msg += "White's turn",
                            }
                            text.set_text(&msg);
                        }
                    }
                }

                Inhibit(true)
            });

            let game2 = Rc::clone(&game);
            let disks2 = Rc::clone(&ui.disks);

            ui.undo_button.connect_clicked(move |_| {
                let mut game = game2.borrow_mut();
                let disks = disks2.borrow();
                game.undo();
                game.render(disks);
            });

            ui.window.show_all();
        });

        application.run(&[]);
    }

    struct UiParts {
        window: gtk::ApplicationWindow,
        // container: gtk::Fixed,
        text: Rc<gtk::TextBuffer>,
        handler: gtk::Button,
        undo_button: gtk::Button,
        disks: Rc<RefCell<Vec<gtk::Image>>>,
    }

    fn build_ui(app: &gtk::Application, image: &gtk::Image) -> UiParts {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Othello");
        window.set_default_size(400, 300);

        let container = gtk::Fixed::new();
        container.set_margin_top(4);
        window.add(&container);

        let text_view = gtk::TextView::new();
        let text_buf = text_view.get_buffer().unwrap();
        container.put(&text_view, 0, 270);

        let handler = gtk::Button::new();
        handler.set_size_request(257, 257);
        container.put(&handler, 2, 2);

        let undo_button = gtk::Button::with_label("Undo!");
        container.put(&undo_button, 290, 0);

        let pixbuf = image.get_pixbuf();
        let mut disks = Vec::with_capacity(64);
        for row in 0..8 {
            for col in 0..8 {
                let image = gtk::Image::from_pixbuf(pixbuf.as_ref());
                container.put(&image, 32 * col, 32 * row);
                disks.push(image);
            }
        }

        UiParts {
            window,
            // container,
            text: Rc::new(text_buf),
            handler,
            undo_button,
            disks: Rc::new(RefCell::new(disks)),
        }
    }
}
