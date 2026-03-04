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
    let mut config = Config::load_or_default();
    apply_args_to_config(&mut config, &args);

    // Auto-detect terminal capabilities if backend is "auto"
    if config.image_backend == "auto" {
        let capabilities = image_cache::detect_terminal_capabilities();
        config.image_backend = capabilities.backend.clone();
    }

    // Only use API image fetching when user explicitly configured it:
    //   image_backend must be a graphics backend (not "ascii" / "off")
    //   AND image_source must be "auto" (API mode)
    //   AND no custom_image already set by user
    let uses_graphics_backend = matches!(
        config.image_backend.as_str(),
        "kitty" | "sixel" | "chafa" | "w3m" | "iterm2"
    );
    let uses_api_source = config.image_source == "auto";

    if uses_graphics_backend && uses_api_source && config.custom_image.is_none() {
        // Use cached image from previous run (instant, no delay)
        if let Some(cached_img) = image_cache::get_available_image() {
            config.custom_image = Some(cached_img.to_string_lossy().to_string());
        }

        // Download a new image in background for next run (non-blocking, zero delay)
        image_cache::download_image_background();
    }

    // Gather system info
    let sys_info = SystemInfo::collect();

    if args.json {
        print_json(&sys_info);
        return;
    }

    // Render
    let display = Display::new(&config);
    display.render(&sys_info);
}

fn apply_args_to_config(config: &mut Config, args: &Args) {
    // Boolean flags: only set if explicitly passed
    if args.no_color || args.stdout {
        config.no_color = true;
    }
    if args.stdout {
        config.stdout = true;
        config.bold = false;
    }
    if let Some(bold) = args.bold {
        config.bold = bold;
    }
    if let Some(ref backend) = args.backend {
        config.image_backend = backend.clone();
    }
    if let Some(ref source) = args.source {
        config.image_source = source.clone();
    }
    if let Some(ref distro) = args.ascii_distro {
        config.ascii_distro = distro.clone();
    }
    if args.emoji {
        config.emoji_mode = true;
    }
    if args.nerd_font {
        config.nerd_font = true;
    }
    if args.custom_image.is_some() {
        config.custom_image = args.custom_image.clone();
    }
    if args.logo_only {
        config.logo_only = true;
    }
    if let Some(gap) = args.gap {
        config.gap = gap;
    }
    if let Some(ref sep) = args.separator {
        config.separator = sep.clone();
    }
    if let Some(start) = args.block_range_start {
        config.block_range.0 = start;
    }
    if let Some(end) = args.block_range_end {
        config.block_range.1 = end;
    }
    if let Some(width) = args.block_width {
        config.block_width = width;
    }
}

fn print_json(info: &SystemInfo) {
    let gpu_str = info.gpu.join(", ");
    println!("{{");
    let fields: Vec<(&str, &str)> = vec![
        ("os", &info.os),
        ("host", &info.host),
        ("kernel", &info.kernel),
        ("uptime", &info.uptime),
        ("packages", &info.packages),
        ("shell", &info.shell),
        ("resolution", &info.resolution),
        ("de", &info.de),
        ("wm", &info.wm),
        ("wm_theme", &info.wm_theme),
        ("theme", &info.theme),
        ("icons", &info.icons),
        ("terminal", &info.terminal),
        ("terminal_font", &info.term_font),
        ("cpu", &info.cpu),
        ("gpu", &gpu_str),
        ("memory", &info.memory),
        ("disk", &info.disk),
        ("battery", &info.battery),
        ("local_ip", &info.local_ip),
        ("public_ip", &info.public_ip),
        ("locale", &info.locale),
        ("song", &info.song),
        ("users", &info.users),
    ];
    for (i, (key, val)) in fields.iter().enumerate() {
        let comma = if i < fields.len() - 1 { "," } else { "" };
        let escaped = val.replace('\\', "\\\\").replace('"', "\\\"");
        println!("  \"{}\": \"{}\"{}", key, escaped, comma);
    }
    println!("}}");
}

/// Interactive image selector using fzf
fn select_image_interactive() {
    use std::fs;
    use std::process::{Command, Stdio};

    let mut image_paths = Vec::new();

    // Scan common image directories
    let search_dirs = vec![
        dirs::download_dir(),
        dirs::picture_dir(),
        dirs::home_dir().map(|h| h.join("Desktop")),
    ];

    for dir in search_dirs.into_iter().flatten() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(
                        ext.to_lowercase().as_str(),
                        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp"
                    ) {
                        if let Some(path_str) = path.to_str() {
                            image_paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    if image_paths.is_empty() {
        println!("No images found in common directories (Downloads, Pictures, Desktop)");
        return;
    }

    // Try to use fzf if available
    let fzf_input = image_paths.join("\n");

    match Command::new("fzf")
        .arg("--preview")
        .arg("file {}")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = std::io::Write::write_all(&mut stdin, fzf_input.as_bytes());
            }

            if let Ok(output) = child.wait_with_output() {
                let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !selected.is_empty() {
                    update_config_image(&selected);
                }
            }
        }
        Err(_) => {
            // Fallback: show a numbered menu if fzf not available
            println!("Available images:");
            for (i, path) in image_paths.iter().enumerate() {
                println!("{}: {}", i + 1, path);
            }
            println!("fzf not found. Please install fzf for interactive selection: https://github.com/junegunn/fzf");
        }
    }
}

