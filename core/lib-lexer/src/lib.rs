
use lib_lexer_types::{
    Result,
    CodePoint,
    Token,
    TokenType,
    Symbol,
    Keyword
};

macro_rules! get_token_ty_from_ident {
    ($($kw:ident => $value:literal),* $(,)?) => {
        |ident| {
            #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
            'here: loop {
                let unreachable: Keyword = break TokenType::Keyword(match ident {
                    $($value => Keyword::$kw,)*
                    _ => break 'here TokenType::Identifier
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
    start: CodePoint
}

fn split_on_false<F: FnMut(char) -> bool>(s: &str, mut f: F) -> (&str, &str) {
    let len = s.chars()
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

        let (tok_type, (lexeme, rest)) = if first.is_whitespace() {
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

            let end = CodePoint::new_unchecked(
                self.start.row() + rows,
                if rows == 0 { self.start.col() + lexeme.len() as u32 }
                else { cols }
            );

            let start = std::mem::replace(&mut self.start, end);

            return Ok(Some(Token {
                lexeme,
                tok_type: TokenType::Whitespace,
                span: start.span(end)
            }))
        } else if first.is_alphabetic() || first == '_' {
            let (ident, rest) = split_on_false(self.input, |c| c.is_alphanumeric() || c == '_');

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

            (get_tok_ty(ident), (ident, rest))
        } else if first.is_numeric() {
            let (first, rest) = split_on_false(self.input, |c| c.is_alphanumeric() || c == '_');
            if let Some('.') = rest.chars().next() {
                let (second, rest) = split_on_false(&rest[1..], |c| c.is_alphanumeric() || c == '_');

                let lexeme = &self.input[..first.len() + 1 + second.len()];

                (TokenType::Float, (lexeme, rest))
            } else {
                (TokenType::Integer, (first, rest))
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
                c => todo!("{}", c)
            };
            
            (tok_type, self.input.split_at(first.len_utf8()))
        };

        self.input = rest;

        let end = CodePoint::new_unchecked(
            self.start.row(),
            self.start.col() + lexeme.len() as u32
        );

        let start = std::mem::replace(&mut self.start, end);

        Ok(Some(Token {
            lexeme,
            tok_type,
            span: start.span(end)
        }))
    }
}
