use lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::{Result, Scope};

#[derive(Debug, Clone)]
pub enum Atom<'nsa> {
    Symbol(Symbol<'nsa>),
    Integer {
        value: i32,
        symbol: Symbol<'nsa>,
    },
}

impl<'nsa> Atom<'nsa> {
    pub fn from_symbol(symbol: Symbol<'nsa>) -> Self {
        match symbol.name.parse::<i32>() {
            Ok(value) => Atom::Integer{symbol, value},
            // TODO: throw a warning if number is treated as a symbol because of an overflow or other stupid reason
            Err(_) => Atom::Symbol(symbol),
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

impl<'nsa> Expr<'nsa> {
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
            Self::Eval{open_paren, lhs, rhs} => {
                let lhs = Box::new(lhs.substitute_bindings(bindings));
                let rhs = Box::new(rhs.substitute_bindings(bindings));
                Self::Eval{open_paren: *open_paren, lhs, rhs}
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
                let lhs = Box::new(Expr::Atom(Atom::from_symbol(lexer.parse_symbol()?)));
                let _ = lexer.expect_symbols(&["+"])?;
                let rhs = Box::new(Expr::Atom(Atom::from_symbol(lexer.parse_symbol()?)));
                let _ = lexer.expect_symbols(&["]"])?;
                Ok(Self::Eval {
                    lhs, rhs, open_paren: symbol
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

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: Option<&Scope<'nsa>>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match self {
            Expr::Atom(Atom::Symbol(pattern_symbol)) => {
                if let Some(scope) = scope {
                    if scope.contains_key(pattern_symbol) {
                        // TODO: check if the name already exists in the bindings
                        bindings.insert(*pattern_symbol, value.clone());
                        true
                    } else {
                        match value {
                            Expr::Atom(Atom::Symbol(value_symbol)) => pattern_symbol == value_symbol,
                            Expr::List{..} | Expr::Eval{..} | Expr::Atom(Atom::Integer{..}) => false,
                        }
                    }
                } else {
                    bindings.insert(*pattern_symbol, value.clone());
                    true
                }
            }
            Expr::Atom(Atom::Integer{value: pattern_value, ..}) => {
                match value {
                    Expr::Atom(Atom::Integer{value: value_value, ..}) => pattern_value == value_value,
                    Expr::Atom(Atom::Symbol(..)) | Expr::List{..} | Expr::Eval{..} => false,
                }
            }
            Expr::Eval{lhs: pattern_lhs, rhs: pattern_rhs, ..} => {
                match value {
                    Expr::Eval{lhs: value_lhs, rhs: value_rhs, ..} => {
                        if !pattern_lhs.pattern_match(value_lhs, scope, bindings) {
                            return false;
                        }
                        if !pattern_rhs.pattern_match(value_rhs, scope, bindings) {
                            return false;
                        }
                        true
                    }
                    Expr::Atom(Atom::Symbol(_)) | Expr::Atom(Atom::Integer{..}) | Expr::List{..} => false,
                }
            }
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
