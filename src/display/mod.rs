//! Display module for FetchX — proper neofetch-style side-by-side rendering.
//!
//! ASCII art on the left, info lines on the right, properly aligned.
//! Uses {c1}-{c6} placeholder substitution, correct visible-width calculation,
//! and respects all config flags.

use crate::ascii;
use crate::colors::{self, ColorScheme, BOLD, RESET};
use crate::config::{ColorConfig, Config};
use crate::image_backend::{self, Backend};
use crate::system::SystemInfo;
use crate::utils::{pad_right, truncate_to_width, visible_width};

pub struct Display {
    config: Config,
}

impl Display {
    pub fn new(config: &Config) -> Self {
        Display {
            config: config.clone(),
        }
    }

    pub fn render(&self, sys_info: &SystemInfo) {
        let backend = Backend::from_str(&self.config.image_backend);

        // Resolve image path if using an image backend
        let image_path = if backend != Backend::Ascii && backend != Backend::Off {
            self.resolve_image_path()
        } else {
            None
        };

        // If we have an image backend + valid image, use image mode
        if let Some(ref img_path) = image_path {
            if backend != Backend::Ascii && backend != Backend::Off {
                self.render_with_image(sys_info, &backend, img_path);
                return;
            }
        }

        // Fall through to ASCII art mode
        self.render_ascii(sys_info);
    }

    /// Resolve which image to display based on config.
    fn resolve_image_path(&self) -> Option<String> {
        // Priority: custom_image > image_source path > None
        if let Some(ref path) = self.config.custom_image {
            if std::path::Path::new(path).exists() {
                return Some(path.clone());
            } else {
                eprintln!("Warning: custom_image '{}' not found", path);
            }
        }
        let src = &self.config.image_source;
        if src != "auto" && src != "ascii" && src != "wallpaper" {
            // Treat as a file path
            if std::path::Path::new(src).exists() {
                return Some(src.clone());
            }
        }
        None
    }

