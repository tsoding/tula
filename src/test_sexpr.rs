mod sexpr;
mod lexer;

use std::{env, fs, result};
use std::process::ExitCode;

use sexpr::*;
use lexer::*;

type Result<T> = result::Result<T, ()>;

fn start() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().expect("Program name is always provided");

    let source_path;
    if let Some(arg) = args.next() {
        source_path = arg;
    } else {
        eprintln!("Usage: {program_name} <input>");
        eprintln!("ERROR: no input is provided");
        return Err(());
    }

    let source = fs::read_to_string(&source_path).map_err(|err| {
        eprintln!("ERROR: could not read file {source_path}: {err}");
    })?;

    let mut lexer = Lexer::new(&source, &source_path);
    let sexpr = Sexpr::parse(&mut lexer)?;
    sexpr.dump(0);

    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
