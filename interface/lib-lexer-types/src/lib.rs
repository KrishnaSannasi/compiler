mod span;
pub use span::{CodePoint, Span};

pub type LexError = lib_error::Error<Error>;
pub type Result<T, E = LexError> = std::result::Result<T, E>;

pub trait Lexer<'input> {
    fn parse(&mut self) -> Result<Option<Token<'input>>>;

    fn peekable(self) -> Peekable<'input, Self>
    where
        Self: Sized,
    {
        Peekable {
            inner: self,
            tokens: Vec::new(),
        }
    }
}

impl<'input, L: Lexer<'input> + ?Sized> Lexer<'input> for &mut L {
    fn parse(&mut self) -> Result<Option<Token<'input>>> {
        L::parse(self)
    }
}

impl<'input, L: Lexer<'input> + ?Sized> Lexer<'input> for Box<L> {
    fn parse(&mut self) -> Result<Option<Token<'input>>> {
        L::parse(self)
    }
}

pub struct Peekable<'input, L> {
    inner: L,
    tokens: Vec<Token<'input>>,
}

impl<'input, L: Lexer<'input>> Peekable<'input, L> {
    pub fn peek(&mut self) -> Result<Option<Token<'input>>> {
        if self.tokens.is_empty() {
            self.tokens.extend(self.inner.parse()?)
        }

        Ok(self.tokens.last().copied())
    }

    // fn peek_n(&mut self, n: usize) -> Option<&[Token<'input>]> {
    //     if self.tokens.len() < n {
    //         self.tokens.extend(self.inner.parse()?)
    //     }

    //     Ok(self.tokens.last().copied())
    // }
}

impl<'input, L: Lexer<'input>> Lexer<'input> for Peekable<'input, L> {
    fn parse(&mut self) -> Result<Option<Token<'input>>> {
        match self.tokens.pop() {
            Some(token) => Ok(Some(token)),
            None => self.inner.parse(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub err: ErrorType,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    UnknownCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Input<'input> {
    pub lexeme: &'input str,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'input> {
    pub tok_type: TokenType,               // 2 bytes
    pub whitespace: Option<Input<'input>>, // 32 bytes
    pub input: Input<'input>,              // 32 bytes
                                           // 6 padding bytes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier,
    Integer,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Assign,
    Dot,
    Semicolon,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Let,
    Mut,
    Match,
    Loop,
    Break,
    Continue,
    Return,
    Type,
}
