//! Image backend module for FetchX
//!
//! Supports: kitty graphics protocol, sixel (via external tool), chafa, and "off".

use base64::Engine;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Image backend types
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    Ascii,
    Kitty,
    Sixel,
    Chafa,
    W3m,
    Iterm2,
    Off,
}

impl Backend {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "kitty" => Backend::Kitty,
            "sixel" => Backend::Sixel,
            "chafa" => Backend::Chafa,
            "w3m" | "w3mimgdisplay" => Backend::W3m,
            "iterm2" | "iterm" => Backend::Iterm2,
            "off" => Backend::Off,
            "auto" => Backend::Ascii, // Default: auto-detection is handled in main.rs
            _ => Backend::Ascii,
        }
    }

    /// Check if the terminal supports this backend.
    #[allow(dead_code)]
    pub fn is_supported(&self) -> bool {
        match self {
            Backend::Ascii | Backend::Off => true,
            Backend::Kitty => {
                // Kitty is supported if TERM contains "kitty" or KITTY_PID is set
                std::env::var("KITTY_PID").is_ok()
                    || std::env::var("TERM")
                        .unwrap_or_default()
                        .contains("kitty")
                    || std::env::var("TERM_PROGRAM")
                        .unwrap_or_default()
                        .contains("kitty")
            }
