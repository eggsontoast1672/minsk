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
