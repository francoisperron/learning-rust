use _12_minigrep::config::Config;
use learning_rust::_02_guessing_game;
use std::io;

fn main() {
    println!("Choose wisely");
    println!("1. Guessing game");
    println!("2. Minigrep");
    println!("3. Web server");

    match select() {
        1 => _02_guessing_game::guess(),
        2 => minigrep(),
        3 => _20_web_server::http::server::Server::start("src/book/_20_web_server"),
        _ => println!("Invalid selection"),
    }
}

fn select() -> u32 {
    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: u32 = selection.trim().parse().unwrap_or_else(|_| {
        panic!("Please type a number");
    });
    selection
}

fn minigrep() {
    let config = Config { file_path: "src/book/_12_minigrep/tests/poem.txt".to_string(), query: "to".to_string(), ignore_case: true };
    if let Err(error) = _12_minigrep::minigrep(config) {
        println!("Application error: {:?}", error);
    }
}
