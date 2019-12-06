use lib_lexer_types::{Symbol, Token, TokenType};

pub mod context;

type ParseError = lib_error::Error<Error, lib_lexer_types::LexError>;
pub type Result<T, E = ParseError> = std::result::Result<T, E>;

pub trait Parser<'input, 'hacx> {
    fn parse(&mut self) -> Result<Option<HAst<'input, 'hacx>>>;
}

impl<'input, 'hacx, P: Parser<'input, 'hacx> + ?Sized> Parser<'input, 'hacx> for &mut P {
    fn parse(&mut self) -> Result<Option<HAst<'input, 'hacx>>> {
        P::parse(self)
    }
}

impl<'input, 'hacx, P: Parser<'input, 'hacx> + ?Sized> Parser<'input, 'hacx> for Box<P> {
    fn parse(&mut self) -> Result<Option<HAst<'input, 'hacx>>> {
        P::parse(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    LexError,
    Expected(TokenType),
    ExpectedOneOf(Vec<TokenType>),
}

pub type HAstPtr<'input, 'hacx> = &'hacx mut HAst<'input, 'hacx>;
pub type ExprPtr<'input, 'hacx> = &'hacx mut Expr<'input, 'hacx>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HAst<'input, 'hacx> {
    Let(&'hacx mut Let<'input, 'hacx>),
    Assign(&'hacx mut Assign<'input, 'hacx>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Expr<'input, 'hacx> {
    Literal(Literal<'input>),
    Identifier(Token<'input>),
    Prefix(Symbol, ExprPtr<'input, 'hacx>),
    Postfix(ExprPtr<'input, 'hacx>, Symbol),
    Binary(ExprPtr<'input, 'hacx>, Symbol, ExprPtr<'input, 'hacx>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Literal<'input> {
    Integer(Token<'input>),
    Float(Token<'input>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Let<'input, 'hacx> {
    pub kw_let: Token<'input>,
    pub kw_mut: Option<Token<'input>>,
    pub ident: Token<'input>,
    pub sym_assign: Token<'input>,
    pub value: Expr<'input, 'hacx>,
    pub sym_semi: Token<'input>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Assign<'input, 'hacx> {
    pub ident: Token<'input>,
    pub sym_assign: Token<'input>,
    pub value: Expr<'input, 'hacx>,
    pub sym_semi: Token<'input>,
}
