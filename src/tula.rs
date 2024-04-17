mod lexer;
mod sexpr;

use std::fs;
use std::result;
use std::fmt::{self, Write};
use std::env;
use std::process::ExitCode;
use std::collections::HashMap;

use lexer::*;
use sexpr::*;

type Result<T> = result::Result<T, ()>;

#[derive(Debug)]
struct Case<'nsa> {
    state: Sexpr<'nsa>,
    read: Sexpr<'nsa>,
    write: Sexpr<'nsa>,
    step: Sexpr<'nsa>,
    next: Sexpr<'nsa>,
}

#[derive(Debug)]
enum Statement<'nsa> {
    Case(Case<'nsa>),
    For {
        // TODO: Support Sexprs for `var` and `set` in for-loops
        var: Symbol<'nsa>,
        set: Symbol<'nsa>,
        body: Box<Statement<'nsa>>,
    }
}

impl<'nsa> fmt::Display for Statement<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Case(Case{state, read, write, step, next}) => {
                write!(f, "case {state} {read} {write} {step} {next}")
            }
            Self::For{var, set, body} => {
                write!(f, "for {var} in {set} {body}", var = var.name, set = set.name)
            }
        }
    }
}

impl<'nsa> Statement<'nsa> {
    fn entry_state(&self, program: &Program<'nsa>) -> Result<Option<Sexpr<'nsa>>> {
        match self {
            Statement::Case(case) => Ok(Some(case.state.clone())),
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
            Statement::Case(Case{state, read, write, step, next}) => {
                let state = state.substitude(var, symbol);
                let read  = read.substitude(var, symbol);
                let write = write.substitude(var, symbol);
                let step  = step.substitude(var, symbol);
                let next  = next.substitude(var, symbol);
                Statement::Case(Case{state, read, write, step, next})
            }
            Statement::For{var: for_var, set: for_set, body} => {
                // TODO: allow subsituting the sets
                let body = Box::new(body.substitude(var, symbol));
                Statement::For{
                    var: *for_var,
                    set: *for_set,
                    body
                }
            }
        }
    }

    fn match_state(&self, program: &Program<'nsa>, state: &Sexpr<'nsa>, read: &Sexpr<'nsa>) -> Result<Option<(Sexpr<'nsa>, Sexpr<'nsa>, Sexpr<'nsa>)>> {
        match self {
            Statement::Case(case) => {
                if case.state.matches(state) && case.read.matches(read) {
                    Ok(Some((case.write.clone(), case.step.clone(), case.next.clone())))
                } else {
                    Ok(None)
                }
            }
            Statement::For{var, set, body} => {
                if let Some(symbols) = program.sets.get(set.name) {
                    for symbol in symbols {
                        let subs_body = body.substitude(*var, *symbol);
                        if let Some(triple) = subs_body.match_state(program, state, read)? {
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
    state: Sexpr<'nsa>,
    tape: Vec<Sexpr<'nsa>>,
    tape_default: Sexpr<'nsa>,
    head: usize,
    halt: bool,
}

impl<'nsa> Machine<'nsa> {
    fn next(&mut self, program: &Program<'nsa>) -> Result<()> {
        for statement in program.statements.iter() {
            if let Some((write, step, next)) = statement.match_state(program, &self.state, &self.tape[self.head])? {
                self.tape[self.head] = write;
                if let Some(step) = step.atom_name() {
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
                        "." => {}
                        "!" => self.print(),
                        _ => {
                            eprintln!("{loc}: ERROR: step is neither -> nor <-", loc = step.loc);
                            return Err(())
                        }
                    }
                } else {
                    eprintln!("{loc}: ERROR: step must be an atom", loc = step.loc());
                    return Err(())
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
        let _ = write!(&mut buffer, "{state}: ", state = self.state);
        let mut head = 0;
        for (i, sexpr) in self.tape.iter().enumerate() {
            if i == self.head {
                head = buffer.len();
            }
            let _ = write!(&mut buffer, "{sexpr} ");
        }
        println!("{buffer}");
        // TODO: use the field width formating magic or something like that
        // for _ in 0..head {
        //     print!(" ");
        // }
        // println!("^");
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
    fn entry_state(&self) -> Result<Option<Sexpr<'nsa>>> {
        for statement in self.statements.iter() {
            if let Some(state) = statement.entry_state(self)? {
                return Ok(Some(state))
            }
        }
        Ok(None)
    }
}

fn parse_case<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Case<'nsa>> {
    let state = Sexpr::parse(lexer)?;
    let read  = Sexpr::parse(lexer)?;
    let write = Sexpr::parse(lexer)?;
    let step  = Sexpr::parse(lexer)?;
    let next  = Sexpr::parse(lexer)?;
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

fn parse_tape<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Sexpr<'nsa>>> {
    let mut tape = vec![];
    while lexer.peek_symbol().is_some() {
        tape.push(Sexpr::parse(lexer)?);
    }
    Ok(tape)
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
        // machine.print();
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
