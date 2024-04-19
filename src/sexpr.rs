use lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::Result;

#[derive(Debug, Clone)]
pub enum Sexpr<'nsa> {
    Atom {
        name: Symbol<'nsa>,
    },
    List {
        open_paren: Symbol<'nsa>,
        items: Vec<Sexpr<'nsa>>
    },
}

impl<'nsa> fmt::Display for Sexpr<'nsa> {
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

impl<'nsa> Sexpr<'nsa> {
    pub fn atom_name(&self) -> Option<Symbol<'nsa>> {
        match self {
            &Self::Atom{name} => Some(name),
            Self::List{..} => None,
        }
    }

    pub fn matches(&self, other: &Sexpr<'nsa>) -> bool {
        match (self, other) {
            (Self::Atom{name: name1}, Self::Atom{name: name2}) => {
                name1.name == name2.name
            }
            (Self::List{items: items1, ..}, Self::List{items: items2, ..}) => {
                if items1.len() != items2.len() {
                    return false
                }

                for (a, b) in items1.iter().zip(items2.iter()) {
                    if !a.matches(b) {
                        return false
                    }
                }

                true
            }
            _ => false
        }
    }

    pub fn substitute(&self, var: Symbol<'nsa>, sexpr: Sexpr<'nsa>) -> Sexpr<'nsa> {
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
                    items.push(Sexpr::parse(lexer)?);
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

    pub fn pattern_match(&self, value: &Sexpr<'nsa>, bindings: &mut HashMap<Symbol<'nsa>, Sexpr<'nsa>>) -> bool {
        match (self, value) {
            (Sexpr::Atom{name}, _) => {
                bindings.insert(*name, value.clone());
                true
            }
            (Sexpr::List{items: pattern_items, ..}, Sexpr::List{items: value_items, ..}) => {
                if pattern_items.len() == value_items.len() {
                    for (a, b) in pattern_items.iter().zip(value_items.iter()) {
                        if !a.pattern_match(b, bindings) {
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
