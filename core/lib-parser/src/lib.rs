#![allow(clippy::try_err)]

use lib_error::WithContext as _;

use lib_lexer_types::{Keyword, Lexer, Peekable, Symbol, Token, TokenData, TokenType};

use lib_parser_types::{context::ContextRef, Error, Expr, HAst, Literal, Result};

pub use lib_parser_types::context;

macro_rules! try_lex {
    ($lexer:expr) => {
        $lexer.with_context(Error::LexError)?
    };
}

macro_rules! any {
    ($first:expr $(, $item:expr)*) => {
        std::iter::once($first)
            $(.chain(Some($item)))*
    }
}

pub struct Parser<'input, 'hacx, L> {
    lexer: Peekable<'input, L>,
    ctx: ContextRef<'input, 'hacx>,
}

impl<'input, 'hacx, L: Lexer<'input>> Parser<'input, 'hacx, L> {
    pub fn new(lexer: L, ctx: ContextRef<'input, 'hacx>) -> Self {
        Self {
            lexer: lexer.peekable(),
            ctx,
        }
    }

    fn expect(&mut self, tok_type: TokenType) -> Result<Token<'input>> {
        let token = try_lex!(self.lexer.parse());

        match token {
            Some(token) if token.data.tok_type() == tok_type => Ok(token),
            _ => Err(Error::Expected(tok_type))?,
        }
    }

    fn expect_any(
        &mut self,
        tok_type: impl Clone + IntoIterator<Item = TokenType>,
    ) -> Result<Token<'input>> {
        let token = try_lex!(self.lexer.parse());

        match token {
            Some(token)
                if tok_type
                    .clone()
                    .into_iter()
                    .any(|tok_type| tok_type == token.data.tok_type()) =>
            {
                Ok(token)
            }
            _ => Err(Error::ExpectedOneOf(tok_type.into_iter().collect()))?,
        }
    }

    pub fn parse(&mut self) -> Result<Option<HAst<'input, 'hacx>>> {
        let token = try_lex!(self.lexer.parse());
        let token = match token {
            Some(token) => token,
            None => return Ok(None),
        };

        match token.data {
            TokenData::Keyword(Keyword::Let) => self.parse_let(token),
            _ => Ok(None),
        }
    }

    fn parse_let(&mut self, kw_let: Token<'input>) -> Result<Option<HAst<'input, 'hacx>>> {
        let token = self.expect_any(any!(
            TokenType::Keyword(Keyword::Mut),
            TokenType::Identifier
        ))?;

        let (kw_mut, ident) = if let TokenData::Keyword(Keyword::Mut) = token.data {
            let ident = self.expect(TokenType::Identifier)?;

            (Some(token), ident)
        } else {
            (None, token)
        };

        let sym_assign = self.expect(TokenType::Symbol(Symbol::Assign))?;
        let value = self.parse_expr()?;
        let sym_semi = self.expect(TokenType::Symbol(Symbol::Semicolon))?;

        let ast_let = lib_parser_types::Let {
            kw_let,
            kw_mut,
            ident,
            sym_assign,
            value,
            sym_semi,
        };

        Ok(Some(HAst::Let(self.ctx.alloc(ast_let))))
    }

    fn parse_expr(&mut self) -> Result<Expr<'input, 'hacx>> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr<'input, 'hacx>> {
        let first = match try_lex!(self.lexer.parse()) {
            Some(first) => first,
            None => Err(Error::ExpectedOneOf(vec![
                TokenType::Identifier,
                TokenType::Integer,
                TokenType::Float,
            ]))?,
        };

        match first.data.tok_type() {
            TokenType::Identifier => Ok(Expr::Identifier(first)),
            TokenType::Integer => Ok(Expr::Literal(Literal::Integer(first))),
            TokenType::Float => Ok(Expr::Literal(Literal::Float(first))),
            TokenType::StringLiteral => Ok(Expr::Literal(Literal::String(first))),
            TokenType::Symbol(_) | TokenType::Keyword(_) => unreachable!(),
        }
    }

    fn parse_arith_prod(&mut self) -> Result<Expr<'input, 'hacx>> {
        let left = self.parse_primary()?;

        loop {
            try_lex!(self.lexer.peek());
            let right = self.parse_primary()?;
        }
    }
}

impl<'input, 'hacx, L: Lexer<'input>> lib_parser_types::Parser<'input, 'hacx>
    for Parser<'input, 'hacx, L>
{
    fn parse(&mut self) -> Result<Option<HAst<'input, 'hacx>>> {
        self.parse()
    }
}
