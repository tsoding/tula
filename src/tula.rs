#[macro_use]
mod lexer;
mod expr;
mod set_expr;

use std::fs;
use std::result;
use std::fmt::{self, Write};
use std::env;
use std::process::ExitCode;
use std::collections::{HashMap};
use unicode_width::UnicodeWidthStr;

use lexer::*;
use expr::*;
use set_expr::*;

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

impl<'nsa> Case<'nsa> {
    fn parse(lexer: &mut Lexer<'nsa>, keyword: Symbol<'nsa>) -> Result<Self> {
        let state = Expr::parse(lexer)?;
        let read  = Expr::parse(lexer)?;
        let write = Expr::parse(lexer)?;
        let step  = Expr::parse(lexer)?;
        let next  = Expr::parse(lexer)?;
        Ok(Case{keyword, state, read, write, step, next})
    }

    fn substitute_bindings(&self, bindings: &HashMap<Symbol<'nsa>, Expr<'nsa>>) -> Self {
        let Case{keyword, state, read, write, step, next} = self;
        let state = state.substitute_bindings(bindings);
        let read  = read.substitute_bindings(bindings);
        let write = write.substitute_bindings(bindings);
        let step  = step.substitute_bindings(bindings);
        let next  = next.substitute_bindings(bindings);
        let keyword = *keyword;
        Case{keyword, state, read, write, step, next}
    }
}

#[derive(Debug, Clone)]
enum Statement<'nsa> {
    Case(Case<'nsa>),
    Block {
        statements: Vec<Statement<'nsa>>
    },
    For {
        var: Symbol<'nsa>,
        set: SetExpr<'nsa>,
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
            Self::Case(Case{keyword, state, read, write, step, next}) => {
                write!(f, "{keyword} {state} {read} {write} {step} {next}")
            }
            Self::For{var, set, body} => {
                write!(f, "for {var} in {set} {body}")
            }
        }
    }
}

type Scope<'nsa> = HashMap<Symbol<'nsa>, SetExpr<'nsa>>;