/// Update config file with selected image
fn update_config_image(image_path: &str) {
    use std::fs;
    let config_path = Config::config_path();

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            let updated = if content.contains("custom_image") {
                // Simple replacement: find and replace the custom_image line
                let lines: Vec<&str> = content.lines().collect();
                let new_lines: Vec<String> = lines
                    .iter()
                    .map(|line| {
                        if line.starts_with("custom_image") {
                            format!(r#"custom_image = "{}""#, image_path)
                        } else {
                            line.to_string()
                        }
                    })
                    .collect();
                new_lines.join("\n") + "\n"
            } else {
                format!("{}\ncustom_image = \"{}\"\n", content, image_path)
            };

            if let Err(e) = fs::write(&config_path, updated) {
                eprintln!("Error updating config: {}", e);
            } else {
                println!("✓ Config updated with image: {}", image_path);
                println!("  Run 'fetch' to see the selected image");
            }
        }
        Err(e) => {
            eprintln!("Error reading config: {}", e);
        }
    }
}

/// Run fetchx as a daemon, updating status file every 10 seconds
fn run_daemon() {
    use std::fs;
    use std::thread;
    use std::time::Duration;

    let status_file = dirs::runtime_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("fetchx-status.json");

    println!("Starting fetchx daemon...");
    println!("Status file: {}", status_file.display());

    loop {
        // Gather system info
        let sys_info = SystemInfo::collect();

        // Create JSON status
        let status = serde_json::json!({
            "hostname": &sys_info.hostname,
            "os": &sys_info.os,
            "cpu": &sys_info.cpu,
            "memory": &sys_info.memory,
            "disk": &sys_info.disk,
            "uptime": &sys_info.uptime,
            "shell": &sys_info.shell,
            "terminal": &sys_info.terminal,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });

        // Write to status file
        if let Ok(json_str) = serde_json::to_string_pretty(&status) {
            let _ = fs::write(&status_file, json_str);
        }

        // Update every 10 seconds
        thread::sleep(Duration::from_secs(10));
    }
}

/// Show compact tray status (read from daemon status file)
fn show_tray_status() {
    use std::fs;

    let status_file = dirs::runtime_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("fetchx-status.json");

    match fs::read_to_string(&status_file) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(status) => {
                    // Compact format for tray/widgets
                    let cpu = status.get("cpu").and_then(|v| v.as_str()).unwrap_or("N/A");
                    let mem = status
                        .get("memory")
                        .and_then(|v| v.as_str())
                        .unwrap_or("N/A");
                    let host = status
                        .get("hostname")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");

                    // Format: Host | CPU: 45% | RAM: 8.2GB/16GB
                    println!("{} | CPU: {} | RAM: {}", host, cpu, mem);
                }
                Err(_) => {
                    eprintln!("Invalid status file format. Start daemon with: fetchx --daemon");
                }
            }
        }
        Err(_) => {
            eprintln!("Status file not found. Start daemon with: fetchx --daemon");
        }
    }
}

/// Initialize config with API image fetching (one-command setup)
fn init_api_image_config() {
    use std::fs;

    let config_path = Config::config_path();

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            let updated = if content.contains("image_backend") && content.contains("image_source") {
                // Replace existing config
                let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

                lines = lines
                    .iter()
                    .map(|line| {
                        if line.starts_with("image_backend") {
                            r#"image_backend = "auto""#.to_string()
                        } else if line.starts_with("image_source") {
                            r#"image_source = "auto""#.to_string()
                        } else {
                            line.clone()
                        }
                    })
                    .collect();

                lines.join("\n") + "\n"
            } else {
                // Add new entries
                format!(
                    "{}\n\n# API Image Fetching (Smart Caching)\nimage_backend = \"auto\"\nimage_source = \"auto\"\n",
                    content
                )
            };

            match fs::write(&config_path, updated) {
                Ok(_) => {
                    println!("✓ Config initialized with API image fetching!");
                    println!();
                    println!("Settings applied:");
                    println!("  • image_backend = \"auto\"    (terminal auto-detection)");
                    println!("  • image_source = \"auto\"     (API + smart 6-hour caching)");
                    println!();
                    println!("Next steps:");
                    println!("  1. Run 'fetch' to see API wallpaper");
                    println!("  2. Run 'fetch' again after a few seconds to see cached image");
                    println!("  3. First run downloads image in background without blocking");
                    println!();
                    println!("Cache location: ~/.cache/fetchx/current_image.png");
                }
                Err(e) => {
                    eprintln!("Error writing config: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!("Config file not found. Create it first:");
            eprintln!("  fetchx --create-config");
            eprintln!("  fetchx --init-api-image");
            std::process::exit(1);
        }
    }
}
