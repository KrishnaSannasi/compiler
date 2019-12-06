mod span;
pub use span::{CodePoint, Span};

pub type LexError = lib_error::Error<Error>;
pub type Result<T, E = LexError> = std::result::Result<T, E>;

pub trait Lexer<'input> {
    fn parse(&mut self) -> Result<Option<Token<'input>>>;
    
    fn parse_with_whitespace(&mut self) -> Result<(Option<Token<'input>>, Option<Token<'input>>)> {
        let token = self.parse()?;

        match token {
            None => Ok((None, None)),
            ws@Some(Token { tok_type: TokenType::Whitespace, .. }) => {
                let token = self.parse()?;

                Ok((ws, token))
            },
            Some(token) => Ok((None, Some(token)))
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
    Whitespace,
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