impl<'nsa> Statement<'nsa> {
    fn parse(lexer: &mut Lexer<'nsa>, sets: &Sets<'nsa>) -> Result<Self> {
        let key = lexer.expect_symbols(&["case", "for", "{"])?;
        match key.name {
            "case" => Ok(Statement::Case(Case::parse(lexer, key)?)),
            "{" => {
                let mut statements = vec![];
                while let Some(symbol) = lexer.peek_symbol() {
                    if symbol.name == "}" {
                        break;
                    }
                    statements.push(Statement::parse(lexer, sets)?);
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
                        Expr::Atom(atom) => {
                            match atom {
                                Atom::Symbol(symbol) => vars.push(symbol),
                                Atom::Integer{..} | Atom::Real{..} | Atom::String{..} => {
                                    eprintln!("{loc}: ERROR: {human} may not be used as variable names", loc = atom.loc(), human = atom.human());
                                    return Err(())
                                }
                            }
                        }
                        Expr::Eval{loc, ..} | Expr::Tuple{loc, ..} => {
                            eprintln!("{loc}: ERROR: Pattern Matching in Universal Quantifiers is not supported");
                            return Err(())
                        }
                    }
                }
                let _ = lexer.expect_symbols(&["in"])?;
                let set = SetExpr::parse(lexer, sets)?;
                let mut result = Statement::parse(lexer, sets)?;
                for var in vars.iter().rev() {
                    result = Statement::For{
                        var: *var,
                        set: set.clone(),
                        body: Box::new(result)
                    }
                }
                Ok(result)
            }
            _ => unreachable!()
        }
    }

    fn match_next_case_scoped(&self, scope: &mut Scope<'nsa>, sets: &Sets<'nsa>, state: &Expr<'nsa>, read: &Expr<'nsa>) -> Result<Option<(Expr<'nsa>, Expr<'nsa>, Expr<'nsa>)>> {
        match self {
            Statement::Case(case) => {
                let mut bindings = HashMap::new();

                if !case.state.clone().force_evals()?.pattern_match(state, scope, &mut bindings) {
                    return Ok(None)
                }
                if !case.read.clone().force_evals()?.pattern_match(read, scope, &mut bindings) {
                    return Ok(None)
                }

                for (var, set) in scope.iter() {
                    if let Some(value) = bindings.get(var) {
                        if !set.contains(sets, value) {
                            return Ok(None)
                        }
                    } else {
                        unreachable!("Sanity check was not performed");
                    }
                }

                Ok(Some((
                    case.write.substitute_bindings(&bindings).force_evals()?,
                    case.step.substitute_bindings(&bindings).force_evals()?,
                    case.next.substitute_bindings(&bindings).force_evals()?
                )))
            }
            Statement::Block{statements} => {
                for statement in statements {
                    if let Some(result) = statement.match_next_case_scoped(scope, sets, state, read)? {
                        return Ok(Some(result))
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
                scope.insert(*var, set.clone());
                let result = body.match_next_case_scoped(scope, sets, state, read)?;
                scope.remove(var);
                Ok(result)
            }
        }
    }

    fn match_next_case(&self, sets: &Sets<'nsa>, state: &Expr<'nsa>, read: &Expr<'nsa>) -> Result<Option<(Expr<'nsa>, Expr<'nsa>, Expr<'nsa>)>>{
        let mut scope = Scope::new();
        self.match_next_case_scoped(&mut scope, sets, state, read)
    }

    fn expand_bound(&self, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>, sets: &Sets<'nsa>, normalize: bool) -> Result<()> {
        match self {
            Statement::Case(case) => {
                let Case{keyword, state, read, write, step, next} = case.substitute_bindings(bindings).clone();
                let write = write.clone().force_evals()?;
                let step = step.clone().force_evals()?;
                let next = next.clone().force_evals()?;
                if normalize {
                    let state = NormExpr(&state);
                    let read = NormExpr(&read);
                    let write = NormExpr(&write);
                    let step = NormExpr(&step);
                    let next = NormExpr(&next);
                    println!("{keyword} {state} {read} {write} {step} {next}");
                } else {
                    println!("{keyword} {state} {read} {write} {step} {next}");
                }
            }
            Statement::For{var, set, body} => {
                if let Some((shadowed_var, _)) = bindings.get_key_value(var) {
                    println!("{loc}: ERROR: {var} shadows another name in the higher scope", loc = var.loc);
                    println!("{loc}: NOTE: the shadowed name is located here", loc = shadowed_var.loc);
                    return Err(())
                }
                for element in set.expand(sets)?.iter() {
                    bindings.insert(*var, element.clone());
                    body.expand_bound(bindings, sets, normalize)?;
                    bindings.remove(var);
                }
            }
            Statement::Block{statements} => {
                for statement in statements {
                    statement.expand_bound(bindings, sets, normalize)?;
                }
            }
        }
        Ok(())
    }

    fn expand(&self, sets: &Sets<'nsa>, normalize: bool) -> Result<()> {
        let mut bindings = HashMap::new();
        self.expand_bound(&mut bindings, sets, normalize)
    }

    fn sanity_check_scoped(&self, scope: &mut Scope<'nsa>) -> Result<()> {
        match self {
            Statement::Case(case) => {
                let mut unused_vars = vec![];
                for (var, _) in scope.iter() {
                    if case.state.uses_var(var).or_else(|| case.read.uses_var(var)).is_none() {
                        unused_vars.push(var);
                    }
                }
                if !unused_vars.is_empty() {
                    eprintln!("{loc}: ERROR: not all variables in the scope are used in the input of the case", loc = case.keyword.loc);
                    unused_vars.sort();
                    for var in unused_vars {
                        eprintln!("{loc}: NOTE: unused variable {var}", loc = var.loc);
                    }
                    return Err(())
                }
            }
            Statement::Block{statements} => {
                for statement in statements {
                    statement.sanity_check_scoped(scope)?
                }
            }
            Statement::For{var, set, body} => {
                if let Some((shadowed_var, _)) = scope.get_key_value(var) {
                    println!("{loc}: ERROR: {var} shadows another name in the higher scope", loc = var.loc);
                    println!("{loc}: NOTE: the shadowed name is located here", loc = shadowed_var.loc);
                    return Err(())
                }
                scope.insert(*var, set.clone());
                body.sanity_check_scoped(scope)?;
                scope.remove(var);
            }
        }
        Ok(())
    }

    fn sanity_check(&self) -> Result<()> {
        let mut scope = Scope::new();
        self.sanity_check_scoped(&mut scope)
    }
}

#[derive(Debug)]
struct Machine<'nsa> {
    state: Expr<'nsa>,
    tape_right: Vec<Expr<'nsa>>,
    tape_left: Vec<Expr<'nsa>>,
    tape_right_default: Expr<'nsa>,
    tape_left_default: Expr<'nsa>,
    head: i32,
    halt: bool,
}

