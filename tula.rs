use std::fs;
use std::result;
use std::fmt::Write;
use std::env;
use std::iter::Peekable;
use std::process::ExitCode;

type Result<T> = result::Result<T, ()>;

#[derive(Debug)]
enum Step {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Symbol<'nsa> {
    name: &'nsa str,
}

#[derive(Debug)]
struct Case<'nsa> {
    state: Symbol<'nsa>,
    read: Symbol<'nsa>,
    write: Symbol<'nsa>,
    step: Step,
    next: Symbol<'nsa>,
}

#[derive(Debug)]
struct Machine<'nsa> {
    state: Symbol<'nsa>,
    tape: Vec<Symbol<'nsa>>,
    tape_default: Symbol<'nsa>,
    head: usize,
    halt: bool,
}

impl<'a> Machine<'a> {
    fn next(&mut self, cases: &[Case<'a>]) -> Result<()> {
        for case in cases {
            if case.state == self.state && case.read == self.tape[self.head] {
                self.tape[self.head].name = case.write.name;
                match case.step {
                    Step::Left => {
                        if self.head == 0 {
                            eprintln!("ERROR: tape underflow");
                            return Err(());
                        }
                        self.head -= 1;
                    }
                    Step::Right => {
                        self.head += 1;
                        if self.head >= self.tape.len() {
                            self.tape.push(self.tape_default.clone());
                        }
                    }
                }
                self.state.name = case.next.name;
                self.halt = false;
                break;
            }
        }
        Ok(())
    }

    fn print(&self) {
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{state}: ", state = self.state.name);
        let mut head = 0;
        for (i, symbol) in self.tape.iter().enumerate() {
            if i == self.head {
                head = buffer.len();
            }
            let _ = write!(&mut buffer, "{name} ", name = symbol.name);
        }
        println!("{buffer}");
        // TODO: use the field width formating magic or something like that
        for _ in 0..head {
            print!(" ");
        }
        println!("^");
    }
}

fn parse_symbol<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Symbol<'a>> {
    if let Some(name) = lexer.next() {
        Ok(Symbol{name})
    } else {
        eprintln!("ERROR: expected symbol but reached the end of the input");
        Err(())
    }
}

fn parse_step<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Step> {
    let symbol = parse_symbol(lexer)?;
    match symbol.name {
        "->" => Ok(Step::Right),
        "<-" => Ok(Step::Left),
        name => {
            eprintln!("ERROR: expected -> or <- but got {name}");
            Err(())
        }
    }
}

fn parse_case<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Case<'a>> {
    let state = parse_symbol(lexer)?;
    let read = parse_symbol(lexer)?;
    let write = parse_symbol(lexer)?;
    let step = parse_step(lexer)?;
    let next = parse_symbol(lexer)?;
    Ok(Case{state, read, write, step, next})
}

fn parse_cases<'a>(lexer: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Case<'a>>> {
    let mut cases = vec![];
    while lexer.peek().is_some() {
        cases.push(parse_case(lexer)?);
    }
    Ok(cases)
}

fn parse_tape<'a>(lexer: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Symbol<'a>>> {
    let mut symbols = vec![];
    while lexer.peek().is_some() {
        symbols.push(parse_symbol(lexer)?);
    }
    Ok(symbols)
}

fn usage(program: &str) {
    eprintln!("Usage: {program} <input.tula> <input.tape>");
}

fn start() -> Result<()> {
    let mut args = env::args();
    let program = args.next().expect("Program name is alway present");

    let tula_path;
    if let Some(path) = args.next() {
        tula_path = path;
    } else {
        usage(&program);
        eprintln!("ERROR: no input.tula is provided");
        return Err(());
    }
    let tula_source = fs::read_to_string(&tula_path).map_err(|err| {
        eprintln!("ERROR: could not read file {tula_path}: {err}");
    })?;
    let cases = parse_cases(&mut tula_source.split(&[' ', '\n']).filter(|t| t.len() > 0).peekable())?;
    let state;
    if let Some(case) = cases.first() {
        state = case.state;
    } else {
        eprintln!("ERROR: The tule file must have at least one case");
        return Err(());
    }

    let tape_path;
    if let Some(path) = args.next() {
        tape_path = path;
    } else {
        usage(&program);
        eprintln!("ERROR: no input.tape is provided");
        return Err(());
    }
    let tape_source = fs::read_to_string(&tape_path).map_err(|err| {
        eprintln!("ERROR: could not read file {tape_path}: {err}");
    })?;
    let tape = parse_tape(&mut tape_source.split(&[' ', '\n']).filter(|t| t.len() > 0).peekable())?;

    let tape_default;
    if let Some(symbol) = tape.last().cloned() {
        tape_default = symbol;
    } else {
        eprintln!("ERROR: The tape file may not be empty. I must contain at least one symbol so we know what to fill the infinite tape with");
        return Err(());
    }

    let mut machine = Machine {
        state,
        tape,
        tape_default,
        head: 0,
        halt: false,
    };

    while !machine.halt {
        machine.print();
        machine.halt = true;
        machine.next(&cases)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
