use super::lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::{Result, Scope};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Atom<'nsa> {
    Symbol(Symbol<'nsa>),
    Integer {
        value: i32,
        symbol: Symbol<'nsa>,
    },
}

impl<'nsa> Atom<'nsa> {
    pub fn expect_integer(&self) -> Result<i32> {
        match self {
            &Self::Integer{value, ..} => Ok(value),
            Self::Symbol(symbol) => {
                eprintln!("{loc}: ERROR: expected integer but got symbol `{symbol}`", loc = symbol.loc);
                Err(())
            }
        }
    }

    pub fn expect_symbol(&self) -> Result<&Symbol<'nsa>> {
        match self {
            Self::Integer{symbol: Symbol{loc, ..}, ..} => {
                eprintln!("{loc}: ERROR: expected symbol but got integer");
                Err(())
            }
            Self::Symbol(symbol) => Ok(symbol),
        }
    }

    pub fn from_symbol(symbol: Symbol<'nsa>) -> Self {
        match symbol.name.parse::<i32>() {
            Ok(value) => Atom::Integer{symbol, value},
            // TODO: throw a warning if number is treated as a symbol because of an overflow or other stupid reason
            Err(_) => Atom::Symbol(symbol),
        }
    }
}

impl<'nsa> Hash for Atom<'nsa> {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        match self {
            Self::Symbol(symbol) => symbol.hash(h),
            Self::Integer{value, ..} => value.hash(h),
        }
    }
}

impl<'nsa> PartialEq for Atom<'nsa> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Symbol(symbol) => {
                match other {
                    Self::Symbol(other_symbol) => symbol == other_symbol,
                    Self::Integer{..} => false,
                }
            }
            Self::Integer{value, ..} => {
                match other {
                    Self::Integer{value: other_value, ..} => value == other_value,
                    Self::Symbol(_) => false,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'nsa> {
    Atom(Atom<'nsa>),
    Eval {
        open_paren: Symbol<'nsa>,
        lhs: Box<Expr<'nsa>>,
        op: Box<Expr<'nsa>>,
        rhs: Box<Expr<'nsa>>,
    },
    List {
        open_paren: Symbol<'nsa>,
        items: Vec<Expr<'nsa>>
    },
}

impl<'nsa> PartialEq for Expr<'nsa> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Atom(atom) => {
                match other {
                    Self::Atom(other_atom) => atom == other_atom,
                    Self::Eval{..} | Self::List{..} => false,
                }
            }
            Self::Eval{lhs, rhs, ..} => {
                match other {
                    Self::Eval{lhs: other_lhs, rhs: other_rhs, ..} => lhs == other_lhs && rhs == other_rhs,
                    Self::Atom(_) | Self::List{..} => false,
                }
            }
            Self::List{items, ..} => {
                match other {
                    Self::List{items: other_items, ..} => items == other_items,
                    Self::Atom(_) | Self::Eval{..} => false,
                }
            }
        }
    }
}

impl<'nsa> Eq for Expr<'nsa> {}

impl<'nsa> Hash for Expr<'nsa> {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        match self {
            Self::Atom(atom) => atom.hash(h),
            Self::List{items, ..} => items.hash(h),
            Self::Eval{lhs, rhs, ..} => {
                lhs.hash(h);
                rhs.hash(h);
            }
        }
    }
}

impl<'nsa> fmt::Display for Atom<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(Symbol{name, ..}) => write!(f, "{name}"),
            Atom::Integer{value, ..} => write!(f, "{value}"),
        }
    }
}

impl<'nsa> fmt::Display for Expr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{atom}"),
            Self::Eval{lhs, rhs, ..} => write!(f, "[{lhs} + {rhs}]"),
            Self::List{items, ..} => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{item}")?
                    } else {
                        write!(f, " {item}")?
                    }
                }
                write!(f, ")")
            }
        }
    }
}

pub struct NormExpr<'nsa, 'cia>(pub &'cia Expr<'nsa>);