impl<'nsa> Machine<'nsa> {
    fn head_cell(&mut self) -> &Expr<'nsa> {
        if self.head >= 0 {
            let index = self.head as usize;
            while index >= self.tape_right.len() {
                self.tape_right.push(self.tape_right_default.clone());
            }
            &self.tape_right[index]
        } else {
            let index = (self.head.abs() - 1) as usize;
            while index >= self.tape_left.len() {
                self.tape_left.push(self.tape_left_default.clone());
            }
            &self.tape_left[index]
        }
    }

    fn head_cell_mut(&mut self) -> &mut Expr<'nsa> {
        if self.head >= 0 {
            let index = self.head as usize;
            while index > self.tape_right.len() {
                self.tape_right.push(self.tape_right_default.clone());
            }
            &mut self.tape_right[index]
        } else {
            let index = (self.head.abs() - 1) as usize;
            while index > self.tape_left.len() {
                self.tape_left.push(self.tape_left_default.clone());
            }
            &mut self.tape_left[index]
        }
    }

    fn next(&mut self, statements: &[Statement<'nsa>], sets: &Sets<'nsa>) -> Result<()> {
        for statement in statements {
            let head_cell = self.head_cell().clone(); // TODO: it would be better to avoid this clone
            if let Some((write, step, next)) = statement.match_next_case(sets, &self.state, &head_cell)? {
                *self.head_cell_mut() = write.force_evals()?;
                let step = step.expect_atom()?.expect_symbol()?;
                match step.name {
                    "<-" => {
                        self.head -= 1;
                        self.head_cell(); // NOTE: touching the head so it creates the cell
                    }
                    "->" => {
                        self.head += 1;
                        self.head_cell(); // NOTE: touching the head so it creates the cell
                    }
                    "." => {}
                    "!" => self.print(),
                    _ => {
                        eprintln!("{loc}: ERROR: unknown step action {step}", loc = step.loc);
                        return Err(())
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
        for expr in self.tape_left.iter().rev() {
            print!("{expr} ");
        }
        for expr in self.tape_right.iter() {
            print!("{expr} ");
        }
        println!()
    }

    fn trace(&self) {
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{state}: ", state = self.state);
        let mut head_begin = 0;
        let mut head_end = 0;
        let mut iter = self.tape_left
            .iter()
            .enumerate()
            .map(|(i, x)| (-(i as i32 + 1), x))
            .rev()
            .chain(self.tape_right.iter().enumerate().map(|(i, x)| (i as i32, x)));
        if let Some((i, expr)) = iter.next() {
            if i == self.head {
                head_begin = buffer.len();
            }
            let _ = write!(&mut buffer, "{expr}");
            if i == self.head {
                head_end = buffer.len();
            }
        }
        for (i, expr) in iter {
            let _ = write!(&mut buffer, " ");
            if i == self.head {
                head_begin = buffer.len();
            }
            let _ = write!(&mut buffer, "{expr}");
            if i == self.head {
                head_end = buffer.len();
            }
        }
        //                     head_end
        //                     v
        // "State: aaa bbb cccc dddd"
        //                 ^
        //                 head_begin
        println!("{buffer}");
        print!("{pad:width$}", pad = "", width = UnicodeWidthStr::width(&buffer[0..head_begin]));
        println!("{x:~<width$}", x = "^", width = UnicodeWidthStr::width(&buffer[head_begin..head_end]));
    }
}

#[derive(PartialEq)]
enum RunKind {
    Run,
    Trace,
}

impl RunKind {
    fn from_name(name: &str) -> Option<RunKind> {
        match name {
            "run" => Some(RunKind::Run),
            "trace" => Some(RunKind::Trace),
            _ => None
        }
    }
}

impl fmt::Display for RunKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunKind::Run => write!(f, "run"),
            RunKind::Trace => write!(f, "trace"),
        }
    }
}

struct Run<'nsa> {
    kind: RunKind,
    keyword: Symbol<'nsa>,
    state: Expr<'nsa>,
    open_curly_of_tape: Symbol<'nsa>,
    tape: Vec<Expr<'nsa>>,
}

impl<'nsa> Run<'nsa> {
    fn parse(lexer: &mut Lexer<'nsa>) -> Result<Self> {
        let keyword = lexer.expect_symbols(&["run", "trace"])?;
        let state = Expr::parse(lexer)?.force_evals()?;
        let (open_curly_of_tape, tape) = Self::parse_tape(lexer)?;
        let kind = RunKind::from_name(keyword.name).unwrap();
        Ok(Run {keyword, state, open_curly_of_tape, tape, kind})
    }

    fn parse_tape(lexer: &mut Lexer<'nsa>) -> Result<(Symbol<'nsa>, Vec<Expr<'nsa>>)> {
        let open_curly = lexer.expect_symbols(&["{"])?;
        let mut seq = vec![];
        while let Some(symbol) = lexer.peek_symbol() {
            if symbol.name == "}" {
                break;
            }
            seq.push(Expr::parse(lexer)?.force_evals()?);
        }
        let _ = lexer.expect_symbols(&["}"])?;
        Ok((open_curly, seq))
    }

