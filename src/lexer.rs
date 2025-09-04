/// The types of tokens in a Minsk program.
#[derive(Clone, Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    ParenLeft,
    ParenRight,
    Number(String),
    EndOfFile,
    Invalid(char),
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
            Self::Invalid(c) => write!(f, "<invalid '{c}'>"),
        }
    }
}

/// The smallest unit of lexical meaning.
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub const fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

/// The Minsk lexer.
pub struct Lexer<I: Iterator> {
    chars: std::iter::Peekable<I>,
    offset: usize,
    dispensed_eof: bool,
}

impl<'a> Lexer<std::str::Chars<'a>> {
    pub fn from_source(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            offset: 0,
            dispensed_eof: false,
        }
    }
}

impl<I: Iterator<Item = char>> Lexer<I> {
    /// Convenience function to advance the underlying iterator and update offset.
    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.offset += c.len_utf8();
        Some(c)
    }

    /// Convenience function to advance the underlying iterator only if the predicate succeeds,
    /// while also keeping the offset up to date.
    fn next_char_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let c = self.chars.next_if(func)?;
        self.offset += c.len_utf8();
        Some(c)
    }

    /// Skips all whitespace at the beginning of the stream.
    ///
    /// Since whitespace is not significant for any reason, we have no reason to process it.
    fn skip_whitespace(&mut self) {
        while self.next_char_if(|c| c.is_whitespace()).is_some() {}
    }

    /// Get the next number token from the character stream.
    ///
    /// This function is to be called after the first character of the number has already been
    /// consumed and passed as the `first` parameter. Additionally, the byte offset of the first
    /// digit is to be passed as `start`.
    fn lex_number(&mut self, first: char, start: usize) -> <Self as Iterator>::Item {
        let mut digits = first.to_string();
        while let Some(digit) = self.chars.next_if(|c| c.is_ascii_digit()) {
            digits.push(digit);
        }

        // This might not constitute a valid integer since it could be too long, however we will
        // leave it to the parser to report this error.
        Token::new(TokenKind::Number(digits), start, self.offset)
    }
}

impl<I> Clone for Lexer<I>
where
    I: Clone + Iterator,
    <I as Iterator>::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            chars: self.chars.clone(),
            offset: self.offset,
            dispensed_eof: self.dispensed_eof,
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let start = self.offset;
        match self.next_char() {
            Some('+') => Some(Token::new(TokenKind::Plus, start, self.offset)),
            Some('-') => Some(Token::new(TokenKind::Minus, start, self.offset)),
            Some('*') => Some(Token::new(TokenKind::Star, start, self.offset)),
            Some('/') => Some(Token::new(TokenKind::Slash, start, self.offset)),
            Some('(') => Some(Token::new(TokenKind::ParenLeft, start, self.offset)),
            Some(')') => Some(Token::new(TokenKind::ParenRight, start, self.offset)),

            Some(first) if first.is_ascii_digit() => Some(self.lex_number(first, start)),
            Some(other) => Some(Token::new(TokenKind::Invalid(other), start, self.offset)),

            // We only want to emit the EndOfFile token once per stream, right at the end.
            None if self.dispensed_eof => None,
            None => {
                self.dispensed_eof = true;
                Some(Token::new(TokenKind::EndOfFile, start, start))
            }
        }
    }
}
