use std::fmt;
use std::iter::Iterator;

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

        let special = &["(", ")", "{", "}", ":"];
        for name in special {
            if self.strip_prefix(name) {
                return Some(Symbol{ name, loc });
            }
        }

        let name = self.strip_while(|x| !x.is_whitespace());
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
}

impl<'nsa> Iterator for Lexer<'nsa> {
    type Item = Symbol<'nsa>;
    fn next(&mut self) -> Option<Symbol<'nsa>> {
        self.next_symbol()
    }
}