    fn expand(&self, normalize: bool) {
        print!("{kind}", kind = self.kind);
        print!(" {entry}", entry = self.state);
        print!(" {{");
        for (i, expr) in self.tape.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            if normalize {
                print!("{expr}", expr = NormExpr(expr));
            } else {
                print!("{expr}");
            }
        }
        println!("}}");
    }
}

fn parse_program<'nsa>(lexer: &mut Lexer<'nsa>) -> Result<(Sets<'nsa>, Vec<Statement<'nsa>>, Vec<Run<'nsa>>)> {
    let mut statements = vec![];
    let mut sets = Sets::new();
    let mut runs = vec![];
    while let Some(key) = lexer.peek_symbol() {
        match key.name {
            "run" | "trace" => {
                runs.push(Run::parse(lexer)?);
            }
            "case" | "for" => {
                statements.push(Statement::parse(lexer, &sets)?);
            }
            "let" => {
                lexer.next_symbol();
                let atom = Atom::from_symbol(lexer.parse_symbol()?)?;
                let name = match atom {
                    Atom::Symbol(name) => name,
                    Atom::Integer{..} | Atom::Real{..} | Atom::String{..} => {
                        eprintln!("{loc}: ERROR: set name may not be {human}", loc = atom.loc(), human = atom.human());
                        return Err(())
                    }
                };
                // TODO: improve extendability of this piece of code.
                //   If I add more magical sets, it's easy to forget to update this match.
                match name.name {
                    "Integer" | "Real" | "String" => {
                        eprintln!("{loc}: ERROR: redefinition of a magical set {name}", loc = name.loc);
                        return Err(());
                    }
                    _ => {}
                }
                if let Some((orig_name, _)) = sets.get_key_value(&name) {
                    eprintln!("{loc}: ERROR: redefinition of set {name}", loc = name.loc);
                    eprintln!("{loc}: NOTE: first definition located here", loc = orig_name.loc);
                    return Err(())
                }
                sets.insert(name, SetExpr::parse(lexer, &sets)?);
            }
            _ => {
                eprintln!("{loc}: ERROR: unknown keyword {name}", loc = key.loc, name = key.name);
                return Err(())
            }
        }
    }
    Ok((sets, statements, runs))
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
    eprintln!("Usage: {program_name} {command_name} {command_signature}");
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
            let (sets, statements, runs) = parse_program(&mut Lexer::new(&tula_source, &tula_path))?;

            for statement in &statements {
                statement.sanity_check()?
            }

            for run in &runs {
                println!("{loc}: {kind}", loc = run.keyword.loc, kind = run.kind);

                let tape_right_default;
                if let Some(symbol) = run.tape.last().cloned() {
                    tape_right_default = symbol;
                } else {
                    eprintln!("{loc}: ERROR: The tape may not be empty. It must contain at least one symbol so we know what to fill it with", loc = run.open_curly_of_tape.loc);
                    return Err(());
                }
                let tape_left_default;
                if let Some(symbol) = run.tape.first().cloned() {
                    tape_left_default = symbol;
                } else {
                    eprintln!("{loc}: ERROR: The tape may not be empty. It must contain at least one symbol so we know what to fill it with", loc = run.open_curly_of_tape.loc);
                    return Err(());
                }
                let mut machine = Machine {
                    state: run.state.clone(),
                    tape_right: run.tape.clone(),
                    tape_left: vec![],
                    tape_right_default,
                    tape_left_default,
                    head: 0,
                    halt: false,
                };

                while !machine.halt {
                    if run.kind == RunKind::Trace {
                        machine.trace();
                    }
                    machine.halt = true;
                    machine.next(&statements, &sets)?;
                }
            }

            Ok(())
        }
    },
    Command {
        name: "expand",
        description: "Expands all the Universal Quantifiers hardcoding all of the cases",
        signature: "[--no-expr] <input.tula>",
        run: |command, program_name: &str, args: env::Args| {
            let mut source_path = None;
            let mut no_expr = false;

            for arg in args {
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

            let (sets, statements, runs) = parse_program(&mut Lexer::new(&source, &source_path))?;

            for statement in &statements {
                statement.sanity_check()?;
            }
            for statement in statements.iter() {
                statement.expand(&sets, no_expr)?;
            }
            for run in &runs {
                run.expand(no_expr);
            }
            Ok(())
        },
    },
    Command {
        name: "lex",
        description: "Lex the given file to see how the Lexer behaves",
        signature: "<input.tula>",
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
        name: "help",
        description: "Prints help about commands",
        signature: "[command]",
        run: |_command, program_name: &str, mut args: env::Args| {
            if let Some(command_name) = args.next() {
                if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
                    command_usage(program_name, command)
                } else {
                    program_usage(program_name);
                    eprintln!("ERROR: unknown command {command_name}");
                    return Err(())
                }
            } else {
                program_usage(program_name)
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
        Err(())
    }
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
