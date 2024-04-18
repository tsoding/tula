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
    fn substitude(&self, var: Symbol<'nsa>, sexpr: Sexpr<'nsa>) -> Statement<'nsa> {
        match self {
            Statement::Case(Case{state, read, write, step, next}) => {
                let state = state.substitude(var, sexpr.clone());
                let read  = read.substitude(var, sexpr.clone());
                let write = write.substitude(var, sexpr.clone());
                let step  = step.substitude(var, sexpr.clone());
                let next  = next.substitude(var, sexpr.clone());
                Statement::Case(Case{state, read, write, step, next})
            }
            Statement::For{var: for_var, set: for_set, body} => {
                // TODO: allow subsituting the sets
                let body = Box::new(body.substitude(var, sexpr));
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
                if let Some(sexprs) = program.sets.get(set.name) {
                    for sexpr in sexprs {
                        let subs_body = body.substitude(*var, sexpr.clone());
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
        for sexpr in &self.tape {
            let _ = print!("{sexpr} ");
        }
        println!()
    }

    fn trace(&self) {
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
        for _ in 0..head {
            print!(" ");
        }
        println!("^");
    }
}

fn parse_seq_of_sexprs<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Vec<Sexpr<'nsa>>> {
    let _ = lexer.expect_symbols(&["{"])?;
    let mut seq = vec![];
    while let Some(symbol) = lexer.peek_symbol() {
        if symbol.name == "}" {
            break;
        }
        seq.push(Sexpr::parse(lexer)?);
    }
    let _ = lexer.expect_symbols(&["}"])?;
    Ok(seq)
}

struct Run<'nsa> {
    keyword: Symbol<'nsa>,
    state: Sexpr<'nsa>,
    tape: Vec<Sexpr<'nsa>>,
    trace: bool,
}

#[derive(Default)]
struct Program<'nsa> {
    statements: Vec<Statement<'nsa>>,
    sets: HashMap<&'nsa str, Vec<Sexpr<'nsa>>>,
    runs: Vec<Run<'nsa>>,
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
            let mut vars = vec![];
            while let Some(symbol) = lexer.peek_symbol() {
                if symbol.name == "in" {
                    break;
                }
                vars.push(lexer.parse_symbol()?);
            }
            let _ = lexer.expect_symbols(&["in"])?;
            let set = lexer.parse_symbol()?;
            let mut result = parse_statement(lexer)?;
            for var in vars.iter().rev() {
                result = Statement::For{
                    var: *var,
                    set,
                    body: Box::new(result)
                }
            }
            Ok(result)
        }
        _ => unreachable!()
    }
}

fn parse_run<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Run<'nsa>> {
    let keyword = lexer.expect_symbols(&["run", "trace"])?;
    let state = Sexpr::parse(lexer)?;
    let tape = parse_seq_of_sexprs(lexer)?;
    let trace = keyword.name == "trace";
    Ok(Run {keyword, state, tape, trace})
}

fn parse_program<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Program<'nsa>> {
    let mut program = Program::default();
    while let Some(key) = lexer.peek_symbol() {
        match key.name {
            "run" | "trace" => {
                program.runs.push(parse_run(lexer)?);
            }
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
                let set = parse_seq_of_sexprs(lexer)?;
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

fn program_usage(program_name: &str) {
    eprintln!("Usage: {program_name} <command> [ARGUMENTS]");
    eprintln!("Commands:");
    for command in COMMANDS.iter() {
        let command_name = &command.name;
        let command_signature = &command.signature;
        let command_description = &command.description;
        eprintln!("    {command_name} {command_signature} - {command_description}");
    }
}

fn command_usage(program_name: &str, command: &Command) {
    let command_name = &command.name;
    let command_signature = &command.signature;
    eprintln!("Usage: {program_name} {command_name} {command_signature}")
}

struct Command {
    name: &'static str,
    description: &'static str,
    signature: &'static str,
    run: fn (command: &Command, program_name: &str, args: env::Args) -> Result<()>,
}

const COMMANDS: &[Command] = &[
    Command {
        name: "run",
        description: "Run the Tula Program",
        signature: "<input.tula> <input.tape>",
        run: |command, program_name, mut args| {
            let tula_path;
            if let Some(path) = args.next() {
                tula_path = path;
            } else {
                command_usage(program_name, command);
                eprintln!("ERROR: no input.tula is provided");
                return Err(());
            }
            let tula_source = fs::read_to_string(&tula_path).map_err(|err| {
                eprintln!("ERROR: could not read file {tula_path}: {err}");
            })?;
            let program = parse_program(&mut Lexer::new(&tula_source, &tula_path))?;

            for run in &program.runs {
                println!("{loc}: run", loc = run.keyword.loc);

                let tape_default;
                if let Some(symbol) = run.tape.last().cloned() {
                    tape_default = symbol;
                } else {
                    eprintln!("ERROR: The tape file may not be empty. I must contain at least one symbol so we know what to fill the infinite tape with");
                    return Err(());
                }
                let mut machine = Machine {
                    state: run.state.clone(),
                    tape: run.tape.clone(),
                    tape_default,
                    head: 0,
                    halt: false,
                };

                while !machine.halt {
                    if run.trace {
                        machine.trace();
                    }
                    machine.halt = true;
                    machine.next(&program)?;
                }
            }

            Ok(())
        }
    },
    Command {
        name: "lex",
        description: "Lex the given file to see how the Lexer behaves",
        signature: "<input-file>",
        run: |command, program_name, mut args| {
            let input_path;
            if let Some(arg) = args.next() {
                input_path = arg;
            } else {
                command_usage(program_name, command);
                return Err(());
            }

            let input_source = fs::read_to_string(&input_path).map_err(|err| {
                eprintln!("ERROR: could not read file {input_path}: {err}");
            })?;

            for Symbol{loc, name} in Lexer::new(&input_source, &input_path) {
                println!("{loc}: {name}");
            }
            Ok(())
        },
    },
    Command {
        name: "sexpr",
        signature: "<input-file>",
        description: "Parse the S-expression from the file to test the behavior of the Parser",
        run: |command, program_name, mut args| {
            let source_path;
            if let Some(arg) = args.next() {
                source_path = arg;
            } else {
                command_usage(program_name, command);
                return Err(());
            }

            let source = fs::read_to_string(&source_path).map_err(|err| {
                eprintln!("ERROR: could not read file {source_path}: {err}");
            })?;

            let mut lexer = Lexer::new(&source, &source_path);
            while lexer.peek_symbol().is_some() {
                let sexpr = Sexpr::parse(&mut lexer)?;
                println!("{loc}: {sexpr}", loc = sexpr.loc());
            }

            Ok(())
        }
    }
];

fn start() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().expect("Program name is alway present");

    let command_name;
    if let Some(arg) = args.next() {
        command_name = arg;
    } else {
        program_usage(&program_name);
        eprintln!("ERROR: no command is provided");
        return Err(())
    }

    if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
        (command.run)(command, &program_name, args)
    } else {
        eprintln!("ERROR: no command with the name {command_name}");
        return Err(())
    }
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
