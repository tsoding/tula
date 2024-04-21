use lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::{Result, Scope};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'nsa> {
    Atom {
        symbol: Symbol<'nsa>,
    },
    Integer {
        value: i32,
        symbol: Symbol<'nsa>,
    },
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

impl<'nsa> fmt::Display for Expr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eval{lhs, rhs, ..} => write!(f, "[{lhs} + {rhs}]"),
            Self::Integer{value, ..} => write!(f, "{value}"),
            Self::Atom{symbol: Symbol{name, ..}} => write!(f, "{name}"),
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

impl<'nsa> Expr<'nsa> {
    pub fn find_symbol(&self, needle: &Symbol<'nsa>) -> Option<&Symbol<'nsa>> {
        match self {
            Self::Atom{symbol} | Self::Integer{symbol, ..} => if symbol == needle {
                Some(symbol)
            } else {
                None
            }
            Self::Eval{lhs, rhs, ..} => {
                lhs.find_symbol(needle).or_else(|| rhs.find_symbol(needle))
            }
            Self::List{items, ..} => {
                items.iter().find_map(|item| item.find_symbol(needle))
            }
        }
    }

    pub fn atom_symbol(&self) -> Option<Symbol<'nsa>> {
        match self {
            &Self::Atom{symbol} | &Self::Integer{symbol, ..} => Some(symbol),
            Self::List{..} | Self::Eval{..} => None,
        }
    }

    pub fn substitute(&self, var: Symbol<'nsa>, expr: Expr<'nsa>) -> Expr<'nsa> {
        match self {
            Self::Atom{symbol} | Self::Integer{symbol, ..} => {
                if symbol.name == var.name {
                    expr
                } else {
                    self.clone()
                }
            }

            Self::Eval{open_paren, lhs, rhs} => {
                let lhs = Box::new(lhs.substitute(var, expr.clone()));
                let rhs = Box::new(rhs.substitute(var, expr.clone()));
                Self::Eval{open_paren: *open_paren, lhs, rhs}
            }

            Self::List{open_paren, items} => {
                let items = items.iter().map(|item| item.substitute(var, expr.clone())).collect();
                Self::List{open_paren: *open_paren, items}
            }
        }
    }

    pub fn from_symbol(symbol: Symbol<'nsa>) -> Self {
        match symbol.name.parse::<i32>() {
            Ok(value) => Self::Integer{symbol, value},
            Err(_) => Self::Atom{symbol},
        }
    }

    pub fn parse(lexer: &mut Lexer<'nsa>) -> Result<Self> {
        let symbol = lexer.parse_symbol()?;
        match symbol.name {
            "[" => {
                let lhs = Box::new(Self::from_symbol(lexer.parse_symbol()?));
                let _ = lexer.expect_symbols(&["+"])?;
                let rhs = Box::new(Self::from_symbol(lexer.parse_symbol()?));
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
            _ => Ok(Self::from_symbol(symbol))
        }
    }

    pub fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Atom{symbol} | Self::Integer{symbol, ..} => &symbol.loc,
            Self::List{open_paren, ..} | Self::Eval{open_paren, ..} => &open_paren.loc,
        }
    }

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: Option<&Scope<'nsa>>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match self {
            Expr::Atom{symbol} | Expr::Integer{symbol, ..} => {
                if let Some(scope) = scope {
                    if scope.contains_key(symbol) {
                        // TODO: check if the name already exists in the bindings
                        bindings.insert(*symbol, value.clone());
                        true
                    } else {
                        match value {
                            Expr::Atom{symbol: symbol2} | Expr::Integer{symbol: symbol2, ..} => symbol == symbol2,
                            Expr::List{..} | Expr::Eval{..} => false,
                        }
                    }
                } else {
                    bindings.insert(*symbol, value.clone());
                    true
                }
            }
            Expr::Eval{lhs: pattern_lhs, rhs: pattern_rhs, ..} => {
                match value {
                    Expr::Atom{..} | Expr::Integer{..} | Expr::List{..} => false,
                    Expr::Eval{lhs: value_lhs, rhs: value_rhs, ..} => {
                        if !pattern_lhs.pattern_match(value_lhs, scope, bindings) {
                            return false;
                        }
                        if !pattern_rhs.pattern_match(value_rhs, scope, bindings) {
                            return false;
                        }
                        true
                    }
                }
            }
            Expr::List{items: pattern_items, ..} => {
                match value {
                    Expr::Atom{..} | Expr::Integer{..} | Expr::Eval{..} => false,
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
                }
            }
        }
    }
}
