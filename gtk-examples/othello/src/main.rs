use clap::{App, Arg};
use othello::{cui, gui};
use std::process;

fn main() {
    let matches = App::new("Othello")
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .help("Use graphical user interface."),
        )
        .get_matches();

    if matches.is_present("graph") {
        gui::main();
    } else {
        if let Err(err) = cui::run() {
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
    }
}
