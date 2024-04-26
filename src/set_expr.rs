use std::collections::{HashSet, HashMap};
use std::fmt;
use super::lexer::{Lexer, Symbol};
use super::expr::{Expr, Atom};
use super::Result;

pub type Sets<'nsa> = HashMap<Symbol<'nsa>, SetExpr<'nsa>>;

#[derive(Debug, Clone)]
pub enum SetExpr<'nsa> {
    Named(Symbol<'nsa>),
    Anonymous {
        elements: HashSet<Expr<'nsa>>
    },
    Integer(Symbol<'nsa>),
    Union {
        lhs: Box<SetExpr<'nsa>>,
        rhs: Box<SetExpr<'nsa>>,
    },
    Diff {
        lhs: Box<SetExpr<'nsa>>,
        rhs: Box<SetExpr<'nsa>>,
    }
}

impl<'nsa> fmt::Display for SetExpr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{name}"),
            Self::Integer(_) => write!(f, "Integer"),
            Self::Anonymous {elements} => {
                write!(f, "{{")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{element}")?;
                }
                write!(f, "}}")
            }
            Self::Union {lhs, rhs} => write!(f, "{lhs} + {rhs}"),
            Self::Diff {lhs, rhs} => write!(f, "{lhs} - {rhs}"),
        }
    }
}

impl<'nsa> SetExpr<'nsa> {
    fn parse_anonymous(lexer: &mut Lexer<'nsa>) -> Result<HashSet<Expr<'nsa>>> {
        let _ = lexer.expect_symbols(&["{"])?;
        let mut set: HashSet<Expr<'nsa>> = HashSet::new();
        while let Some(symbol) = lexer.peek_symbol() {
            if symbol.name == "}" {
                break;
            }
            let value = Expr::parse(lexer)?.force_evals()?;
            if let Some(existing_value) = set.get(&value) {
                eprintln!("{loc}: ERROR: Set may only consist of non-repeating values", loc = value.loc());
                eprintln!("{loc}: NOTE: Same value was provided here", loc = existing_value.loc());
                return Err(());
            }
            set.insert(value);
        }
        let _ = lexer.expect_symbols(&["}"])?;
        Ok(set)
    }

    fn parse_primary(lexer: &mut Lexer<'nsa>, sets: &Sets<'nsa>) -> Result<Self> {
        let Some(symbol) = lexer.peek_symbol() else {
            eprintln!("{loc}: ERROR: expected symbol but reached the end of the input", loc = lexer.loc());
            return Err(())
        };
        let set = match symbol.name {
            "{" => {
                let elements = Self::parse_anonymous(lexer)?;
                Self::Anonymous {elements}
            },
            "(" => {
                let _ = lexer.next_symbol().unwrap();
                let inner = Self::parse(lexer, sets)?;
                lexer.expect_symbols(&[")"])?;
                inner
            }
            _ => {
                let _ = lexer.next_symbol().unwrap();
                match Atom::from_symbol(symbol) {
                    Atom::Integer{symbol: Symbol{loc, ..}, ..} => {
                        eprintln!("{loc}: ERROR: integer is not a set expression");
                        return Err(())
                    }
                    Atom::Symbol(symbol) => if symbol.name == "Integer" {
                        Self::Integer(symbol)
                    } else {
                        if !sets.contains_key(&symbol) {
                            eprintln!("{loc}: ERROR: set {symbol} does not exist", loc = symbol.loc);
                            return Err(());
                        }
                        Self::Named(symbol)
                    }
                }
            }
        };
        Ok(set)
    }

    pub fn parse(lexer: &mut Lexer<'nsa>, sets: &Sets<'nsa>) -> Result<Self> {
        let mut lhs = Self::parse_primary(lexer, sets)?;
        while let Some(symbol) = lexer.peek_symbol() {
            match symbol.name {
                "+" => {
                    let _ = lexer.next_symbol().unwrap();
                    let rhs = SetExpr::parse_primary(lexer, sets)?;
                    lhs = SetExpr::Union {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                "-" => {
                    let _ = lexer.next_symbol().unwrap();
                    let rhs = SetExpr::parse_primary(lexer, sets)?;
                    lhs = SetExpr::Diff {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                _ => break
            }
        }
        Ok(lhs)
    }

    pub fn contains(&self, sets: &Sets<'nsa>, element: &Expr<'nsa>) -> bool {
        match self {
            Self::Union{lhs, rhs} => lhs.contains(sets, element) || rhs.contains(sets, element),
            Self::Diff{lhs, rhs} => lhs.contains(sets, element) && !rhs.contains(sets, element),
            Self::Anonymous{elements} => elements.contains(element),
            Self::Integer(_)=> {
                if let Expr::Atom(Atom::Integer{..}) = element {
                    true
                } else {
                    false
                }
            }
            Self::Named(name) => {
                sets.get(name)
                    .expect("The existence of all Named Set Expressions must be checked upfront")
                    .contains(sets, element)
            }
        }
    }

    pub fn expand(&self, sets: &Sets<'nsa>) -> Result<HashSet<Expr<'nsa>>> {
        match self {
            Self::Union{lhs, rhs} => Ok(lhs.expand(sets)?.union(&rhs.expand(sets)?).cloned().collect()),
            Self::Diff{lhs, rhs} => Ok(lhs.expand(sets)?.difference(&rhs.expand(sets)?).cloned().collect()),
            Self::Anonymous{elements} => Ok(elements.clone()),
            Self::Integer(Symbol{loc, ..})=> {
                eprintln!("{loc}: Impossible to expand set Integer: it's too big");
                Err(())
            }
            Self::Named(name) => {
                sets.get(name)
                    .expect("The existence of all Named Set Expressions must be checked upfront")
                    .expand(sets)
            }
        }
    }
}
