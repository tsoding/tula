use super::lexer::*;
use std::fmt;
use std::collections::HashMap;
use super::{Result, Scope};
use std::hash::{Hash, Hasher};
use std::num::IntErrorKind;

#[derive(Debug, Clone)]
pub enum Atom<'nsa> {
    Symbol(Symbol<'nsa>),
    Integer {
        loc: Loc<'nsa>,
        value: i64,
    },
    Real {
        loc: Loc<'nsa>,
        value: f32,
    },
    String {
        loc: Loc<'nsa>,
        value: String,
    },
}

impl<'nsa> Atom<'nsa> {
    pub fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Symbol(Symbol{loc, ..}) => loc,
            Self::Integer{loc, ..} => loc,
            Self::Real{loc, ..} => loc,
            Self::String{loc, ..} => loc,
        }
    }

    pub fn human(&self) -> &'static str {
        match self {
            Self::Symbol(..) => "symbol",
            Self::Integer{..} => "integer value",
            Self::Real{..} => "real value",
            Self::String{..} => "string value",
        }
    }

    pub fn expect_real(&self) -> Result<f32> {
        match self {
            &Self::Real{value, ..} => Ok(value),
            _ => {
                eprintln!("{loc}: ERROR: expected real value but got {human} `{value}`", loc = self.loc(), human = self.human(), value = self);
                Err(())
            }
        }
    }

    pub fn expect_integer(&self) -> Result<i64> {
        match self {
            &Self::Integer{value, ..} => Ok(value),
            _ => {
                eprintln!("{loc}: ERROR: expected integer value but got {human} `{value}`", loc = self.loc(), human = self.human(), value = self);
                Err(())
            }
        }
    }

    pub fn expect_symbol(&self) -> Result<&Symbol<'nsa>> {
        match self {
            Self::Symbol(symbol) => Ok(symbol),
            _ => {
                eprintln!("{loc}: ERROR: expected symbol value but got {human} `{value}`", loc = self.loc(), human = self.human(), value = self);
                Err(())
            }
        }
    }

    pub fn expect_string(&self) -> Result<&String> {
        match self {
            Self::String{value, ..} => Ok(value),
            _ => {
                eprintln!("{loc}: ERROR: expected string value but got {human} `{value}`", loc = self.loc(), human = self.human(), value = self);
                Err(())
            }
        }
    }

    pub fn from_symbol(symbol: Symbol<'nsa>) -> Result<Self> {
        match symbol.name.parse::<i64>() {
            Ok(value) => return Ok(Atom::Integer{loc: symbol.loc, value}),
            Err(err) => {
                match err.kind() {
                    IntErrorKind::PosOverflow => {
                        eprintln!("{loc}: ERROR: could not parse Integer because positive overflow", loc = symbol.loc);
                        return Err(())
                    }
                    IntErrorKind::NegOverflow => {
                        eprintln!("{loc}: ERROR: could not parse Integer because negative overflow", loc = symbol.loc);
                        return Err(())
                    }
                    _ => {}
                }
            }
        }
        if let Ok(value) = symbol.name.parse::<f32>() {
            return Ok(Atom::Real{loc: symbol.loc, value});
        }
        Ok(Atom::Symbol(symbol))
    }

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: &Scope<'nsa>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match self {
            Atom::Symbol(pattern_symbol) => {
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
                        _ => false,
                    }
                }
            }
            Atom::Integer{value: pattern_value, ..} => {
                match value {
                    Expr::Atom(Atom::Integer{value: value_value, ..}) => pattern_value == value_value,
                    _ => false,
                }
            }
            Atom::Real{value: pattern_value, ..} => {
                match value {
                    Expr::Atom(Atom::Real{value: value_value, ..}) => pattern_value == value_value,
                    _ => false,
                }
            }
            Atom::String{value: pattern_value, ..} => {
                match value {
                    Expr::Atom(Atom::String{value: value_value, ..}) => pattern_value == value_value,
                    _ => false,
                }
            }
        }
    }
}

impl<'nsa> Hash for Atom<'nsa> {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        match self {
            Self::Symbol(symbol) => symbol.hash(h),
            Self::Integer{value, ..} => value.hash(h),
            Self::Real{value, ..} => value.to_le_bytes().hash(h),
            Self::String{value, ..} => value.hash(h),
        }
    }
}

impl<'nsa> PartialEq for Atom<'nsa> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol(symbol), Self::Symbol(other_symbol)) => symbol == other_symbol,
            (Self::Real{value, ..}, Self::Real{value: other_value, ..}) => value == other_value,
            (Self::Integer{value, ..}, Self::Integer{value: other_value, ..}) => value == other_value,
            (Self::String{value, ..}, Self::String{value: other_value, ..}) => value == other_value,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'nsa> {
    Atom(Atom<'nsa>),
    Eval {
        loc: Loc<'nsa>,
        lhs: Box<Expr<'nsa>>,
        op: Box<Expr<'nsa>>,
        rhs: Box<Expr<'nsa>>,
    },
    Tuple {
        loc: Loc<'nsa>,
        elements: Vec<Expr<'nsa>>
    },
}

