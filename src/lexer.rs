use std::fmt;
use std::iter::Iterator;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use super::Result;

pub const SPECIAL: &[char] = &['(', ')', '{', '}', ':'];

#[derive(Debug, Clone, Copy)]
pub struct Loc<'nsa> {
    pub file_path: &'nsa str,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol<'nsa> {
    pub name: &'nsa str,
    pub loc: Loc<'nsa>,
}

impl<'nsa> PartialEq for Symbol<'nsa> {
    fn eq(&self, other: &Symbol<'nsa>) -> bool {
        self.name.eq(other.name)
    }
}

impl<'nsa> Eq for Symbol<'nsa> {}

impl<'nsa> Hash for Symbol<'nsa> {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        self.name.hash(h)
    }
}

impl<'nsa> fmt::Display for Loc<'nsa> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Loc{file_path, row, col} = self;
        write!(f, "{file_path}:{row}:{col}")
    }
}

pub struct Lexer<'nsa> {
    source: &'nsa str,
    file_path: &'nsa str,
    pos: usize,
    bol: usize,
    row: usize,
    peek: Option<Symbol<'nsa>>,
}

impl<'nsa> Lexer<'nsa> {
    pub fn new(source: &'nsa str, file_path: &'nsa str) -> Self {
        Self {
            source,
            file_path,
            pos: 0,
            bol: 0,
            row: 0,
            peek: None,
        }
    }

    pub fn loc(&self) -> Loc<'nsa> {
        Loc {
            file_path: self.file_path,
            row: self.row + 1,
            col: self.pos - self.bol + 1,
        }
    }

    fn advance_loc(&mut self, skipped_char: char) {
        self.pos += 1;
        if skipped_char == '\n' {
            self.bol = self.pos;
            self.row += 1;
        }
    }

    fn strip_char_prefix(&mut self, prefix: char) -> Option<&'nsa str> {
        if self.source.starts_with(prefix) {
            let mut char_indices = self.source.char_indices();
            let _ = char_indices.next();
            let end = char_indices
                .next()
                .map(|(i, _)| i)
                .unwrap_or(self.source.len());
            self.advance_loc(prefix);
            let result = &self.source[..end];
            self.source = &self.source[end..];
            Some(result)
        } else {
            None
        }
    }

    fn strip_prefix(&mut self, prefix: &str) -> bool {
        if let Some(source) = self.source.strip_prefix(prefix) {
            for x in prefix.chars() {
                self.advance_loc(x);
            }
            self.source = source;
            true
        } else {
            false
        }
    }

    fn strip_while<P>(&mut self, mut skip: P) -> &'nsa str where P: FnMut(&char) -> bool {
        let end = self.source
            .char_indices()
            .find(|(_, x)| {
                if skip(x) {
                    self.advance_loc(*x);
                    false
                } else {
                    true
                }
            })
            .map(|(i, _)| i)
            .unwrap_or(self.source.len());
        let prefix = &self.source[..end];
        self.source = &self.source[end..];
        prefix
    }

    fn chop_symbol(&mut self) -> Option<Symbol<'nsa>> {
        'strip_whitespaces_and_comments: loop {
            let _ = self.strip_while(|x| x.is_whitespace());
            if self.strip_prefix("//") {
                let _ = self.strip_while(|x| *x != '\n');
            } else {
                break 'strip_whitespaces_and_comments
            }
        }

        if self.source.is_empty() {
            return None
        }

        let loc = self.loc();

        for name in SPECIAL {
            if let Some(name) = self.strip_char_prefix(*name) {
                return Some(Symbol{ name, loc });
            }
        }

        if self.source.starts_with("'") {
            let mut char_indices = self.source.char_indices();
            self.advance_loc(char_indices.next().unwrap().1);

            while let Some((_, x)) = char_indices.next() {
                // TODO: implement escaping inside of symbol literals
                self.advance_loc(x);
                if x == '\'' {
                    break;
                }
            }

            let end = char_indices.next().map(|(i, _)| i).unwrap_or(self.source.len());

            let name = &self.source[..end];
            self.source = &self.source[end..];
            return Some(Symbol { name, loc })
        }

        let name = self.strip_while(|x| !x.is_whitespace() && !SPECIAL.contains(x) && *x != '\'');
        Some(Symbol { name, loc })
    }

    pub fn next_symbol(&mut self) -> Option<Symbol<'nsa>> {
        self.peek.take().or_else(|| self.chop_symbol())
    }

    pub fn peek_symbol(&mut self) -> Option<Symbol<'nsa>> {
        if self.peek.is_none() {
            self.peek = self.chop_symbol();
        }
        self.peek
    }

    pub fn parse_symbol(&mut self) -> Result<Symbol<'nsa>> {
        if let Some(symbol) = self.next_symbol() {
            Ok(symbol)
        } else {
            eprintln!("{loc}: ERROR: expected symbol but reached the end of the input", loc = self.loc());
            Err(())
        }
    }

    pub fn expect_symbols(&mut self, expected_names: &[&str]) -> Result<Symbol<'nsa>> {
        let symbol = self.parse_symbol()?;
        for name in expected_names.iter() {
            if &symbol.name == name {
                return Ok(symbol);
            }
        }
        let mut buffer = String::new();
        for (i, name) in expected_names.iter().enumerate() {
            if i == 0 {
                let _ = write!(&mut buffer, "{name}");
            } if i + 1 == expected_names.len() {
                let _ = write!(&mut buffer, ", or {name}");
            } else {
                let _ = write!(&mut buffer, ", {name}");
            }
        }
        eprintln!("{loc}: ERROR: expected {buffer} got {name}", loc = symbol.loc, name = symbol.name);
        Err(())
    }
}

impl<'nsa> Iterator for Lexer<'nsa> {
    type Item = Symbol<'nsa>;
    fn next(&mut self) -> Option<Symbol<'nsa>> {
        self.next_symbol()
    }
}
