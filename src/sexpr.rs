use lexer::*;
use super::Result;

#[derive(Debug)]
pub enum Sexpr<'nsa> {
    Atom {
        name: Symbol<'nsa>,
    },
    List {
        open_paren: Symbol<'nsa>,
        items: Vec<Sexpr<'nsa>>
    },
}

impl<'nsa> Sexpr<'nsa> {
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

    pub fn dump(&self, level: usize) {
        match self {
            Self::Atom{name} => {
                println!("{loc}: {pad:width$} {name}", loc = name.loc, pad = "", width = level*2, name = name.name);
            }
            Self::List{open_paren, items} => {
                println!("{loc}: {pad:width$} List:", loc = open_paren.loc, pad = "", width = level*2);
                for item in items {
                    item.dump(level + 1);
                }
            }
        }
    }
}
