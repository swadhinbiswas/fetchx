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
