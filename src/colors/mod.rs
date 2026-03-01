//! Color system for FetchX — mirrors neofetch's color() / set_colors() logic.
//!
//! Colors 0-7 map to ANSI 30-37.
//! Color 8+ maps to 256-color: \e[38;5;Nm
//! "fg" resets to default foreground.

/// ANSI reset sequence.
pub const RESET: &str = "\x1b[0m";
/// ANSI bold sequence.
pub const BOLD: &str = "\x1b[1m";

/// Generate an ANSI foreground color escape sequence from a neofetch color number.
/// 0-7 → standard ANSI (30-37). 8-255 → 256-color mode.
pub fn color(n: u8) -> String {
    match n {
        0..=7 => format!("{}\x1b[3{}m", RESET, n),
        _ => format!("\x1b[38;5;{}m", n),
    }
}

/// A set of up to 6 colors for ASCII art (c1-c6) plus text colors.
#[derive(Clone, Debug)]
pub struct ColorScheme {
    /// ASCII art colors c1-c6
    pub c: [String; 6],
    /// Title color (username@hostname)
    pub title: String,
    /// @ separator color
    pub at: String,
    /// Underline color
    pub underline: String,
    /// Subtitle / label color
    pub subtitle: String,
    /// Colon separator color
    pub colon: String,
    /// Info value color
    pub info: String,
    /// Whether bold is enabled for ASCII art
    #[allow(dead_code)]
    pub ascii_bold: bool,
