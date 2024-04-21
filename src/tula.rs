mod lexer;
mod expr;

use std::fs;
use std::result;
use std::fmt::{self, Write};
use std::env;
use std::process::ExitCode;
use std::collections::HashMap;

use lexer::*;
use expr::*;

type Result<T> = result::Result<T, ()>;

#[derive(Debug, Clone)]
struct Case<'nsa> {
    state: Expr<'nsa>,
    read: Expr<'nsa>,
    write: Expr<'nsa>,
    step: Expr<'nsa>,
    next: Expr<'nsa>,
}

#[derive(Debug, Clone)]
enum Statement<'nsa> {
    Case(Case<'nsa>),
    Block {
        statements: Vec<Statement<'nsa>>
    },
    For {
        // TODO: Support Exprs for `var` and `set` in for-loops
        var: Symbol<'nsa>,
        set: Symbol<'nsa>,
        body: Box<Statement<'nsa>>,
    }
}

impl<'nsa> fmt::Display for Statement<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block{statements} => {
                write!(f, "{{")?;
                for (i, statement) in statements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{statement}")?;
                }
                write!(f, "}}")
            }
            Self::Case(Case{state, read, write, step, next}) => {
                write!(f, "case {state} {read} {write} {step} {next}")
            }
            Self::For{var, set, body} => {
                write!(f, "for {var} in {set} {body}", set = set.name)
            }
        }
    }
}

type Scope<'nsa> = HashMap<Symbol<'nsa>, Symbol<'nsa>>;

fn set_contains_value(program: &Program<'_>, set: &Symbol<'_>, value: &Expr<'_>) -> Result<bool> {
    if let Some(set_values) = program.sets.get(set) {
        Ok(set_values.contains(value))
    } else {
        match set.name {
            "Integer" => {
                match value {
                    Expr::Atom{name: symbol} => {
                        match symbol.name.parse::<i32>() {
                            Ok(_) => Ok(true),
                            Err(_) => Ok(false),
                        }
                    }
                    Expr::List{..} => Ok(false),
                }
            }
            _ => {
                eprintln!("{loc}: ERROR: unknown set {set}", loc = set.loc);
                Err(())
            }
        }
    }
}

impl<'nsa> Statement<'nsa> {
    fn type_check_case(&self, program: &Program<'nsa>, state: &Expr<'nsa>, read: &Expr<'nsa>, scope: &mut Scope<'nsa>) -> Result<Option<(Expr<'nsa>, Expr<'nsa>, Expr<'nsa>)>> {
        match self {
            Statement::Case(case) => {
                let mut bindings = HashMap::new();
                if !case.state.pattern_match(state, Some(scope), &mut bindings) {
                    return Ok(None)
                }
                if !case.read.pattern_match(read, Some(scope), &mut bindings) {
                    return Ok(None)
                }

                let mut write = case.write.clone();
                let mut step  = case.step.clone();
                let mut next  = case.next.clone();
                for (var, set) in scope.iter() {
                    if let Some(value) = bindings.get(var) {
                        if set_contains_value(program, set, value)? {
                            write = write.substitute(*var, value.clone());
                            step = step.substitute(*var, value.clone());
                            next = next.substitute(*var, value.clone());
                        } else {
                            return Ok(None)
                        }
                    } else {
                        if let Some(symbol) = case.write.find_symbol(var)
                            .or_else(|| case.step.find_symbol(var))
                            .or_else(|| case.next.find_symbol(var))
                        {
                            eprintln!("{loc}: ERROR: ambiguous use of variable {var}", loc = symbol.loc);
                            eprintln!("{loc}: NOTE: to make it unambiguous it must be use here", loc = case.state.loc());
                            eprintln!("{loc}: NOTE: or here", loc = case.read.loc());
                            return Err(())
                        } else {
                            eprintln!("{loc}: ERROR: unused variable {var}", loc = var.loc);
                            return Err(())
                        }
                    }
                }
                Ok(Some((write, step, next)))
            }
            Statement::Block{statements} => {
                for statement in statements {
                    if let Some(triple) = statement.type_check_case(program, state, read, scope)? {
                        return Ok(Some(triple))
                    }
                }
                Ok(None)
            }
            Statement::For{var, set, body} => {
                if let Some((shadowed_var, _)) = scope.get_key_value(var) {
                    println!("{loc}: ERROR: {var} shadows another name in the higher scope", loc = var.loc);
                    println!("{loc}: NOTE: the shadowed name is located here", loc = shadowed_var.loc);
                    return Err(())
                }
                scope.insert(*var, *set);
                let result = body.type_check_case(program, state, read, scope);
                scope.remove(var);
                result
            }
        }
    }

    fn expand(&self, program: &Program) -> Result<()> {
        match self {
            Statement::Case(_) => {
                println!("{self}");
                Ok(())
            },
            Statement::For{var, set, body} => {
                if let Some(sexprs) = program.sets.get(set) {
                    for sexpr in sexprs {
                        body.substitute(*var, sexpr.clone()).expand(program)?;
                    }
                    Ok(())
                } else {
                    match set.name {
                        "Integer" => {
                            eprintln!("{loc}: ERROR: cannot expand infinite set {set}", loc = set.loc);
                            Err(())
                        }
                        _ => {
                            eprintln!("{loc}: ERROR: unknown set {name}", loc = set.loc, name = set.name);
                            Err(())
                        }
                    }
                }
            }
            Statement::Block{statements} => {
                for statement in statements.iter() {
                    statement.expand(program)?;
                }
                Ok(())
            }
        }
    }

