//! Utility functions for FetchX

use std::process::Command;

/// Run a command and return its stdout trimmed, or empty string on failure.
pub fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Run a command and return its stdout even on non-zero exit.
#[allow(dead_code)]
pub fn run_cmd_lossy(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Read a file to string, trimmed. Returns empty string on failure.
pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Read first line of a file, trimmed.
pub fn read_first_line(path: &str) -> String {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.lines().next().map(|l| l.trim().to_string()))
        .unwrap_or_default()
}

/// Check if a command exists on the system.
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get environment variable or empty string.
pub fn env_or(key: &str) -> String {
    std::env::var(key).unwrap_or_default()
}

/// Strip ANSI escape sequences from a string for width calculation.
/// Handles CSI (\x1b[...), OSC (\x1b]...), APC (\x1b_...) and simple \x1b<char> sequences.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            match chars.peek() {
                Some(&'[') => {
                    // CSI sequence: \x1b[ ... <letter>
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some(&']') | Some(&'_') | Some(&'^') => {
                    // OSC / APC / PM: terminated by ST (\x1b\\ or BEL)
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '\x07' {
                            break; // BEL terminator
                        }
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') {
                                chars.next(); // consume '\\'
                                break;
                            }
                        }
                    }
                }
                Some(&c) if c.is_ascii_alphabetic() || c == '(' || c == ')' => {
                    // Simple two-char escape like \x1bM, \x1b(B etc.
                    chars.next();
                    if c == '(' || c == ')' {
                        chars.next(); // consume charset designator
                    }
                }
                _ => {} // Lone ESC, skip it
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Calculate the visible width of a string (strip ANSI, use unicode-width).
pub fn visible_width(s: &str) -> usize {
    use unicode_width::UnicodeWidthStr;
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

/// Pad a string to a given visible width with spaces on the right.
pub fn pad_right(s: &str, target_width: usize) -> String {
    let current = visible_width(s);
    if current >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - current))
    }
}

/// Truncate a string (which may contain ANSI escapes) so its visible width
/// does not exceed `max_width`. Preserves ANSI sequences but cuts visible chars.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    let mut result = String::with_capacity(s.len());
    let mut vis_w: usize = 0;
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Pass through the entire escape sequence
            result.push(ch);
            match chars.peek() {
                Some(&'[') => {
                    result.push(chars.next().unwrap());
                    while let Some(&c) = chars.peek() {
                        result.push(chars.next().unwrap());
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some(&']') | Some(&'_') | Some(&'^') => {
