#[derive(Clone, Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    ParenLeft,
    ParenRight,
    Number(i32),
    EndOfFile,
}

#[derive(Debug)]
pub enum LexError {
    InvalidToken(char),
    InvalidInteger(std::num::ParseIntError),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(c) => write!(f, "invalid token '{c}'"),
            Self::InvalidInteger(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for LexError {}

pub struct Lexer<I: Iterator> {
    chars: std::iter::Peekable<I>,
    dispensed_eof: bool,
}

impl<'a> Lexer<std::str::Chars<'a>> {
    pub fn from_source(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            dispensed_eof: false,
        }
    }
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek()
            && c.is_whitespace()
        {
            self.chars.next();
        }
    }

    fn lex_number(&mut self, first: char) -> <Self as Iterator>::Item {
        let mut digits = first.to_string();
        while let Some(d) = self.chars.peek()
            && d.is_ascii_digit()
        {
            digits.push(*d);
            self.chars.next();
        }

        return match digits.parse::<i32>() {
            Ok(value) => Ok(Token::Number(value)),
            Err(error) => Err(LexError::InvalidInteger(error)),
        };
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.chars.next() {
            Some('+') => Some(Ok(Token::Plus)),
            Some('-') => Some(Ok(Token::Minus)),
            Some('*') => Some(Ok(Token::Star)),
            Some('/') => Some(Ok(Token::Slash)),
            Some('(') => Some(Ok(Token::ParenLeft)),
            Some(')') => Some(Ok(Token::ParenRight)),
            Some(other) if other.is_ascii_digit() => Some(self.lex_number(other)),
            Some(other) => Some(Err(LexError::InvalidToken(other))),

            // We only want to emit the EndOfFile token once per stream, right at the end.
            None if self.dispensed_eof => None,
            None => {
                self.dispensed_eof = true;
                Some(Ok(Token::EndOfFile))
            }
        }
    }
}
