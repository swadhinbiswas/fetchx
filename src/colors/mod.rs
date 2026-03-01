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
}

impl ColorScheme {
    /// Create a color scheme from a list of color numbers (like neofetch's set_colors).
    /// First color is primary, second is secondary, etc.
    pub fn from_colors(nums: &[u8], bold: bool) -> Self {
        let mut c = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        let bold_str = if bold { BOLD } else { "" };
        for (i, &n) in nums.iter().take(6).enumerate() {
            c[i] = format!("{}{}", color(n), bold_str);
        }
        // Fill remaining with first color
        if !nums.is_empty() {
            let first = c[0].clone();
            for item in c.iter_mut() {
                if item.is_empty() {
                    item.clone_from(&first);
                }
            }
        }

        // Text colors derived from distro colors (neofetch "distro" mode)
        // Mirrors neofetch's set_text_colors(): title=color($1), subtitle=color($2),
        // at/underline/colon/info all reset. Special cases for colors 7 and 8.
        let c1 = nums.first().copied().unwrap_or(1);
        let c2 = nums.get(1).copied().unwrap_or(7);

        // If the first color is 8 or 7, title resets to default fg
        let title = if c1 == 8 || c1 == 7 {
            RESET.to_string()
        } else {
            color(c1)
        };

        // If the second color is 7 or 8, subtitle uses the first color instead
        let subtitle = if c2 == 7 || c2 == 8 {
            if c1 == 8 || c1 == 7 {
                RESET.to_string()
            } else {
                color(c1)
            }
        } else {
            color(c2)
        };

        Self {
            c,
            title,
            at: RESET.to_string(),
            underline: RESET.to_string(),
            subtitle,
            colon: RESET.to_string(),
            info: RESET.to_string(),
            ascii_bold: bold,
        }
    }

    /// Create a no-color scheme (for --stdout / --no-color).
    pub fn plain() -> Self {
        Self {
            c: Default::default(),
            title: String::new(),
            at: String::new(),
            underline: String::new(),
            subtitle: String::new(),
            colon: String::new(),
            info: String::new(),
            ascii_bold: false,
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::from_colors(&[6, 6, 7, 1], true)
    }
}

/// Generate color blocks matching neofetch's get_cols().
/// Colors 0-7 use standard ANSI (fg+bg), colors 8+ use 256-color mode.
/// Blocks are split into two rows: standard (0-7) and extended (8+).
pub fn color_blocks(start: u8, end: u8, width: usize) -> Vec<String> {
    let block = "\u{2588}".repeat(width); // █ repeated