    /// Render with an image on the left side.
    fn render_with_image(&self, sys_info: &SystemInfo, backend: &Backend, image_path: &str) {
        // Build color scheme for info text
        let distro_id = if self.config.ascii_distro == "auto" {
            &sys_info.distro_id
        } else {
            &self.config.ascii_distro
        };
        let scheme = self.build_color_scheme(distro_id);
        let info_lines = self.build_info_lines(sys_info, &scheme);

        // Determine size for the image
        let max_cols = self.parse_image_size_cols().unwrap_or(35);
        let max_rows = info_lines.len().max(20);

        match backend {
            Backend::Kitty => {
                // Kitty: render image directly via escape codes, then print info beside it
                match image_backend::display_kitty(image_path, max_cols, max_rows) {
                    Ok((img_cols, img_rows)) => {
                        let gap = self.config.gap;
                        let reset = if self.config.no_color { "" } else { RESET };
                        let total = img_rows.max(info_lines.len());
                        let term_w = terminal_width();
                        let right_avail = term_w.saturating_sub(img_cols + gap);

                        // Move cursor up by img_rows to start printing info beside image
                        print!("\x1b[{}A", img_rows);

                        for i in 0..total {
                            // Move to the right of the image
                            print!("\x1b[{}C", img_cols + gap);
                            if i < info_lines.len() {
                                let truncated = if right_avail > 0 {
                                    truncate_to_width(&info_lines[i], right_avail)
                                } else {
                                    info_lines[i].clone()
                                };
                                println!("{}{}", truncated, reset);
                            } else {
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: kitty image failed: {}", e);
                        self.render_ascii(sys_info);
                    }
                }
            }
            Backend::Chafa => {
                // Chafa: get text lines and use them like ASCII art
                match image_backend::display_chafa(image_path, max_cols, max_rows) {
                    Ok(img_lines) => {
                        self.render_side_by_side(&img_lines, &info_lines);
                    }
                    Err(e) => {
                        eprintln!("Warning: chafa failed: {}", e);
                        self.render_ascii(sys_info);
                    }
                }
            }
            Backend::Sixel => {
                match image_backend::display_sixel(image_path, max_cols, max_rows) {
                    Ok(()) => {
                        // Sixel prints inline; just print info lines below
                        let reset = if self.config.no_color { "" } else { RESET };
                        for line in &info_lines {
                            println!("{}{}", line, reset);
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: sixel failed: {}", e);
                        self.render_ascii(sys_info);
                    }
                }
            }
            Backend::W3m => {
                match image_backend::display_w3m(image_path, max_cols, max_rows) {
                    Ok(()) => {
                        // w3m renders image at position; print info offset to the right
                        let gap = self.config.gap;
                        let offset = max_cols + gap;
                        let reset = if self.config.no_color { "" } else { RESET };
                        let term_w = terminal_width();
                        let right_avail = term_w.saturating_sub(offset);
                        // Move up to start of image area
                        print!("\x1b[{}A", max_rows);
                        for i in 0..max_rows.max(info_lines.len()) {
                            print!("\x1b[{}C", offset);
                            if i < info_lines.len() {
                                let truncated = if right_avail > 0 {
                                    truncate_to_width(&info_lines[i], right_avail)
                                } else {
                                    info_lines[i].clone()
                                };
                                println!("{}{}", truncated, reset);
                            } else {
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: w3m image failed: {}", e);
                        self.render_ascii(sys_info);
                    }
                }
            }
            Backend::Iterm2 => {
                match image_backend::display_iterm2(image_path, max_cols, max_rows) {
                    Ok(()) => {
                        // iTerm2 inline image works like kitty; print info beside it
                        let gap = self.config.gap;
                        let reset = if self.config.no_color { "" } else { RESET };
                        let total = max_rows.max(info_lines.len());
                        let term_w = terminal_width();
                        let right_avail = term_w.saturating_sub(max_cols + gap);

                        print!("\x1b[{}A", max_rows);
                        for i in 0..total {
                            print!("\x1b[{}C", max_cols + gap);
                            if i < info_lines.len() {
                                let truncated = if right_avail > 0 {
                                    truncate_to_width(&info_lines[i], right_avail)
                                } else {
                                    info_lines[i].clone()
                                };
                                println!("{}{}", truncated, reset);
                            } else {
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: iTerm2 image failed: {}", e);
                        self.render_ascii(sys_info);
                    }
                }
            }
            _ => self.render_ascii(sys_info),
        }
    }

    /// Parse image_size config into column count.
    fn parse_image_size_cols(&self) -> Option<usize> {
        let s = &self.config.image_size;
        if s == "auto" || s == "none" {
            return None;
        }
        if let Some(stripped) = s.strip_suffix("px") {
            // Convert pixels to approximate columns (8px per col)
            stripped.parse::<usize>().ok().map(|px| px / 8)
        } else if let Some(stripped) = s.strip_suffix('%') {
            // Percentage of terminal width
            let term_cols = terminal_width();
            stripped
                .parse::<usize>()
                .ok()
                .map(|pct| term_cols * pct / 100)
        } else {
            s.parse::<usize>().ok()
        }
    }

    /// Standard ASCII art rendering path.
    fn render_ascii(&self, sys_info: &SystemInfo) {
        // Resolve which distro art to use
        let distro_id = if self.config.ascii_distro == "auto" {
            &sys_info.distro_id
        } else {
            &self.config.ascii_distro
        };

        let scheme = self.build_color_scheme(distro_id);

        // Get ASCII art lines with color placeholders resolved
        let ascii_lines = self.build_ascii_lines(distro_id, &scheme);

        if self.config.logo_only {
            let reset = if self.config.no_color { "" } else { RESET };
            for line in &ascii_lines {
                println!("{}{}", line, reset);
            }
            return;
        }

        // Build info lines
        let info_lines = self.build_info_lines(sys_info, &scheme);

        self.render_side_by_side(&ascii_lines, &info_lines);
    }

    /// Build the color scheme based on config and distro.
    fn build_color_scheme(&self, distro_id: &str) -> ColorScheme {
        // Determine ASCII art colors to use
        let art_color_nums: Vec<u8> = match &self.config.ascii_colors {
            ColorConfig::Numbers(nums) if !nums.is_empty() => nums.clone(),
            _ => {
                let art = ascii::get_ascii(distro_id);
                art.colors.clone()
            }
        };

        let art_bold = self.config.ascii_bold && self.config.bold;
        if self.config.no_color {
            ColorScheme::plain()
        } else {
            let mut cs = ColorScheme::from_colors(&art_color_nums, art_bold);

            // Override text colors if user specified custom colors
            match &self.config.colors {
                ColorConfig::Numbers(nums) if !nums.is_empty() => {
                    if let Some(&n) = nums.first() {
                        cs.title = colors::color(n);
                    }
                    if let Some(&n) = nums.get(1) {
                        cs.at = colors::color(n);
                    }
                    if let Some(&n) = nums.get(2) {
                        cs.underline = colors::color(n);
                    }
                    if let Some(&n) = nums.get(3) {
                        cs.subtitle = colors::color(n);
                    }
                    if let Some(&n) = nums.get(4) {
                        cs.colon = colors::color(n);
                    }
                    if let Some(&n) = nums.get(5) {
                        cs.info = colors::color(n);
                    }
                }
                _ => {}
            }

            cs
        }
    }

    /// Render left-side lines and right-side info lines side-by-side.
    fn render_side_by_side(&self, left_lines: &[String], right_lines: &[String]) {
        // Calculate max visible width of ASCII art (for padding calculations)
        let max_left_width = left_lines
            .iter()
            .map(|l| visible_width(l))
            .max()
            .unwrap_or(0);

        let gap = self.config.gap;
        let term_w = terminal_width();
        let right_avail = term_w.saturating_sub(max_left_width + gap);
        let reset = if self.config.no_color { "" } else { RESET };

        // Simple approach: just pad ASCII lines and print with info alongside
        // No cursor repositioning - just proper column alignment

        let total_rows = left_lines.len().max(right_lines.len());

        for i in 0..total_rows {
            // Left column: ASCII art (padded to width)
            let left_part = if i < left_lines.len() {
                pad_right(&left_lines[i], max_left_width)
            } else {
                " ".repeat(max_left_width)
            };

            // Gap between columns
            let gap_str = " ".repeat(gap);

            // Right column: info lines (truncated to fit)
            if i < right_lines.len() {
                let truncated = if right_avail > 0 {
                    truncate_to_width(&right_lines[i], right_avail)
                } else {
                    right_lines[i].clone()
                };
                println!("{}{}{}{}{}", left_part, reset, gap_str, truncated, reset);
            } else {
                println!("{}{}", left_part, reset);
            }
        }
    }

    /// Build ASCII art lines with {c1}-{c6} placeholders replaced by actual ANSI codes.
    fn build_ascii_lines(&self, distro_id: &str, scheme: &ColorScheme) -> Vec<String> {
        if self.config.emoji_mode {
            return ascii::get_emoji_art(distro_id)
                .into_iter()
                .map(|s| s.to_string())
                .collect();
        }

        // If user specified a custom ASCII file, load it from disk
        let raw_lines: Vec<String> = if let Some(ref path) = self.config.ascii_file {
            match std::fs::read_to_string(path) {
                Ok(contents) => contents.lines().map(|l| l.to_string()).collect(),
                Err(e) => {
                    eprintln!("Warning: could not read ascii_file '{}': {}", path, e);
                    let art = ascii::get_ascii(distro_id);
                    art.lines.iter().map(|s| s.to_string()).collect()
                }
            }
        } else {
            let art = ascii::get_ascii(distro_id);
            art.lines.iter().map(|s| s.to_string()).collect()
        };

        raw_lines
            .iter()
            .map(|line| {
                let mut s = line.to_string();
                for (i, color_seq) in scheme.c.iter().enumerate() {
                    let placeholder = format!("{{c{}}}", i + 1);
                    s = s.replace(&placeholder, color_seq);
                }
                s
            })
            .collect()
    }

    /// Build all info lines (title, underline, fields, color blocks).
    fn build_info_lines(&self, sys: &SystemInfo, scheme: &ColorScheme) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let sep = &self.config.separator;

        // Title: user@host
        if self.config.show_title {
            if self.config.no_color {
                lines.push(format!("{}@{}", sys.username, sys.hostname));
            } else {
                let title_line = format!(
                    "{}{}{}{}{}@{}{}{}{}",
                    scheme.title,
                    BOLD,
                    sys.username,
                    RESET,
                    scheme.at,
                    scheme.title,
                    BOLD,
                    sys.hostname,
                    RESET,
                );
                lines.push(title_line);
            }
        }

        // Underline (uses configurable underline_char, defaults to '-' like neofetch)
        if self.config.show_underline && self.config.show_title {
            let title_visible_len = sys.username.len() + 1 + sys.hostname.len(); // user@host
            let dashes = self.config.underline_char.repeat(title_visible_len);
            if self.config.no_color {
                lines.push(dashes);
            } else {
                lines.push(format!("{}{}{}", scheme.underline, dashes, RESET));
            }
        }

        // Info fields — each one: colored label + separator + value
        let nf = self.config.nerd_font;

        let field = |label: &str, icon: &str, value: &str| -> String {
            if self.config.no_color {
                let prefix = if nf {
                    format!("{} {}", icon, label)
                } else {
                    label.to_string()
                };
                format!("{}{}{}", prefix, sep, value)
            } else {
                let prefix = if nf {
                    format!("{} {}{}{}", icon, scheme.subtitle, BOLD, label)
                } else {
                    format!("{}{}{}", scheme.subtitle, BOLD, label)
                };
                format!(
