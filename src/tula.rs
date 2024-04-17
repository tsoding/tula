mod lexer;

use std::fs;
use std::result;
use std::fmt::Write;
use std::env;
use std::process::ExitCode;
use std::collections::HashMap;

use lexer::*;

type Result<T> = result::Result<T, ()>;

#[derive(Debug)]
struct Case<'nsa> {
    state: Symbol<'nsa>,
    read: Symbol<'nsa>,
    write: Symbol<'nsa>,
    step: Symbol<'nsa>,
    next: Symbol<'nsa>,
}

#[derive(Debug)]
enum Statement<'nsa> {
    Case(Case<'nsa>),
    For {
        var: Symbol<'nsa>,
        set: Symbol<'nsa>,
        body: Box<Statement<'nsa>>,
    }
}

impl<'nsa> Statement<'nsa> {
    fn entry_state(&self, program: &Program<'nsa>) -> Result<Option<Symbol<'nsa>>> {
        match self {
            Statement::Case(case) => Ok(Some(case.state)),
            Statement::For{var, set, body} => {
                if let Some(symbols) = program.sets.get(set.name) {
                    if let Some(symbol) = symbols.first() {
                        body.substitude(*var, *symbol).entry_state(program)
                    } else {
                        Ok(None)
                    }
                } else {
                    eprintln!("{loc}: ERROR: unknown set {name}", loc = set.loc, name = set.name);
                    Err(())
                }
            }
        }
    }

    fn substitude(&self, var: Symbol<'nsa>, symbol: Symbol<'nsa>) -> Statement<'nsa> {
        match self {
            &Statement::Case(Case{state, read, write, step, next}) => {
                let state = if state.name == var.name {symbol} else {state};
                let read  = if read.name  == var.name {symbol} else {read};
                // TODO: Implement support for substituting step.
                // We need to update the parser to enable that.
                // We also need to do some runtime checks to ensure that only things from the Arrow set are put in the step
                // let Arrow { -> <- }
                let write = if write.name == var.name {symbol} else {write};
                let next  = if next.name  == var.name {symbol} else {next};
                Statement::Case(Case{state, read, write, step, next})
            }
            Statement::For{var, set, body} => {
                let var = *var;
                let set = if set.name == var.name {symbol} else {*set};
                let body = Box::new(body.substitude(var, set));
                Statement::For{var, set, body}
            }
        }
    }

    fn match_state(&self, program: &Program<'nsa>, state: Symbol<'nsa>, read: Symbol<'nsa>) -> Result<Option<(Symbol<'nsa>, Symbol<'nsa>, Symbol<'nsa>)>> {
        match self {
            Statement::Case(case) => {
                if case.state.name == state.name && case.read.name == read.name {
                    Ok(Some((case.write, case.step, case.next)))
                } else {
                    Ok(None)
                }
            }
            Statement::For{var, set, body} => {
                if let Some(symbols) = program.sets.get(set.name) {
                    for symbol in symbols {
                        if let Some(triple) = body.substitude(*var, *symbol).match_state(program, state, read)? {
                            return Ok(Some(triple));
                        }
                    }
                    Ok(None)
                } else {
                    eprintln!("{loc}: ERROR: unknown set {name}", loc = set.loc, name = set.name);
                    Err(())
                }
            }
        }
    }
}

#[derive(Debug)]
struct Machine<'nsa> {
    state: Symbol<'nsa>,
    tape: Vec<Symbol<'nsa>>,
    tape_default: Symbol<'nsa>,
    head: usize,
    halt: bool,
}

impl<'nsa> Machine<'nsa> {
    fn next(&mut self, program: &Program<'nsa>) -> Result<()> {
        for statement in program.statements.iter() {
            if let Some((write, step, next)) = statement.match_state(program, self.state, self.tape[self.head])? {
                self.tape[self.head] = write;
                match step.name {
                    "<-" => {
                        if self.head == 0 {
                            eprintln!("{loc}: ERROR: tape underflow", loc = step.loc);
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
                self.state = next;
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

fn parse_set<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Symbol<'nsa>>> {
    let _ = lexer.expect_symbols(&["{"])?;
    let mut set = vec![];
    while let Some(symbol) = lexer.next_symbol() {
        if symbol.name == "}" {
            break;
        }
        set.push(symbol);
    }
    Ok(set)
}

#[derive(Default)]
struct Program<'nsa> {
    statements: Vec<Statement<'nsa>>,
    sets: HashMap<&'nsa str, Vec<Symbol<'nsa>>>,
}

impl<'nsa> Program<'nsa> {
    fn entry_state(&self) -> Result<Option<Symbol<'nsa>>> {
        for statement in self.statements.iter() {
            if let Some(state) = statement.entry_state(self)? {
                return Ok(Some(state))
            }
        }
        Ok(None)
    }
}

fn parse_case<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Case<'nsa>> {
    let state = lexer.parse_symbol()?;
    let read  = lexer.parse_symbol()?;
    let write = lexer.parse_symbol()?;
    let step  = lexer.expect_symbols(&["->", "<-"])?;
    let next  = lexer.parse_symbol()?;
    Ok(Case{state, read, write, step, next})
}

fn parse_statement<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Statement<'nsa>> {
    let key = lexer.expect_symbols(&["case", "for"])?;
    match key.name {
        "case" => Ok(Statement::Case(parse_case(lexer)?)),
        "for" => {
            let var = lexer.parse_symbol()?;
            let _ = lexer.expect_symbols(&["in"])?;
            let set = lexer.parse_symbol()?;
            let body = Box::new(parse_statement(lexer)?);
            Ok(Statement::For{var, set, body})
        }
        _ => unreachable!()
    }
}

fn parse_program<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Program<'nsa>> {
    let mut program = Program::default();
    while let Some(key) = lexer.peek_symbol() {
        match key.name {
            "case" | "for" => {
                program.statements.push(parse_statement(lexer)?);
            }
            "let" => {
                lexer.next_symbol();
                let Symbol{name, loc} = lexer.parse_symbol()?;
                if program.sets.contains_key(name) {
                    eprintln!("{loc}: ERROR: redefinition of set {name}");
                    return Err(())
                }
                let set = parse_set(lexer)?;
                program.sets.insert(name, set);
            }
            _ => {
                eprintln!("{loc}: ERROR: unknown keyword {name}", loc = key.loc, name = key.name);
                return Err(())
            }
        }
    }
    Ok(program)
}

fn parse_tape<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Symbol<'nsa>>> {
    let mut symbols = vec![];
    while lexer.peek_symbol().is_some() {
        symbols.push(lexer.parse_symbol()?);
    }
    Ok(symbols)
}

fn usage(program_name: &str) {
    eprintln!("Usage: {program_name} <input.tula> <input.tape>");
}

fn start() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().expect("Program name is alway present");

    let tula_path;
    if let Some(path) = args.next() {
        tula_path = path;
    } else {
        usage(&program_name);
        eprintln!("ERROR: no input.tula is provided");
        return Err(());
    }
    let tula_source = fs::read_to_string(&tula_path).map_err(|err| {
        eprintln!("ERROR: could not read file {tula_path}: {err}");
    })?;
    let program = parse_program(&mut Lexer::new(&tula_source, &tula_path))?;
    let state = if let Some(state) = program.entry_state()? {
        state
    } else {
        eprintln!("ERROR: The tule file must have at least one case");
        return Err(());
    };

    let tape_path;
    if let Some(path) = args.next() {
        tape_path = path;
    } else {
        usage(&program_name);
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
        machine.next(&program)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
