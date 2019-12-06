#![allow(clippy::try_err)]

use lib_error::WithContext as _;

use lib_lexer_types::{
    Lexer,
    Token,
    TokenType,
    Keyword,
    Symbol,
};

use lib_parser_types::{
    Result,
    Error,
    HAst,
    Expr,
    HAstPtr,
    context::ContextRef,
};

pub use lib_parser_types::context;

macro_rules! try_lex {
    ($lexer:expr) => {
        $lexer.with_context(Error::LexError)?
    }
}

macro_rules! any {
    ($first:expr $(, $item:expr)*) => {
        std::iter::once($first)
            $(.chain(Some($item)))*
    }
}

pub struct Parser<'input, 'hacx, L> {
    lexer: L,
    ctx: ContextRef<'input, 'hacx>
}

impl<'input, 'hacx, L: Lexer<'input>> Parser<'input, 'hacx, L> {
    pub fn new(lexer: L, ctx: ContextRef<'input, 'hacx>) -> Self {
        Self { lexer, ctx }
    }

    fn expect(&mut self, tok_type: TokenType) -> Result<(Option<Token<'input>>, Token<'input>)> {
        let (ws, token) = try_lex!(self.lexer.parse_with_whitespace());

        match token {
            Some(token) if token.tok_type == tok_type => Ok((ws, token)),
            _ => Err(Error::Expected(tok_type))?
        }
    }

    fn expect_any(&mut self, tok_type: impl Clone + IntoIterator<Item = TokenType>) -> Result<(Option<Token<'input>>, Token<'input>)> {
        let (ws, token) = try_lex!(self.lexer.parse_with_whitespace());

        match token {
            Some(token) if tok_type.clone().into_iter().any(|tok_type| tok_type == token.tok_type) => Ok((ws, token)),
            _ => Err(Error::ExpectedOneOf(tok_type.into_iter().collect()))?
        }
    }

    pub fn parse(&mut self) -> Result<Option<HAstPtr<'input, 'hacx>>> {
        let (ws, token) = try_lex!(self.lexer.parse_with_whitespace());
        let token = match token {
            Some(token) => token,
            None => return Ok(None)
        };

        match token.tok_type {
            TokenType::Keyword(Keyword::Let) => self.parse_let(ws, token),
            _ => Ok(None)
        }
    }

    fn parse_let(&mut self, ws_0: Option<Token<'input>>, kw_let: Token<'input>) -> Result<Option<HAstPtr<'input, 'hacx>>> {
        let (ws, token) = self.expect_any(any!(TokenType::Keyword(Keyword::Mut), TokenType::Identifier))?;
        
        let (kw_mut, ws_1, ident) = if let TokenType::Keyword(Keyword::Mut) = token.tok_type {
            let (ws_1, ident) = self.expect(TokenType::Identifier)?;

            (Some((ws, token)), ws_1, ident)
        } else {
            (None, ws, token)
        };
        
        let (ws_2, sym_assign) = self.expect(TokenType::Symbol(Symbol::Assign))?;
        let (ws_value, value) = self.expect(TokenType::Identifier)?;
        let (ws_3, sym_semi) = self.expect(TokenType::Symbol(Symbol::Semicolon))?;

        let value = Expr::Identifier(ws_value, value);

        #[rustfmt::ignore]
        let ast_let = lib_parser_types::Let {
                  kw_let,
                  kw_mut,
            ws_1, ident,
            ws_2, sym_assign,
                  value,
            ws_3, sym_semi,
        };

        let ast_let = HAst::Let(ws_0, self.ctx.alloc(ast_let));

        Ok(Some(self.ctx.alloc(ast_let)))
    }
}

impl<'input, 'hacx, L: Lexer<'input>> lib_parser_types::Parser<'input, 'hacx> for Parser<'input, 'hacx, L> {
    fn parse(&mut self) -> Result<Option<HAstPtr<'input, 'hacx>>> {
        self.parse()
    }
}
