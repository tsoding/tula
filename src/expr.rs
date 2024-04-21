use lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::{Result, Scope};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'nsa> {
    Atom {
        name: Symbol<'nsa>,
    },
    List {
        open_paren: Symbol<'nsa>,
        items: Vec<Expr<'nsa>>
    },
}

impl<'nsa> fmt::Display for Expr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom{name: Symbol{name, ..}} => write!(f, "{name}"),
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
    pub fn find_symbol(&self, symbol: &Symbol<'nsa>) -> Option<&Symbol<'nsa>> {
        match self {
            Self::Atom{name} => if name == symbol {
                Some(name)
            } else {
                None
            }
            Self::List{items, ..} => {
                items.iter().find_map(|item| item.find_symbol(symbol))
            }
        }
    }

    pub fn atom_name(&self) -> Option<Symbol<'nsa>> {
        match self {
            &Self::Atom{name} => Some(name),
            Self::List{..} => None,
        }
    }

    pub fn substitute(&self, var: Symbol<'nsa>, sexpr: Expr<'nsa>) -> Expr<'nsa> {
        match self {
            Self::Atom{name} => {
                if name.name == var.name {
                    sexpr
                } else {
                    self.clone()
                }
            }

            Self::List{open_paren, items} => {
                let items = items.iter().map(|item| item.substitute(var, sexpr.clone())).collect();
                Self::List{open_paren: *open_paren, items}
            }
        }
    }

    pub fn parse(lexer: &mut Lexer<'nsa>) -> Result<Self> {
        let symbol1 = lexer.parse_symbol()?;
        match symbol1.name {
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
                    open_paren: symbol1,
                    items,
                })
            }
            _ => Ok(Self::Atom{name: symbol1}),
        }
    }

    pub fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Atom{name} => &name.loc,
            Self::List{open_paren, ..} => &open_paren.loc,
        }
    }

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: Option<&Scope<'nsa>>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match (self, value) {
            (Expr::Atom{name}, _) => {
                if let Some(scope) = scope {
                    if scope.contains_key(name) {
                        // TODO: check if the name already exists in the bindings
                        bindings.insert(*name, value.clone());
                        true
                    } else {
                        match value {
                            Expr::Atom{name: name2} => name == name2,
                            _ => false,
                        }
                    }
                } else {
                    bindings.insert(*name, value.clone());
                    true
                }
            }
            (Expr::List{items: pattern_items, ..}, Expr::List{items: value_items, ..}) => {
                if pattern_items.len() == value_items.len() {
                    for (a, b) in pattern_items.iter().zip(value_items.iter()) {
                        if !a.pattern_match(b, scope, bindings) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }
}