impl<'nsa> PartialEq for Expr<'nsa> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Atom(atom) => {
                match other {
                    Self::Atom(other_atom) => atom == other_atom,
                    Self::Eval{..} | Self::Tuple{..} => false,
                }
            }
            Self::Eval{lhs, rhs, ..} => {
                match other {
                    Self::Eval{lhs: other_lhs, rhs: other_rhs, ..} => lhs == other_lhs && rhs == other_rhs,
                    Self::Atom(_) | Self::Tuple{..} => false,
                }
            }
            Self::Tuple{elements, ..} => {
                match other {
                    Self::Tuple{elements: other_elements, ..} => elements == other_elements,
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
            Self::Tuple{elements, ..} => elements.hash(h),
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
            Atom::Real{value, ..} => write!(f, "{value}"),
            Atom::String{value, ..} => write!(f, "'{value}'"),
        }
    }
}

impl<'nsa> fmt::Display for Expr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{atom}"),
            Self::Eval{lhs, rhs, ..} => write!(f, "[{lhs} + {rhs}]"),
            Self::Tuple{elements, ..} => {
                write!(f, "(")?;
                for (i, element) in elements.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{element}")?
                    } else {
                        write!(f, " {element}")?
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
            Expr::Tuple{elements, ..} => {
                write!(f, "_")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, "_")?;
                    }
                    write!(f, "{}", NormExpr(element))?;
                }
                write!(f, "_")
            }
            Expr::Eval{..} => todo!("I don't know how to normalize these things yet"),
        }
    }
}

