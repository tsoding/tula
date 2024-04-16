use std::fs;
use std::result;
use std::fmt::{self, Write};
use std::env;
use std::process::ExitCode;

type Result<T> = result::Result<T, ()>;

struct Lexer<'nsa> {
    source: &'nsa str,
    file_path: &'nsa str,
    pos: usize,
    bol: usize,
    row: usize,
    peek: Option<Symbol<'nsa>>,
}

impl<'nsa> Lexer<'nsa> {
    fn new(source: &'nsa str, file_path: &'nsa str) -> Self {
        Self {
            source,
            file_path,
            pos: 0,
            bol: 0,
            row: 0,
            peek: None,
        }
    }

    fn loc(&self) -> Loc<'nsa> {
        Loc {
            file_path: self.file_path,
            row: self.row + 1,
            col: self.pos - self.bol + 1,
        }
    }

    fn advance_loc(&mut self, skipped_char: char) {
        self.pos += 1;
        if skipped_char == '\n' {
            self.bol = self.pos;
            self.row += 1;
        }
    }

    fn strip_prefix(&mut self, prefix: &str) -> bool {
        if let Some(source) = self.source.strip_prefix(prefix) {
            for x in prefix.chars() {
                self.advance_loc(x);
            }
            self.source = source;
            true
        } else {
            false
        }
    }

    fn strip_while<P>(&mut self, mut skip: P) -> &'nsa str where P: FnMut(&char) -> bool {
        let end = self.source
            .char_indices()
            .find(|(_, x)| {
                if skip(x) {
                    self.advance_loc(*x);
                    false
                } else {
                    true
                }
            })
            .map(|(i, _)| i)
            .unwrap_or(self.source.len());
        let prefix = &self.source[..end];
        self.source = &self.source[end..];
        prefix
    }

    fn chop_symbol(&mut self) -> Option<Symbol<'nsa>> {
        let _ = self.strip_while(|x| x.is_whitespace());

        if self.source.is_empty() {
            return None
        }

        let loc = self.loc();

        let special = &["(", ")", "{", "}", ":"];
        for name in special {
            if self.strip_prefix(name) {
                return Some(Symbol{ name, loc });
            }
        }

        let name = self.strip_while(|x| !x.is_whitespace());
        Some(Symbol { name, loc })
    }

    fn next_symbol(&mut self) -> Option<Symbol<'nsa>> {
        self.peek.take().or_else(|| self.chop_symbol())
    }

    fn peek_symbol(&mut self) -> Option<Symbol<'nsa>> {
        if self.peek.is_none() {
            self.peek = self.chop_symbol();
        }
        self.peek
    }
}

#[derive(Debug, Clone, Copy)]
struct Loc<'nsa> {
    file_path: &'nsa str,
    row: usize,
    col: usize,
}

impl<'nsa> fmt::Display for Loc<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Loc{file_path, row, col} = self;
        write!(f, "{file_path}:{row}:{col}")
    }
}

#[derive(Debug, Clone, Copy)]
struct Symbol<'nsa> {
    name: &'nsa str,
    loc: Loc<'nsa>,
}

#[derive(Debug)]
struct Case<'nsa> {
    state: Symbol<'nsa>,
    read: Symbol<'nsa>,
    write: Symbol<'nsa>,
    step: Symbol<'nsa>,
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
            if case.state.name == self.state.name && case.read.name == self.tape[self.head].name {
                self.tape[self.head] = case.write;
                match case.step.name {
                    "<-" => {
                        if self.head == 0 {
                            eprintln!("{loc}: ERROR: tape underflow", loc = case.step.loc);
                            return Err(());
                        }
                        self.head -= 1;
                    }
                    "->" => {
                        self.head += 1;
                        if self.head >= self.tape.len() {
                            self.tape.push(self.tape_default.clone());
                        }
                    }
                    _ => {
                        unreachable!("Parser didn't parse the arrow correctly");
                    }
                }
                self.state = case.next;
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

fn parse_symbol<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Symbol<'nsa>> {
    if let Some(symbol) = lexer.next_symbol() {
        Ok(symbol)
    } else {
        eprintln!("{loc}: ERROR: expected symbol but reached the end of the input", loc = lexer.loc());
        Err(())
    }
}

fn parse_step<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Symbol<'nsa>> {
    let symbol = parse_symbol(lexer)?;
    match symbol.name {
        "->" | "<-" => Ok(symbol),
        name => {
            eprintln!("{loc}: ERROR: expected -> or <- but got {name}", loc = symbol.loc);
            Err(())
        }
    }
}

fn parse_case<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Case<'nsa>> {
    let state = parse_symbol(lexer)?;
    let read = parse_symbol(lexer)?;
    let write = parse_symbol(lexer)?;
    let step = parse_step(lexer)?;
    let next = parse_symbol(lexer)?;
    Ok(Case{state, read, write, step, next})
}

fn parse_cases<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Case<'nsa>>> {
    let mut cases = vec![];
    while lexer.peek_symbol().is_some() {
        cases.push(parse_case(lexer)?);
    }
    Ok(cases)
}

fn parse_tape<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Symbol<'nsa>>> {
    let mut symbols = vec![];
    while lexer.peek_symbol().is_some() {
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
    let cases = parse_cases(&mut Lexer::new(&tula_source, &tula_path))?;
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
    let tape = parse_tape(&mut Lexer::new(&tape_source, &tape_path))?;

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
