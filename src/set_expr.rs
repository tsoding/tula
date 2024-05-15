use std::collections::{HashSet, HashMap};
use std::fmt;
use super::lexer::{Lexer, Symbol, Loc};
use super::expr::{Expr, Atom};
use super::Result;

pub type Sets<'nsa> = HashMap<Symbol<'nsa>, SetExpr<'nsa>>;

#[derive(Debug, Clone)]
pub enum SetExpr<'nsa> {
    Named(Symbol<'nsa>),
    /// Set Expression wrapped in parenthesis.
    ///
    /// loc points at the left most `(`
    Enclosed {
        loc: Loc<'nsa>,
        inner: Box<SetExpr<'nsa>>
    },
    Anonymous {
        loc: Loc<'nsa>,
        elements: HashSet<Expr<'nsa>>
    },
    Integer(Symbol<'nsa>),
    Real(Symbol<'nsa>),
    Union {
        lhs: Box<SetExpr<'nsa>>,
        rhs: Box<SetExpr<'nsa>>,
    },
    Diff {
        lhs: Box<SetExpr<'nsa>>,
        rhs: Box<SetExpr<'nsa>>,
    },
    Product {
        elements: Vec<SetExpr<'nsa>>
    }
}

impl<'nsa> fmt::Display for SetExpr<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{name}"),
            Self::Integer(_) => write!(f, "Integer"),
            Self::Real(_) => write!(f, "Real"),
            Self::Enclosed{inner, ..} => write!(f, "({inner})"),
            Self::Product {elements, ..} => {
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{element}")?;
                }
                Ok(())
            }
            Self::Anonymous {elements, ..} => {
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
    fn loc(&self) -> &Loc<'nsa> {
        match self {
            Self::Named(Symbol{loc, ..}) => loc,
            Self::Enclosed{loc, ..} => loc,
            Self::Anonymous{loc, ..} => loc,
            Self::Integer(Symbol{loc, ..}) => loc,
            Self::Real(Symbol{loc, ..}) => loc,
            Self::Union {lhs, ..} => lhs.loc(),
            Self::Diff {lhs, ..} => lhs.loc(),
            Self::Product {elements} => elements.first().expect("Parser must not produce products that have 0 elements").loc(),
        }
    }

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
                Self::Anonymous {
                    loc: symbol.loc,
                    elements
                }
            },
            "(" => {
                let open_paren = lexer.next_symbol().unwrap();
                let inner = Box::new(Self::parse(lexer, sets)?);
                lexer.expect_symbols(&[")"])?;
                Self::Enclosed{loc: open_paren.loc, inner}
            }
            _ => {
                let _ = lexer.next_symbol().unwrap();
                match Atom::from_symbol(symbol)? {
                    Atom::Integer{loc, ..} => {
                        eprintln!("{loc}: ERROR: integer is not a set expression");
                        return Err(())
                    }
                    Atom::Real{loc, ..} => {
                        eprintln!("{loc}: ERROR: real is not a set expression");
                        return Err(())
                    }
                    Atom::Symbol(symbol) => match symbol.name {
                        "Integer" => Self::Integer(symbol),
                        "Real" => Self::Real(symbol),
                        _ => {
                            if !sets.contains_key(&symbol) {
                                eprintln!("{loc}: ERROR: set {symbol} does not exist", loc = symbol.loc);
                                return Err(());
                            }
                            Self::Named(symbol)
                        }
                    }
                }
            }
        };
        Ok(set)
    }

    fn parse_product(lexer: &mut Lexer<'nsa>, sets: &Sets<'nsa>) -> Result<Self> {
        let mut elements = vec![Self::parse_primary(lexer, sets)?];
        while let Some(symbol) = lexer.peek_symbol() {
            if symbol.name == "*" {
                let _ = lexer.next_symbol().unwrap();
                elements.push(Self::parse_primary(lexer, sets)?);
            } else {
                break;
            }
        }
        match elements.len() {
            0 => unreachable!("elements may never be empty"),
            // TODO: think about a better solution that does not involve this kind of hackery
            1 => Ok(elements.pop().unwrap()),
            _ => Ok(Self::Product{elements})
        }
    }

    pub fn parse(lexer: &mut Lexer<'nsa>, sets: &Sets<'nsa>) -> Result<Self> {
        let mut lhs = Self::parse_product(lexer, sets)?;
        while let Some(symbol) = lexer.peek_symbol() {
            match symbol.name {
                "+" => {
                    let _ = lexer.next_symbol().unwrap();
                    let rhs = SetExpr::parse_product(lexer, sets)?;
                    lhs = SetExpr::Union {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
                "-" => {
                    let _ = lexer.next_symbol().unwrap();
                    let rhs = SetExpr::parse_product(lexer, sets)?;
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
            Self::Enclosed{inner, ..} => inner.contains(sets, element),
            Self::Product{elements: product_elements} => {
                match element {
                    Expr::Tuple{elements, ..} => {
                        if elements.len() != product_elements.len() {
                            return false;
                        }

                        for (subelement, subset) in elements.iter().zip(product_elements.iter()) {
                            if !subset.contains(sets, subelement) {
                                return false;
                            }
                        }

                        true
                    }
                    _ => false
                }
            }
            Self::Union{lhs, rhs} => lhs.contains(sets, element) || rhs.contains(sets, element),
            Self::Diff{lhs, rhs} => lhs.contains(sets, element) && !rhs.contains(sets, element),
            Self::Anonymous{elements, ..} => elements.contains(element),
            Self::Integer(_) => matches!(element, Expr::Atom(Atom::Integer{..})),
            Self::Real(_) => matches!(element, Expr::Atom(Atom::Real{..})),
            Self::Named(name) => {
                sets.get(name)
                    .expect("The existence of all Named Set Expressions must be checked upfront")
                    .contains(sets, element)
            }
        }
    }

    pub fn expand(&self, sets: &Sets<'nsa>) -> Result<HashSet<Expr<'nsa>>> {
        match self {
            Self::Product{elements} => {
                let mut product = vec![];
                for element in elements.iter() {
                    product.push(element.expand(sets)?)
                }
                let mut elements = vec![];
                let mut result = HashSet::new();
                expand_product_recursively(&product, self.loc(), &mut elements, &mut result);
                Ok(result)
            }
            Self::Enclosed{inner, ..} => inner.expand(sets),
            Self::Union{lhs, rhs} => Ok(lhs.expand(sets)?.union(&rhs.expand(sets)?).cloned().collect()),
            Self::Diff{lhs, rhs} => Ok(lhs.expand(sets)?.difference(&rhs.expand(sets)?).cloned().collect()),
            Self::Anonymous{elements, ..} => Ok(elements.clone()),
            Self::Integer(Symbol{loc, ..})=> {
                eprintln!("{loc}: Impossible to expand set Integer: it's too big");
                Err(())
            }
            Self::Real(Symbol{loc, ..})=> {
                eprintln!("{loc}: Impossible to expand set Real: it's too big");
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

fn expand_product_recursively<'nsa>(product: &[HashSet<Expr<'nsa>>], element_loc: &Loc<'nsa>, elements: &mut Vec<Expr<'nsa>>, result: &mut HashSet<Expr<'nsa>>) {
    match product {
        [head, tail @ ..] => {
            for element in head {
                elements.push(element.clone());
                expand_product_recursively(tail, element_loc, elements, result);
                elements.pop();
            }
        }
        [] => {
            let new = result.insert(Expr::Tuple{elements: elements.clone(), loc: *element_loc});
            assert!(new);
        }
    }
}
