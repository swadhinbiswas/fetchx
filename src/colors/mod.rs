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
    let mut row1 = String::new(); // standard colors 0-7
    let mut row2 = String::new(); // extended colors 8+

    for i in start..=end {
        match i {
            0..=7 => {
                // Standard ANSI: set both fg and bg to same color
                row1.push_str(&format!("\x1b[3{}m\x1b[4{}m{}", i, i, block));
            }
            _ => {
                // 256-color mode: set both fg and bg
                row2.push_str(&format!("\x1b[38;5;{}m\x1b[48;5;{}m{}", i, i, block));
            }
        }
    }

    let mut lines = Vec::new();
    if !row1.is_empty() {
        row1.push_str(RESET);
        lines.push(row1);
    }
    if !row2.is_empty() {
        row2.push_str(RESET);
        lines.push(row2);
    }
    lines
}

/// Generate a progress bar for memory/disk usage.
/// `percent` is 0-100. `width` is the total bar width in chars.
/// Returns something like: `[████████░░░░░░░░] 50%`
pub fn progress_bar(percent: f64, width: usize, bar_color: &str) -> String {
    let pct = percent.clamp(0.0, 100.0);
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let bar = format!(
        "{}{}{}{}{}",
        bar_color,
        "█".repeat(filled),
        RESET,
        "░".repeat(empty),
        RESET,
    );
    format!("[{}] {:.0}%", bar, pct)
}

/// Apply a gradient effect to text using 256-color (or RGB) ANSI codes.
/// Interpolates between `start_color` (r,g,b) and `end_color` (r,g,b) across the text.
#[allow(dead_code)]
pub fn gradient_text(text: &str, start: (u8, u8, u8), end: (u8, u8, u8)) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let mut result = String::with_capacity(text.len() * 20);
    for (i, ch) in chars.iter().enumerate() {
        let t = if len > 1 {
            i as f64 / (len - 1) as f64
        } else {
            0.0
        };
        let r = (start.0 as f64 + (end.0 as f64 - start.0 as f64) * t) as u8;
        let g = (start.1 as f64 + (end.1 as f64 - start.1 as f64) * t) as u8;
        let b = (start.2 as f64 + (end.2 as f64 - start.2 as f64) * t) as u8;
        result.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
    }
    result.push_str(RESET);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_standard() {
        assert_eq!(color(0), "\x1b[0m\x1b[30m");
        assert_eq!(color(1), "\x1b[0m\x1b[31m");
        assert_eq!(color(7), "\x1b[0m\x1b[37m");
    }

    #[test]
    fn test_color_256() {
        assert_eq!(color(8), "\x1b[38;5;8m");
        assert_eq!(color(255), "\x1b[38;5;255m");
    }

    #[test]
    fn test_color_scheme_from_colors() {
        let scheme = ColorScheme::from_colors(&[6, 6, 7, 1], true);
        assert_eq!(scheme.c.len(), 6);
        // c1 should be cyan (6)
        assert!(scheme.c[0].contains("\x1b[36m"));
    }

    #[test]
    fn test_color_scheme_plain() {
        let scheme = ColorScheme::plain();
        assert!(scheme.title.is_empty());
        assert!(scheme.subtitle.is_empty());
    }

    #[test]
    fn test_color_blocks() {
        let blocks = color_blocks(0, 7, 3);
        assert!(!blocks.is_empty());
        assert!(blocks[0].contains("███"));
    }

    #[test]
    fn test_color_blocks_extended() {
        let blocks = color_blocks(0, 15, 3);
        assert!(blocks.len() >= 2); // standard + extended rows
    }

    #[test]
    fn test_progress_bar_zero() {
        let bar = progress_bar(0.0, 10, "\x1b[31m");
        assert!(bar.contains("0%"));
        assert!(bar.contains("░"));
    }

    #[test]
    fn test_progress_bar_full() {
        let bar = progress_bar(100.0, 10, "\x1b[32m");
        assert!(bar.contains("100%"));
        assert!(bar.contains("█"));
    }

    #[test]
    fn test_progress_bar_half() {
        let bar = progress_bar(50.0, 10, "\x1b[33m");
        assert!(bar.contains("50%"));
    }

    #[test]
    fn test_progress_bar_clamp() {
        let bar = progress_bar(150.0, 10, "");
        assert!(bar.contains("100%"));
        let bar2 = progress_bar(-50.0, 10, "");
        assert!(bar2.contains("0%"));
    }

    #[test]
    fn test_gradient_text() {
        let result = gradient_text("Hello", (255, 0, 0), (0, 0, 255));
        assert!(result.contains("\x1b[38;2;"));
        assert!(result.ends_with(RESET));
    }

    #[test]
    fn test_gradient_text_empty() {
        let result = gradient_text("", (255, 0, 0), (0, 0, 255));
        assert!(result.is_empty());
    }

    #[test]
    fn test_gradient_text_single_char() {
        let result = gradient_text("A", (255, 0, 0), (0, 0, 255));
        assert!(result.contains("A"));
    }
}
// fix: null-safe gradient
