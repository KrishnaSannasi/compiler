mod span;
pub use span::{CodePoint, Span};

use lib_str_interner::ThinStr;

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
    InvalidFloat(Option<std::num::ParseFloatError>),
    InvalidInt(std::num::ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'input> {
    pub data: TokenData<'input>, // 24 bytes
    pub span: Span,              // 16 bytes
}

#[derive(Debug, Clone, Copy)]
pub struct Real(f64);

impl Eq for Real {}
impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl Real {
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenData<'input> {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier(ThinStr),
    Integer(u128),
    Float(Real),
    StringLiteral(&'input str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier,
    Integer,
    Float,
    StringLiteral,
}

impl TokenData<'_> {
    pub fn tok_type(self) -> TokenType {
        match self {
            TokenData::Symbol(sym) => TokenType::Symbol(sym),
            TokenData::Keyword(kw) => TokenType::Keyword(kw),
            TokenData::Identifier(_) => TokenType::Identifier,
            TokenData::Integer(_) => TokenType::Integer,
            TokenData::Float(_) => TokenType::Float,
            TokenData::StringLiteral(_) => TokenType::StringLiteral,
        }
    }
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
