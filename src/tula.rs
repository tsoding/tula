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
    keyword: Symbol<'nsa>,
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
        var: Symbol<'nsa>,
        set: Symbol<'nsa>,
        body: Box<Statement<'nsa>>,
    }
}

struct NormStatement<'nsa, 'cia>(&'cia Statement<'nsa>);

impl<'nsa, 'cia> fmt::Display for NormStatement<'nsa, 'cia> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NormStatement(stmt) = self;
        match stmt {
            Statement::Block{statements} => {
                write!(f, "{{")?;
                for (i, statement) in statements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{statement}", statement = NormStatement(statement))?;
                }
                write!(f, "}}")
            }
            Statement::Case(Case{keyword, state, read, write, step, next}) => {
                let state = NormExpr(state);
                let read = NormExpr(read);
                let write = NormExpr(write);
                let step = NormExpr(step);
                let next = NormExpr(next);
                write!(f, "{keyword} {state} {read} {write} {step} {next}")
            }
            Statement::For{var, set, body} => {
                write!(f, "for {var} in {set} {body}")
            }
        }
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
            Self::Case(Case{keyword, state, read, write, step, next}) => {
                write!(f, "{keyword} {state} {read} {write} {step} {next}")
            }
            Self::For{var, set, body} => {
                write!(f, "for {var} in {set} {body}")
            }
        }
    }
}

type Scope<'nsa> = HashMap<Symbol<'nsa>, Symbol<'nsa>>;
const MAGICAL_SETS: &[&str] = &["Integer"];