impl<'nsa, 'cia> fmt::Display for NormExpr<'nsa, 'cia> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NormExpr(expr) = self;
        match expr {
            // TODO: normalize literals wrapped in single quotes
            Expr::Atom(atom) => write!(f, "{atom}"),
            // () => __
            // (1 2 3 4) => _1_2_3_4_
            // (1 (2 3) 4) _1__2_3__4_
            Expr::List{items, ..} => {
                write!(f, "_")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, "_")?;
                    }
                    write!(f, "{}", NormExpr(item))?;
                }
                write!(f, "_")
            }
            Expr::Eval{..} => todo!("I don't know how to normalize these things yet"),
        }
    }
}

fn expect_bool<'nsa>(symbol: &Symbol<'nsa>) -> Result<bool> {
    match symbol.name {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => {
            eprintln!("{loc}: ERROR: expected boolean but got symbol {symbol}", loc = symbol.loc);
            Err(())
        }
    }
}

fn bool_to_str(cond: bool) -> &'static str {
    if cond {
        "true"
    } else {
        "false"
    }
}

impl<'nsa> Expr<'nsa> {
    pub fn expect_atom(&self) -> Result<&Atom<'nsa>> {
        match self {
            Self::Atom(atom) => Ok(atom),
            Self::List{open_paren: Symbol{loc, ..}, ..} => {
                eprintln!("{loc}: ERROR: expected atom but got list");
                Err(())
            }
            Self::Eval{open_paren: Symbol{loc, ..}, ..} => {
                eprintln!("{loc}: ERROR: expected atom but got eval");
                Err(())
            }
        }
    }

    pub fn force_evals(self) -> Result<Expr<'nsa>> {
        match self {
            Self::Atom(_) => Ok(self),
            Self::List{open_paren, items} => {
                let mut new_items = vec![];
                for item in items {
                    new_items.push(item.force_evals()?)
                }
                Ok(Self::List{open_paren, items: new_items})
            }
            Self::Eval{open_paren, lhs, op, rhs} => {
                let lhs = lhs.force_evals()?.expect_atom()?.clone();
                match lhs {
                    Atom::Integer{value: lhs, ..} => {
                        let rhs = rhs.force_evals()?.expect_atom()?.expect_integer()?;
                        let op  = op.force_evals()?.expect_atom()?.expect_symbol()?.clone();
                        match op.name {
                            "+" => Ok(Expr::Atom(Atom::Integer {
                                symbol: open_paren,
                                value: lhs + rhs,
                            })),
                            "-" => Ok(Expr::Atom(Atom::Integer {
                                symbol: open_paren,
                                value: lhs - rhs,
                            })),
                            "*" => Ok(Expr::Atom(Atom::Integer {
                                symbol: open_paren,
                                value: lhs * rhs,
                            })),
                            "/" => Ok(Expr::Atom(Atom::Integer {
                                symbol: open_paren,
                                value: lhs / rhs,
                            })),
                            "%" => Ok(Expr::Atom(Atom::Integer {
                                symbol: open_paren,
                                value: lhs % rhs,
                            })),
                            ">" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs > rhs),
                            }))),
                            ">=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs >= rhs),
                            }))),
                            "<" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs < rhs),
                            }))),
                            "<=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs <= rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs == rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs != rhs),
                            }))),
                            _ => {
                                eprintln!("{loc}: ERROR: Unexpected Integer operation {op}", loc = op.loc);
                                Err(())
                            }
                        }
                    }
                    Atom::Symbol(symbol) => {
                        let lhs = expect_bool(&symbol)?;
                        let rhs = expect_bool(rhs.force_evals()?.expect_atom()?.expect_symbol()?)?;
                        let op  = op.force_evals()?.expect_atom()?.expect_symbol()?.clone();
                        match op.name {
                            "||" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs || rhs),
                            }))),
                            "&&" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs && rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs == rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc: open_paren.loc,
                                name: bool_to_str(lhs != rhs),
                            }))),
                            _ => {
                                eprintln!("{loc}: ERROR: Unexpected Boolean operation", loc = op.loc);
                                Err(())
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn uses_var(&self, var: &Symbol<'nsa>) -> Option<&Symbol<'nsa>> {
        match self {
            Self::Atom(Atom::Symbol(symbol)) => if symbol == var {
                Some(symbol)
            } else {
                None
            },
            Self::Atom(Atom::Integer{..}) => None,
            Self::Eval{lhs, rhs, ..} => {
                lhs.uses_var(var).or_else(|| rhs.uses_var(var))
            }
            Self::List{items, ..} => {
                items.iter().find_map(|item| item.uses_var(var))
            }
        }
    }

    pub fn substitute_bindings(&self, bindings: &HashMap<Symbol<'nsa>, Expr<'nsa>>) -> Expr<'nsa> {
        match self {
            Self::Atom(Atom::Symbol(symbol))  => {
                if let Some(expr) = bindings.get(symbol).cloned() {
                    expr
                } else {
                    self.clone()
                }
            }
            Self::Atom(Atom::Integer{..}) => self.clone(),
            Self::Eval{open_paren, lhs, op, rhs} => {
                let lhs = Box::new(lhs.substitute_bindings(bindings));
                let op  = Box::new(op.substitute_bindings(bindings));
                let rhs = Box::new(rhs.substitute_bindings(bindings));
                Self::Eval{open_paren: *open_paren, lhs, op, rhs}
            }
            Self::List{open_paren, items} => {
                let items = items.iter().map(|item| item.substitute_bindings(bindings)).collect();
                Self::List{open_paren: *open_paren, items}
            }
        }
    }

    pub fn parse(lexer: &mut Lexer<'nsa>) -> Result<Self> {
        let symbol = lexer.parse_symbol()?;
        match symbol.name {
            "[" => {
                let lhs = Box::new(Expr::parse(lexer)?);
                let op  = Box::new(Expr::parse(lexer)?);
                let rhs = Box::new(Expr::parse(lexer)?);
                let _ = lexer.expect_symbols(&["]"])?;
                Ok(Self::Eval {
                    lhs, op, rhs, open_paren: symbol
                })
            }
            "(" => {
                let mut items = vec![];
                while let Some(symbol2) = lexer.peek_symbol() {
                    if symbol2.name == ")" {
                        break;
                    }
                    items.push(Expr::parse(lexer)?);
                }
                let _ = lexer.expect_symbols(&[")"])?;
                Ok(Self::List {
                    open_paren: symbol,
                    items,
                })
            }
            _ => Ok(Expr::Atom(Atom::from_symbol(symbol)))
        }
    }

    pub fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Atom(Atom::Symbol(symbol)) | Self::Atom(Atom::Integer{symbol, ..}) => &symbol.loc,
            Self::List{open_paren, ..} | Self::Eval{open_paren, ..} => &open_paren.loc,
        }
    }

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: &Scope<'nsa>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match self {
            Expr::Atom(Atom::Symbol(pattern_symbol)) => {
                if scope.contains_key(pattern_symbol) {
                    if let Some(existing_value) = bindings.get(pattern_symbol) {
                        existing_value == value
                    } else {
                        bindings.insert(*pattern_symbol, value.clone());
                        true
                    }
                } else {
                    match value {
                        Expr::Atom(Atom::Symbol(value_symbol)) => pattern_symbol == value_symbol,
                        Expr::List{..} | Expr::Eval{..} | Expr::Atom(Atom::Integer{..}) => false,
                    }
                }
            }
            Expr::Atom(Atom::Integer{value: pattern_value, ..}) => {
                match value {
                    Expr::Atom(Atom::Integer{value: value_value, ..}) => pattern_value == value_value,
                    Expr::Atom(Atom::Symbol(..)) | Expr::List{..} | Expr::Eval{..} => false,
                }
            }
            Expr::Eval{..} => unreachable!(),
            Expr::List{items: pattern_items, ..} => {
                match value {
                    Expr::List{items: value_items, ..} => {
                        if pattern_items.len() != value_items.len() {
                            return false
                        }
                        for (a, b) in pattern_items.iter().zip(value_items.iter()) {
                            if !a.pattern_match(b, scope, bindings) {
                                return false;
                            }
                        }
                        true
                    }
                    Expr::Atom(Atom::Symbol(_)) | Expr::Atom(Atom::Integer{..}) | Expr::Eval{..} => false,
                }
            }
        }
    }
}
