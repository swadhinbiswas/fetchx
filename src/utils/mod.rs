use unicode_width::UnicodeWidthStr;

/// Get the display width of a string (Unicode-aware)
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}