fn set_contains_value(program: &Program<'_>, set: &Symbol<'_>, value: &Expr<'_>) -> Result<bool> {
    if let Some(set_values) = program.sets.get(set) {
        Ok(set_values.contains(value))
    } else {
        match set.name {
            "Integer" => {
                match value {
                    Expr::Atom(Atom::Integer{..}) => Ok(true),
                    _ => Ok(false),
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
    fn sanity_check(&self, program: &Program<'nsa>, scope: &mut Scope<'nsa>) -> Result<()> {
        match self {
            Statement::Case(case) => {
                let mut unused_vars = vec![];
                for (var, set) in scope.iter() {
                    if !(program.sets.contains_key(set) || MAGICAL_SETS.contains(&set.name)) {
                        eprintln!("{loc}: ERROR: {set} does not exists", loc = set.loc);
                        return Err(())
                    }

                    if case.state.uses_var(var).or_else(|| case.read.uses_var(var)).is_none() {
                        unused_vars.push(var);
                    }
                }
                if !unused_vars.is_empty() {
                    eprintln!("{loc}: ERROR: not all variables in the scope are used in the input of the case", loc = case.keyword.loc);
                    for var in unused_vars {
                        eprintln!("{loc}: NOTE: unused variable {var}", loc = var.loc);
                    }
                    return Err(())
                }
                Ok(())
            }
            Statement::Block{statements} => {
                for statement in statements {
                    statement.sanity_check(program, scope)?
                }
                Ok(())
            }
            Statement::For{var, set, body} => {
                if let Some((shadowed_var, _)) = scope.get_key_value(var) {
                    println!("{loc}: ERROR: {var} shadows another name in the higher scope", loc = var.loc);
                    println!("{loc}: NOTE: the shadowed name is located here", loc = shadowed_var.loc);
                    return Err(())
                }
                scope.insert(*var, *set);
                let result = body.sanity_check(program, scope);
                scope.remove(var);
                result
            }
        }
    }

    fn type_check_next_case(&self, program: &Program<'nsa>, state: &Expr<'nsa>, read: &Expr<'nsa>, scope: &mut Scope<'nsa>) -> Result<Option<(Expr<'nsa>, Expr<'nsa>, Expr<'nsa>)>> {
        match self {
            Statement::Case(case) => {
                let mut bindings = HashMap::new();

                if !case.state.pattern_match(state, scope, &mut bindings) {
                    return Ok(None)
                }
                if !case.read.pattern_match(read, scope, &mut bindings) {
                    return Ok(None)
                }

                for (var, set) in scope.iter() {
                    if let Some(value) = bindings.get(var) {
                        if !set_contains_value(program, set, value)? {
                            return Ok(None)
                        }
                    } else {
                        unreachable!("Sanity check was not performed");
                    }
                }

                Ok(Some((
                    case.write.substitute_bindings(&bindings),
                    case.step.substitute_bindings(&bindings),
                    case.next.substitute_bindings(&bindings)
                )))
            }
            Statement::Block{statements} => {
                for statement in statements {
                    if let Some(triple) = statement.type_check_next_case(program, state, read, scope)? {
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
                let result = body.type_check_next_case(program, state, read, scope);
                scope.remove(var);
                result
            }
        }
    }

    fn expand(&self, program: &Program, normalize: bool) -> Result<()> {
        match self {
            Statement::Case(_) => {
                if normalize {
                    println!("{}", NormStatement(self));
                } else {
                    println!("{self}");
                }
                Ok(())
            },
            Statement::For{var, set, body} => {
                if let Some(exprs) = program.sets.get(set) {
                    for expr in exprs {
                        body.substitute_var(*var, expr.clone()).expand(program, normalize)?;
                    }
                    Ok(())
                } else {
                    if MAGICAL_SETS.contains(&set.name) {
                        eprintln!("{loc}: ERROR: cannot expand magical set {set} because it's too big", loc = set.loc);
                        Err(())
                    } else {
                        eprintln!("{loc}: ERROR: unknown set {name}", loc = set.loc, name = set.name);
                        Err(())
                    }
                }
            }
            Statement::Block{statements} => {
                for statement in statements.iter() {
                    statement.expand(program, normalize)?;
                }
                Ok(())
            }
        }
    }

    fn substitute_var(&self, var: Symbol<'nsa>, expr: Expr<'nsa>) -> Statement<'nsa> {
        match self {
            Statement::Block{statements} => {
                Statement::Block {
                    statements: statements.iter().map(|s| s.substitute_var(var, expr.clone())).collect()
                }
            }
            Statement::Case(Case{keyword, state, read, write, step, next}) => {
                let mut bindings = HashMap::new();
                bindings.insert(var, expr.clone());
                let state = state.substitute_bindings(&bindings);
                let read  = read.substitute_bindings(&bindings);
                let write = write.substitute_bindings(&bindings);
                let step  = step.substitute_bindings(&bindings);
                let next  = next.substitute_bindings(&bindings);
                let keyword = *keyword;
                Statement::Case(Case{keyword, state, read, write, step, next})
            }
            Statement::For{var: for_var, set: for_set, body} => {
                // TODO: allow subsituting the sets
                let body = Box::new(body.substitute_var(var, expr));
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
            let mut scope = Scope::new();
            if let Some((write, step, next)) = statement.type_check_next_case(program, &self.state, &self.tape[self.head], &mut scope)? {
                if let Expr::Eval{open_paren, lhs, rhs} = write {
                    match *lhs {
                        Expr::Atom(Atom::Integer{value: lhs_value, ..}) => {
                            match *rhs {
                                Expr::Atom(Atom::Integer{value: rhs_value, ..}) => {
                                    self.tape[self.head] = Expr::Atom(Atom::Integer {
                                        symbol: open_paren,
                                        value: lhs_value + rhs_value,
                                    })
                                }
                                _ => {
                                    eprintln!("{loc}: ERROR: right hand side value must be an integer", loc = rhs.loc());
                                    return Err(());
                                }
                            }
                        }
                        _ => {
                            eprintln!("{loc}: ERROR: left hand side value must be an integer", loc = lhs.loc());
                            return Err(());
                        }
                    }
                } else {
                    self.tape[self.head] = write;
                }
                if let Expr::Atom(Atom::Symbol(step)) = step {
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
        for expr in &self.tape {
            let _ = print!("{expr} ");
        }
        println!()
    }

    fn trace(&self) {
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{state}: ", state = self.state);
        let mut head_begin = 0;
        let mut head_end = 0;
        for (i, expr) in self.tape.iter().enumerate() {
            if i > 0 {
                let _ = write!(&mut buffer, " ");
            }
            if i == self.head {
                head_begin = buffer.len();
            }
            let _ = write!(&mut buffer, "{expr}");
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

fn parse_seq_of_exprs<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<(Symbol<'nsa>, Vec<Expr<'nsa>>)> {
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

impl<'nsa> Run<'nsa> {
    fn expand(&self, normalize: bool) {
        if self.trace {
            print!("trace");
        } else {
            print!("run");
        }
        print!(" {entry}", entry = self.state);
        print!(" {{");
        for (i, expr) in self.tape.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            if normalize {
                print!("{expr}", expr = NormExpr(&expr));
            } else {
                print!("{expr}");
            }
        }
        print!("}}");
    }
}

#[derive(Default)]
struct Program<'nsa> {
    statements: Vec<Statement<'nsa>>,
    sets: HashMap<Symbol<'nsa>, Vec<Expr<'nsa>>>,
    runs: Vec<Run<'nsa>>,
}

impl<'nsa> Program<'nsa> {
    fn sanity_check(&self) -> Result<()> {
        for statement in &self.statements {
            let mut scope = Scope::new();
            statement.sanity_check(self, &mut scope)?;
        }
        Ok(())
    }
}

fn parse_case<'nsa>(lexer: &mut Lexer<'nsa>, keyword: Symbol<'nsa>) -> Result<Case<'nsa>> {
    let state = Expr::parse(lexer)?;
    let read  = Expr::parse(lexer)?;
    let write = Expr::parse(lexer)?;
    let step  = Expr::parse(lexer)?;
    let next  = Expr::parse(lexer)?;
    Ok(Case{keyword, state, read, write, step, next})
}

fn parse_statement<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<Statement<'nsa>> {
    let key = lexer.expect_symbols(&["case", "for", "{"])?;
    match key.name {
        "case" => Ok(Statement::Case(parse_case(lexer, key)?)),
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
                let expr = Expr::parse(lexer)?;
                match expr {
                    Expr::Atom(Atom::Symbol(symbol)) => vars.push(symbol),
                    Expr::Atom(Atom::Integer{symbol: Symbol{loc, ..}, ..}) => {
                        eprintln!("{loc}: ERROR: Integers may not be used as variable names");
                        return Err(())
                    }
                    Expr::Eval{open_paren: Symbol{loc, ..}, ..} |
                    Expr::List{open_paren: Symbol{loc, ..}, ..} => {
                        eprintln!("{loc}: ERROR: Pattern Matching in Universal Quantifiers is not supported");
                        return Err(())
                    }
                }
            }
            let _ = lexer.expect_symbols(&["in"])?;
            let set = match Expr::parse(lexer)? {
                Expr::Atom(Atom::Symbol(symbol)) => symbol,
                Expr::Atom(Atom::Integer{symbol: Symbol{loc, ..}, ..}) => {
                    eprintln!("{loc}: ERROR: Integers may not be used as set names");
                    return Err(())
                }
                Expr::Eval{open_paren: Symbol{loc, ..}, ..} |
                Expr::List{open_paren: Symbol{loc, ..}, ..} => {
                    eprintln!("{loc}: ERROR: Set must be just a name");
                    return Err(())
                }
            };
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
    let (open_curly_of_tape, tape) = parse_seq_of_exprs(lexer)?;
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
                let name = match Atom::from_symbol(lexer.parse_symbol()?) {
                    Atom::Symbol(name) => name,
                    Atom::Integer{symbol: Symbol{loc, ..}, ..} => {
                        eprintln!("{loc}: ERROR: set name may not be an integer");
                        return Err(())
                    }
                };
                if let Some((orig_name, _)) = program.sets.get_key_value(&name) {
                    eprintln!("{loc}: ERROR: redefinition of set {name}", loc = name.loc);
                    eprintln!("{loc}: NOTE: first definition located here", loc = orig_name.loc);
                    return Err(())
                }
                let (_open_curly, set) = parse_seq_of_exprs(lexer)?;
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
        name: "debug",
        description: "Just debug some shit",
        signature: "",
        run: |_command, _program_name, _arg| {
            let source = "(1 (2 3) 4)";
            let mut lexer = Lexer::new(&source, file!());
            let expr = Expr::parse(&mut lexer)?;
            println!("{}", NormExpr(&expr));
            Ok(())
        }
    },
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

            program.sanity_check()?;

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
        name: "expr",
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
                let expr = Expr::parse(&mut lexer)?;
                println!("{loc}: {expr}", loc = expr.loc());
            }

            Ok(())
        }
    },
    Command {
        name: "expand",
        description: "Expands all the Universal Quantifiers hardcoding all of the cases",
        signature: "[--no-expr] <input.tula>",
        run: |command, program_name: &str, mut args: env::Args| {
            let mut source_path = None;
            let mut no_expr = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--no-expr" => no_expr = true,
                    _ => {
                        if source_path.is_some() {
                            command_usage(program_name, command);
                            eprintln!("ERROR: interpreting several files is not supported");
                            return Err(())
                        }
                        source_path = Some(arg)
                    }
                }
            }

            let Some(source_path) = source_path else {
                command_usage(program_name, command);
                eprintln!("ERROR: no input is provided");
                return Err(());
            };

            let source = fs::read_to_string(&source_path).map_err(|err| {
                eprintln!("ERROR: could not read file {source_path}: {err}");
            })?;

            let program = parse_program(&mut Lexer::new(&source, &source_path))?;

            program.sanity_check()?;

            for statement in &program.statements {
                statement.expand(&program, no_expr)?;
            }
            for run in &program.runs {
                run.expand(no_expr);
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
