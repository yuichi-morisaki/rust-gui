use std::cmp::Ordering;
use std::io;

use othello::{Game, GameStatus, Position, Side};

fn main() {
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
