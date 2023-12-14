use std::{env, fs, process};

pub fn init_challenge() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    return fs::read_to_string(file_path).expect("Failed to read file");
}
