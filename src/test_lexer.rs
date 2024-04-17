mod lexer;

use std::{fs, env, result};
use lexer::*;
use std::process::ExitCode;

type Result<T> = result::Result<T, ()>;

fn start() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().expect("Program name is always provided");

    let input_path;
    if let Some(arg) = args.next() {
        input_path = arg;
    } else {
        eprintln!("Usage: {program_name} <input>");
        eprintln!("ERROR: no input is provided");
        return Err(());
    }

    let input_source = fs::read_to_string(&input_path).map_err(|err| {
        eprintln!("ERROR: could not read file {input_path}: {err}");
    })?;

    for Symbol{loc, name} in Lexer::new(&input_source, &input_path) {
        println!("{loc}: {name}");
    }

    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