    fn substitute(&self, var: Symbol<'nsa>, sexpr: Expr<'nsa>) -> Statement<'nsa> {
        match self {
            Statement::Block{statements} => {
                Statement::Block {
                    statements: statements.iter().map(|s| s.substitute(var, sexpr.clone())).collect()
                }
            }
            Statement::Case(Case{state, read, write, step, next}) => {
                let state = state.substitute(var, sexpr.clone());
                let read  = read.substitute(var, sexpr.clone());
                let write = write.substitute(var, sexpr.clone());
                let step  = step.substitute(var, sexpr.clone());
                let next  = next.substitute(var, sexpr.clone());
                Statement::Case(Case{state, read, write, step, next})
            }
            Statement::For{var: for_var, set: for_set, body} => {
                // TODO: allow subsituting the sets
                let body = Box::new(body.substitute(var, sexpr));
                Statement::For{
                    var: for_var.clone(),
                    set: *for_set,
                    body
                }
            }
        }
    }
}

#[derive(Debug)]
struct Machine<'nsa> {
    state: Expr<'nsa>,
    tape: Vec<Expr<'nsa>>,
    tape_default: Expr<'nsa>,
    head: usize,
    halt: bool,
}

impl<'nsa> Machine<'nsa> {
    fn next(&mut self, program: &Program<'nsa>) -> Result<()> {
        for statement in program.statements.iter() {
            // if let Some((write, step, next)) = statement.match_state(program, &self.state, &self.tape[self.head])? {
            let mut scope = Scope::new();
            if let Some((write, step, next)) = statement.type_check_case(program, &self.state, &self.tape[self.head], &mut scope)? {
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
        let mut head_begin = 0;
        let mut head_end = 0;
        for (i, sexpr) in self.tape.iter().enumerate() {
            if i > 0 {
                let _ = write!(&mut buffer, " ");
            }
            if i == self.head {
                head_begin = buffer.len();
            }
            let _ = write!(&mut buffer, "{sexpr}");
            if i == self.head {
                head_end = buffer.len();
            }
        }
        println!("{buffer}");
        // TODO: use the field width formating magic or something like that
        for _ in 0..head_begin {
            print!(" ");
        }
        print!("^");
        for _ in head_begin+1..head_end {
            print!("~");
        }
        println!();
    }
}

fn parse_seq_of_sexprs<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<(Symbol<'nsa>, Vec<Expr<'nsa>>)> {
    let open_curly = lexer.expect_symbols(&["{"])?;
    let mut seq = vec![];
    while let Some(symbol) = lexer.peek_symbol() {
        if symbol.name == "}" {
            break;
        }
        seq.push(Expr::parse(lexer)?);
    }
    let _ = lexer.expect_symbols(&["}"])?;
    Ok((open_curly, seq))
}

struct Run<'nsa> {
    keyword: Symbol<'nsa>,
    state: Expr<'nsa>,
    open_curly_of_tape: Symbol<'nsa>,
    tape: Vec<Expr<'nsa>>,
    trace: bool,
}

#[derive(Default)]
struct Program<'nsa> {
    statements: Vec<Statement<'nsa>>,
    sets: HashMap<Symbol<'nsa>, Vec<Expr<'nsa>>>,
    runs: Vec<Run<'nsa>>,
}

fn parse_case<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Case<'nsa>> {
    let state = Expr::parse(lexer)?;
    let read  = Expr::parse(lexer)?;
    let write = Expr::parse(lexer)?;
    let step  = Expr::parse(lexer)?;
    let next  = Expr::parse(lexer)?;
    Ok(Case{state, read, write, step, next})
}

fn parse_statement<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Statement<'nsa>> {
    let key = lexer.expect_symbols(&["case", "for", "{"])?;
    match key.name {
        "case" => Ok(Statement::Case(parse_case(lexer)?)),
        "{" => {
            let mut statements = vec![];
            while let Some(symbol) = lexer.peek_symbol() {
                if symbol.name == "}" {
                    break;
                }
                statements.push(parse_statement(lexer)?);
            }
            let _ = lexer.expect_symbols(&["}"])?;
            Ok(Statement::Block{statements})
        }
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
                    var: var.clone(),
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
    let state = Expr::parse(lexer)?;
    let (open_curly_of_tape, tape) = parse_seq_of_sexprs(lexer)?;
    let trace = keyword.name == "trace";
    Ok(Run {keyword, state, open_curly_of_tape, tape, trace})
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
                let name = lexer.parse_symbol()?;
                if program.sets.contains_key(&name) {
                    eprintln!("{loc}: ERROR: redefinition of set {name}", loc = name.loc);
                    return Err(())
                }
                let (_open_curly, set) = parse_seq_of_sexprs(lexer)?;
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
        signature: "<input.tula>",
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
                    eprintln!("{loc}: ERROR: The tape may not be empty. It must contain at least one symbol so we know what to fill it with to the right indefinitely", loc = run.open_curly_of_tape.loc);
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
                let sexpr = Expr::parse(&mut lexer)?;
                println!("{loc}: {sexpr}", loc = sexpr.loc());
            }

            Ok(())
        }
    },
    Command {
        name: "expand",
        description: "Expands all the Universal Quantifiers hardcoding all of the cases",
        signature: "<input.tula>",
        run: |command, program_name: &str, mut args: env::Args| {
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

            let program = parse_program(&mut Lexer::new(&source, &source_path))?;

            for statement in &program.statements {
                statement.expand(&program)?;
            }
            for run in &program.runs {
                if run.trace {
                    print!("trace");
                } else {
                    print!("run");
                }
                print!(" {entry}", entry = run.state);
                print!(" {{");
                for (i, sexpr) in run.tape.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{sexpr}");
                }
                print!("}}");
            }
            println!();
            Ok(())
        },
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
