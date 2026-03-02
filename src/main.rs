//! FetchX - A fast, feature-rich system information tool written in Rust.
//!
//! An exact clone of neofetch with extra features: emoji support, image display,
//! Nerd Font icons, and blazing fast performance.

mod ascii;
mod colors;
mod config;
mod display;
mod image_backend;
mod image_cache;
mod system;
mod utils;

use clap::Parser;
use config::Config;
use display::Display;
use system::SystemInfo;

/// FetchX - Fast system information tool with emoji & image support
#[derive(Parser, Debug)]
#[command(name = "fetchx")]
#[command(author = "FetchX Contributors")]
#[command(version = "0.2.0")]
#[command(about = "A fast, modern system information tool — neofetch clone in Rust", long_about = None)]
struct Args {
    /// Disable colors in output
    #[arg(long)]
    no_color: bool,

    /// Enable/disable bold text
    #[arg(long)]
    bold: Option<bool>,

    /// Image backend: ascii, kitty, sixel, chafa, off
    #[arg(short, long, value_name = "BACKEND")]
    backend: Option<String>,

    /// Image source: auto, ascii, wallpaper, or /path/to/image
    #[arg(short, long, value_name = "SOURCE")]
    source: Option<String>,

    /// Which distro's ASCII art to display
    #[arg(long, value_name = "DISTRO")]
    ascii_distro: Option<String>,

    /// Use emoji mode instead of ASCII art
    #[arg(long)]
    emoji: bool,

    /// Use Nerd Font icons for info labels
    #[arg(long)]
    nerd_font: bool,

    /// Path to custom image to display
    #[arg(short, long, value_name = "PATH")]
    custom_image: Option<String>,

    /// Only show the logo/ASCII art, no system info
    #[arg(short, long)]
    logo_only: bool,

    /// Gap between logo and info text
    #[arg(long, value_name = "NUM")]
    gap: Option<usize>,

    /// Separator between label and value
    #[arg(long, value_name = "SEP")]
    separator: Option<String>,

    /// Disable all formatting (for piping)
    #[arg(long)]
    stdout: bool,

    /// Show the config file path and exit
    #[arg(long)]
    show_config: bool,

    /// Print the default config to stdout and exit
    #[arg(long)]