fn expect_bool(symbol: &Symbol) -> Result<bool> {
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
            Self::Tuple{loc, ..} => {
                eprintln!("{loc}: ERROR: expected atom but got tuple");
                Err(())
            }
            Self::Eval{loc, ..} => {
                eprintln!("{loc}: ERROR: expected atom but got eval");
                Err(())
            }
        }
    }

    pub fn force_evals(self) -> Result<Expr<'nsa>> {
        match self {
            Self::Atom(_) => Ok(self),
            Self::Tuple{loc, elements} => {
                let mut new_elements = vec![];
                for element in elements {
                    new_elements.push(element.force_evals()?)
                }
                Ok(Self::Tuple{loc, elements: new_elements})
            }
            Self::Eval{loc, lhs, op, rhs} => {
                let lhs = lhs.force_evals()?.expect_atom()?.clone();
                match lhs {
                    Atom::Integer{value: lhs, ..} => {
                        let rhs = rhs.force_evals()?.expect_atom()?.expect_integer()?;
                        let op  = *op.force_evals()?.expect_atom()?.expect_symbol()?;
                        match op.name {
                            "+" => Ok(Expr::Atom(Atom::Integer {
                                loc,
                                value: if let Some(value) = lhs.checked_add(rhs) {
                                    value
                                } else {
                                    eprintln!("{loc}: ERROR: integer overflow while trying evaluate [{lhs} {op} {rhs}]");
                                    return Err(());
                                },
                            })),
                            "-" => Ok(Expr::Atom(Atom::Integer {
                                loc,
                                value: if let Some(value) = lhs.checked_sub(rhs) {
                                    value
                                } else {
                                    eprintln!("{loc}: ERROR: integer overflow while trying evaluate [{lhs} {op} {rhs}]");
                                    return Err(());
                                },
                            })),
                            "*" => Ok(Expr::Atom(Atom::Integer {
                                loc,
                                value: if let Some(value) = lhs.checked_mul(rhs) {
                                    value
                                } else {
                                    eprintln!("{loc}: ERROR: integer overflow while trying evaluate [{lhs} {op} {rhs}]");
                                    return Err(());
                                },
                            })),
                            "/" => Ok(Expr::Atom(Atom::Integer {
                                loc,
                                value: if let Some(value) = lhs.checked_div(rhs) {
                                    value
                                } else {
                                    eprintln!("{loc}: ERROR: integer overflow while trying evaluate [{lhs} {op} {rhs}]");
                                    return Err(());
                                },
                            })),
                            "%" => Ok(Expr::Atom(Atom::Integer {
                                loc,
                                value: if let Some(value) = lhs.checked_rem(rhs) {
                                    value
                                } else {
                                    eprintln!("{loc}: ERROR: integer overflow while trying evaluate [{lhs} {op} {rhs}]");
                                    return Err(());
                                },
                            })),
                            ">" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs > rhs),
                            }))),
                            ">=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs >= rhs),
                            }))),
                            "<" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs < rhs),
                            }))),
                            "<=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs <= rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs == rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
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
                        let op  = *op.force_evals()?.expect_atom()?.expect_symbol()?;
                        match op.name {
                            "||" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs || rhs),
                            }))),
                            "&&" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs && rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs == rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs != rhs),
                            }))),
                            _ => {
                                eprintln!("{loc}: ERROR: Unexpected Boolean operation", loc = op.loc);
                                Err(())
                            }
                        }
                    }
                    Atom::Real{value: lhs, ..} => {
                        let rhs = rhs.force_evals()?.expect_atom()?.expect_real()?;
                        let op  = *op.force_evals()?.expect_atom()?.expect_symbol()?;
                        match op.name {
                            "+" => Ok(Expr::Atom(Atom::Real {
                                loc,
                                value: lhs + rhs,
                            })),
                            "-" => Ok(Expr::Atom(Atom::Real {
                                loc,
                                value: lhs - rhs,
                            })),
                            "*" => Ok(Expr::Atom(Atom::Real {
                                loc,
                                value: lhs * rhs,
                            })),
                            "/" => Ok(Expr::Atom(Atom::Real {
                                loc,
                                value: lhs / rhs,
                            })),
                            "%" => Ok(Expr::Atom(Atom::Real {
                                loc,
                                value: lhs % rhs,
                            })),
                            ">" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs > rhs),
                            }))),
                            ">=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs >= rhs),
                            }))),
                            "<" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs < rhs),
                            }))),
                            "<=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs <= rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs == rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs != rhs),
                            }))),
                            _ => {
                                eprintln!("{loc}: ERROR: Unexpected Integer operation {op}", loc = op.loc);
                                Err(())
                            }
                        }
                    }
                    Atom::String{value: lhs, ..} => {
                        let rhs = rhs.force_evals()?.expect_atom()?.expect_string()?.clone();
                        let op  = *op.force_evals()?.expect_atom()?.expect_symbol()?;
                        match op.name {
                            "+" => Ok(Expr::Atom(Atom::String {
                                loc,
                                value: lhs + &rhs,
                            })),
                            ">" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs > rhs),
                            }))),
                            ">=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs >= rhs),
                            }))),
                            "<" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs < rhs),
                            }))),
                            "<=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs <= rhs),
                            }))),
                            "==" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs == *rhs),
                            }))),
                            "!=" => Ok(Expr::Atom(Atom::Symbol(Symbol {
                                loc,
                                name: bool_to_str(lhs != *rhs),
                            }))),
                            _ => {
                                eprintln!("{loc}: ERROR: Unexpected Integer operation {op}", loc = op.loc);
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
            Self::Atom(Atom::Integer{..}) |
            Self::Atom(Atom::Real{..}) |
            Self::Atom(Atom::String{..})=> None,
            Self::Eval{lhs, rhs, ..} => {
                lhs.uses_var(var).or_else(|| rhs.uses_var(var))
            }
            Self::Tuple{elements, ..} => {
                elements.iter().find_map(|element| element.uses_var(var))
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
            Self::Atom(Atom::Integer{..}) |
            Self::Atom(Atom::Real{..}) |
            Self::Atom(Atom::String{..})=> self.clone(),
            Self::Eval{loc, lhs, op, rhs} => {
                let lhs = Box::new(lhs.substitute_bindings(bindings));
                let op  = Box::new(op.substitute_bindings(bindings));
                let rhs = Box::new(rhs.substitute_bindings(bindings));
                Self::Eval{loc: *loc, lhs, op, rhs}
            }
            Self::Tuple{loc, elements} => {
                let elements = elements.iter().map(|element| element.substitute_bindings(bindings)).collect();
                Self::Tuple{loc: *loc, elements}
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
                    lhs, op, rhs, loc: symbol.loc
                })
            }
            "(" => {
                let mut elements = vec![];
                while let Some(symbol2) = lexer.peek_symbol() {
                    if symbol2.name == ")" {
                        break;
                    }
                    elements.push(Expr::parse(lexer)?);
                }
                let _ = lexer.expect_symbols(&[")"])?;
                Ok(Self::Tuple {
                    loc: symbol.loc,
                    elements,
                })
            }
            _ => Ok(Expr::Atom(Atom::from_symbol(symbol)?))
        }
    }

    pub fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Tuple{loc, ..} | Self::Eval{loc, ..} => loc,
            Self::Atom(atom) => atom.loc(),
        }
    }

    pub fn pattern_match(&self, value: &Expr<'nsa>, scope: &Scope<'nsa>, bindings: &mut HashMap<Symbol<'nsa>, Expr<'nsa>>) -> bool {
        match self {
            Expr::Atom(pattern_atom) => pattern_atom.pattern_match(value, scope, bindings),
            Expr::Eval{..} => unreachable!("Pattern matching of Eval nodes should be forbidden before it's even tried"),
            Expr::Tuple{elements: pattern_elements, ..} => {
                match value {
                    Expr::Tuple{elements: value_elements, ..} => {
                        if pattern_elements.len() != value_elements.len() {
                            return false
                        }
                        for (a, b) in pattern_elements.iter().zip(value_elements.iter()) {
                            if !a.pattern_match(b, scope, bindings) {
                                return false;
                            }
                        }
                        true
                    }
                    _ => false,
                }
            }
        }
    }
}
