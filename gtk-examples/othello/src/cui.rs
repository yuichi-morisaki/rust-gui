use crate::board::Disk;
use crate::engine::{Command, Engine};
use crate::position::Coordinate;
use std::io::{self, Write};

pub fn run() -> Result<(), &'static str> {
    let mut buffer = String::with_capacity(4096);
    let game = Game::new();

    loop {
        print!("Command? => ");
        if io::stdout().flush().is_err() {
            return Err("Failed to flush in stdout");
        }
        if io::stdin().read_line(&mut buffer).is_err() {
            return Err("Failed to read input");
        }

        let mut iter = buffer.trim().split_whitespace();
        if let Some(command) = iter.next() {
            if command == "quit" {
                game.engine.action(Command::Quit);
                break;
            } else if command == "init" {
                game.engine.action(Command::Init);
                game.render();
            } else if command == "undo" {
                game.engine.action(Command::Undo);
                game.render();
            } else if command == "move" {
                match parse_coordinate(iter.next()) {
                    Ok((col, row)) => {
                        game.engine.action(Command::Move(col, row));
                        game.render();
                    }
                    Err(s) => println!("{}", s),
                }
            } else {
                println!("Unknown command: {}", command);
            }
        }

        buffer.clear();
    }

    Ok(())
}

fn parse_coordinate(
    coord: Option<&str>,
) -> Result<(char, usize), &'static str> {
    if let Some(coord) = coord {
        if coord.len() >= 2 {
            let col = coord.as_bytes()[0] as char;
            let row = &coord.as_bytes()[1..];
            if let Ok(row) = std::str::from_utf8(&row) {
                if let Ok(row) = row.parse::<usize>() {
                    if 'a' <= col && col < 'h' && 1 <= row && row <= 8 {
                        return Ok((col, row));
                    }
                }
            }
        }
    }

    Err("Invalid coordinate")
}

pub struct Game {
    engine: Engine,
}

impl Game {
    pub fn new() -> Game {
        Game {
            engine: Engine::new(),
        }
    }

    pub fn render(&self) {
        let mut output = String::with_capacity(4096);
        let board = self.engine.current_board();

        output += "   a  b  c  d  e  f  g  h\n";
        for row in 1..=8 {
            output += format!("{} ", row).as_str();
            for col in 'a'..='h' {
                let coord = Coordinate::new(col, row);
                let symbol = match board.get_disk(coord) {
                    None => '.',
                    Some(Disk::Black) => 'x',
                    Some(Disk::White) => 'o',
                };
                output += format!(" {} ", symbol).as_str();
            }
            output += "\n";
        }
        println!("{}", output);
    }
}
