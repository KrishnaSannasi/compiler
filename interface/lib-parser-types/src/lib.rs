use lib_lexer_types::{Token, TokenType};

pub mod context;

type ParseError = lib_error::Error<Error, lib_lexer_types::LexError>;
pub type Result<T, E = ParseError> = std::result::Result<T, E>;

pub trait Parser<'input, 'hacx> {
    fn parse(&mut self) -> Result<Option<HAstPtr<'input, 'hacx>>>;
}

impl<'input, 'hacx, P: Parser<'input, 'hacx> + ?Sized> Parser<'input, 'hacx> for &mut P {
    fn parse(&mut self) -> Result<Option<HAstPtr<'input, 'hacx>>> {
        P::parse(self)
    }
}

impl<'input, 'hacx, P: Parser<'input, 'hacx> + ?Sized> Parser<'input, 'hacx> for Box<P> {
    fn parse(&mut self) -> Result<Option<HAstPtr<'input, 'hacx>>> {
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
pub type Whitespace<'input> = Option<Token<'input>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HAst<'input, 'hacx> {
    Let(Whitespace<'input>, &'hacx mut Let<'input, 'hacx>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Expr<'input, 'hacx> {
    Literal(Whitespace<'input>, Literal<'input>),
    Identifier(Whitespace<'input>, Token<'input>),
    Neg(ExprPtr<'input, 'hacx>)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Literal<'input> {
    Integer(Token<'input>),
    Float(Token<'input>)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Let<'input, 'hacx> {
    pub kw_let: Token<'input>,
    pub kw_mut: Option<(Whitespace<'input>, Token<'input>)>,
    pub ws_1: Whitespace<'input>,
    pub ident: Token<'input>,
    pub ws_2: Whitespace<'input>,
    pub sym_assign: Token<'input>,
    pub value: Expr<'input, 'hacx>,
    pub ws_3: Whitespace<'input>,
    pub sym_semi: Token<'input>,
}