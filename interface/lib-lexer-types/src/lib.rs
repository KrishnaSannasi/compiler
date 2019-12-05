mod span;
pub use span::{Span, CodePoint};

pub type Result<T, E = lib_error::Error<Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'input> {
    pub tok_type: TokenType, // 2 bytes
    pub lexeme: &'input str, // 16 bytes
    pub span: Span,          // 16 bytes
                             // 6 padding bytes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier,
    Integer,
    Float,
    Whitespace
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Assign, Dot, Semicolon,
    Add, Sub, Mul, Div, Rem,
    Equal, NotEqual, LessThan, GreaterThan, LessEqual, GreaterEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Let, Mut, Match, Loop, Break,
    Continue, Return, Type,
}
