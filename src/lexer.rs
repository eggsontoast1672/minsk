#[derive(Clone, Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    ParenLeft,
    ParenRight,
    Number(i32),
    EndOfFile,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::ParenLeft => write!(f, "("),
            Self::ParenRight => write!(f, ")"),
            Self::Number(n) => write!(f, "{n}"),
            Self::EndOfFile => write!(f, "<eof>"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

impl Token {
    pub const fn new(kind: TokenKind, length: usize) -> Self {
        Self { kind, length }
    }
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

        match digits.parse::<i32>() {
            Ok(value) => Ok(Token::new(TokenKind::Number(value), digits.len())),
            Err(error) => Err(LexError::InvalidInteger(error)),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.chars.next() {
            Some('+') => Some(Ok(Token::new(TokenKind::Plus, 1))),
            Some('-') => Some(Ok(Token::new(TokenKind::Minus, 1))),
            Some('*') => Some(Ok(Token::new(TokenKind::Star, 1))),
            Some('/') => Some(Ok(Token::new(TokenKind::Slash, 1))),
            Some('(') => Some(Ok(Token::new(TokenKind::ParenLeft, 1))),
            Some(')') => Some(Ok(Token::new(TokenKind::ParenRight, 1))),
            Some(first) if first.is_ascii_digit() => Some(self.lex_number(first)),
            Some(other) => Some(Err(LexError::InvalidToken(other))),

            // We only want to emit the EndOfFile token once per stream, right at the end.
            None if self.dispensed_eof => None,
            None => {
                self.dispensed_eof = true;
                Some(Ok(Token::new(TokenKind::EndOfFile, 0)))
            }
        }
    }
}
