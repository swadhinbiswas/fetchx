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
