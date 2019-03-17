use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);

    // cargo run hello
    let command = args[1].clone();
    if command == "hello" {
        println!("Command: {}", command);
    }
}