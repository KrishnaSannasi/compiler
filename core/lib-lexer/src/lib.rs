use lib_lexer_types::{
    CodePoint, Error, ErrorType, Keyword, Real, Result, Symbol, Token, TokenType,
};

macro_rules! get_token_ty_from_ident {
    ($($kw:ident => $value:literal),* $(,)?) => {
        |ident| {
            #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
            'here: loop {
                let unreachable: Keyword = break TokenType::Keyword(match ident {
                    $($value => Keyword::$kw,)*
                    _ => break 'here TokenType::Identifier(lib_str_interner::intern(ident))
                });

                match unreachable {
                    $(Keyword::$kw => ()),*
                }
            }
        }
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    start: CodePoint,
}

fn split_on_false<F: FnMut(char) -> bool>(s: &str, mut f: F) -> (&str, &str) {
    let len = s
        .chars()
        .take_while(move |&c| f(c))
        .map(|c| c.len_utf8())
        .sum();

    s.split_at(len)
}

impl<'input> lib_lexer_types::Lexer<'input> for Lexer<'input> {
    fn parse(&mut self) -> Result<Option<Token<'input>>> {
        self.parse()
    }
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            start: CodePoint::new_unchecked(1, 1),
        }
    }

    pub fn parse(&mut self) -> Result<Option<Token<'input>>> {
        let first = match self.input.chars().next() {
            Some(first) => first,
            None => return Ok(None),
        };

        if first.is_whitespace() {
            let mut rows = 0;
            let mut cols = 0;
            let (lexeme, rest) = split_on_false(self.input, |c| {
                let is_whitespace = c.is_whitespace();
                if c == '\n' {
                    rows += 1;
                    cols = 1;
                } else {
                    cols += u32::from(is_whitespace);
                }

                is_whitespace
            });

            self.input = rest;

            self.start = CodePoint::new_unchecked(
                self.start.row() + rows,
                if rows == 0 {
                    self.start.col() + lexeme.len() as u32
                } else {
                    cols
                },
            );
        };

        let first = match self.input.chars().next() {
            Some(first) => first,
            None => return Ok(None),
        };

        let start = self.start;
        let make_end = move |len| CodePoint::new_unchecked(
            start.row(), start.col() + len as u32
        );

        let (tok_type, (end, rest)) = if first.is_alphabetic() || first == '_' {
            let (ident, rest) = split_on_false(self.input, |c| c.is_alphanumeric() || c == '_');

            let end = make_end(ident.len());

            let get_tok_ty = get_token_ty_from_ident!(
                Let => "let",
                Mut => "mut",
                Match => "match",
                Loop => "loop",
                Break => "break",
                Continue => "continue",
                Return => "return",
                Type => "type",
            );

            (get_tok_ty(ident), (end, rest))
        } else if first.is_numeric() {
            let (first, rest) = split_on_false(self.input, |c| c.is_alphanumeric() || c == '_');
            if let Some('.') = rest.chars().next() {
                let (second, rest) =
                    split_on_false(&rest[1..], |c| c.is_alphanumeric() || c == '_');

                let lexeme = &self.input[..first.len() + 1 + second.len()];

                let end = make_end(lexeme.len());

                let real = match Real::new(lexeme.parse::<f64>()
                    .map_err(|err| Error {
                        err: ErrorType::InvalidFloat(Some(err)),
                        span: self.start.span(end),
                    })?) {
                    Some(real) => real,
                    None => Err(Error {
                        err: ErrorType::InvalidFloat(None),
                        span: self.start.span(end),
                    })?
                };

                (TokenType::Float(real), (end, rest))
            } else {
                let end = make_end(first.len());

                let int = first.parse::<u128>()
                    .map_err(|err| Error {
                        err: ErrorType::InvalidInt(err),
                        span: self.start.span(end),
                    })?;

                (TokenType::Integer(int), (end, rest))
            }
        } else {
            let tok_type = match first {
                '+' => TokenType::Symbol(Symbol::Add),
                '-' => TokenType::Symbol(Symbol::Sub),
                '*' => TokenType::Symbol(Symbol::Mul),
                '/' => TokenType::Symbol(Symbol::Div),
                '.' => TokenType::Symbol(Symbol::Dot),
                '=' => TokenType::Symbol(Symbol::Assign),
                ';' => TokenType::Symbol(Symbol::Semicolon),
                c => {
                    let end = make_end(c.len_utf8());

                    #[allow(clippy::try_err)]
                    Err(Error {
                        err: ErrorType::UnknownCharacter(c),
                        span: self.start.span(end),
                    })?
                }
            };
            
            let (_, rest) = self.input.split_at(first.len_utf8());
            let end = make_end(first.len_utf8());

            (tok_type, (end, rest))
        };

        self.input = rest;
        let start = end;

        Ok(Some(Token {
            tok_type,
            span: start.span(end),
        }))
    }
}
