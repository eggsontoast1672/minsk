/// Specialized foreground colors for terminal text.
///
/// There are two kinds of text, normal and colored. Normal text is white with standard weight.
/// Colored text is always bold and can have some color, the options for which are defined by this
/// enum.
#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Green,
    White,
}

impl Color {
    /// Convert a color to an ANSI escape code.
    ///
    /// This function is useful for changing the text color via writing directly to the stream.
    /// Since this is not portable, it is recommended to use a function like [`set_text_color`].
    fn as_ansi_escape_code(&self) -> &'static str {
        match self {
            Self::Red => "\x1b[1;31m",
            Self::Green => "\x1b[1;32m",
            Self::White => "\x1b[1;37m",
        }
    }
}

#[cfg(not(windows))]
pub fn set_text_color(color: Color) {
    let code = color.as_ansi_escape_code();
    print!("{code}");
}

#[cfg(not(windows))]
pub fn reset_text_color() {
    print!("\x1b[0m");
}
