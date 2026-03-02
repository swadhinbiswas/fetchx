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
    print_config: bool,

    /// Create the default config file if it doesn't exist and exit
    #[arg(long)]
    create_config: bool,

    /// Print output as JSON
    #[arg(long)]
    json: bool,

    /// Color block range start (0-15)
    #[arg(long)]
    block_range_start: Option<u8>,

    /// Color block range end (0-15)
    #[arg(long)]
    block_range_end: Option<u8>,

    /// Width of color blocks
    #[arg(long)]
    block_width: Option<usize>,

    /// Interactive image selector using fzf
    #[arg(long)]
    select_image: bool,

    /// Run as daemon (updates status file every 10 seconds)
    #[arg(long)]
    daemon: bool,

    /// Show tray status (compact output for system tray/widget)
    #[arg(long)]
    tray_status: bool,

    /// Initialize config with API image fetching (one-command setup)
    #[arg(long)]
    init_api_image: bool,

    /// Internal: background download helper (not for user use)
    #[arg(long, hide = true)]
    bg_download: bool,
}

fn main() {
    let args = Args::parse();

    // Handle special flags
    if args.show_config {
        let path = Config::config_path();
        println!("Config file: {}", path.display());
        return;
    }

    if args.print_config {
        println!("{}", Config::example_config());
        return;
    }

    if args.create_config {
        match Config::create_default_config() {
            Ok(path) => {
                println!("Config file created at: {}", path.display());
            }
            Err(e) => {
                eprintln!("Error creating config: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    if args.bg_download {
        // Internal: called as a detached child process to download image
        if let Err(e) = image_cache::download_image_from_api() {
            eprintln!("fetchx bg-download error: {}", e);
        }
        return;
    }

    if args.init_api_image {
        init_api_image_config();
        return;
    }

    if args.select_image {
        select_image_interactive();
        return;
    }

    if args.daemon {
        run_daemon();
        return;
    }

    if args.tray_status {
        show_tray_status();
        return;
    }

    // Load config, then override with CLI args
