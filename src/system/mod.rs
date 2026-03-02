//! System information module for FetchX
//!
//! Gathers all system info fields, mirroring neofetch's detection logic.

use crate::utils::{command_exists, env_or, read_file, read_first_line, run_cmd};
use std::path::Path;

/// All system information fields.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os: String,
    pub distro_id: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub de: String,
    pub wm: String,
    pub wm_theme: String,
    pub theme: String,
    pub icons: String,
    pub terminal: String,
    pub term_font: String,
    pub cpu: String,
    pub gpu: Vec<String>,
    pub memory: String,
    pub memory_percent: f64,
    pub disk: String,
    pub disk_percent: f64,
    pub battery: String,
    pub local_ip: String,
    pub public_ip: String,
    pub locale: String,
    pub song: String,
    pub users: String,
}

impl SystemInfo {
    /// Collect all system information using parallel threads for speed.
    pub fn collect() -> Self {
        use std::thread;

        let distro_id = detect_distro_id();
        let did = distro_id.clone();

        // Spawn threads for slow/independent operations
        let h_packages = thread::spawn(get_packages);
        let h_resolution = thread::spawn(get_resolution);
        let h_gpu = thread::spawn(get_gpu);
        let h_public_ip = thread::spawn(get_public_ip);
        let h_song = thread::spawn(get_song);
        let h_cpu = thread::spawn(get_cpu);
        let h_term_font = thread::spawn(get_term_font);
        let h_wm_theme = thread::spawn(get_wm_theme);

        // Fast operations run on main thread
        let username = get_username();
        let hostname = get_hostname();
        let os = get_os();
        let host = get_host();
        let kernel = get_kernel();
        let uptime = get_uptime();
        let shell = get_shell();
        let de = get_de();
        let wm = get_wm();
        let theme = get_theme();
        let icons = get_icons();
        let terminal = get_terminal();
        let (memory, memory_percent) = get_memory_with_percent();
        let (disk, disk_percent) = get_disk_with_percent();
        let battery = get_battery();
        let local_ip = get_local_ip();
        let locale = get_locale();
        let users = get_users();

        // Join threads
        let packages = h_packages.join().unwrap_or_default();
        let resolution = h_resolution.join().unwrap_or_default();
        let gpu = h_gpu.join().unwrap_or_default();
        let public_ip = h_public_ip.join().unwrap_or_else(|_| "Unknown".to_string());
        let song = h_song.join().unwrap_or_default();
        let cpu = h_cpu.join().unwrap_or_default();
        let term_font = h_term_font.join().unwrap_or_default();
        let wm_theme = h_wm_theme.join().unwrap_or_else(|_| "Unknown".to_string());

        Self {
            username,
            hostname,
            os,
            distro_id: did,
            host,
            kernel,
            uptime,
            packages,
            shell,
            resolution,
            de,
            wm,
            wm_theme,
            theme,
            icons,
            terminal,
            term_font,
            cpu,
            gpu,
            memory,
            memory_percent,
            disk,
            disk_percent,
            battery,
            local_ip,
            public_ip,
            locale,
            song,
            users,
        }
    }

    /// Format the title line: user@host
    #[allow(dead_code)]
    pub fn title(&self) -> String {
        format!("{}@{}", self.username, self.hostname)
    }
}

// ─── Detection Functions ────────────────────────────────────────────

fn get_username() -> String {
    let user = env_or("USER");
    if !user.is_empty() {
        return user;
    }
    let logname = env_or("LOGNAME");
    if !logname.is_empty() {
        return logname;
    }
    let whoami = run_cmd("whoami", &[]);
    if !whoami.is_empty() {
        return whoami;
    }
    "unknown".to_string()
}

// Helper trait: allow Option-like behavior on String
#[allow(dead_code)]
trait OrElseStr {
    fn or_else_str<F: FnOnce() -> String>(self, f: F) -> String;
}
impl OrElseStr for String {
    fn or_else_str<F: FnOnce() -> String>(self, f: F) -> String {
        if self.is_empty() {
            f()
        } else {
            self
        }
    }
}

fn get_hostname() -> String {
    let h = read_first_line("/etc/hostname");
    if !h.is_empty() {
        return h;
    }
    run_cmd("hostname", &[])
}

fn detect_distro_id() -> String {
    let content = read_file("/etc/os-release");
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            return val.trim_matches('"').to_lowercase();
        }
    }
    "linux".to_string()
}

fn get_os() -> String {
    // Try /etc/os-release PRETTY_NAME first
    let content = read_file("/etc/os-release");
    if !content.is_empty() {
        let mut pretty = String::new();
        let mut name = String::new();
        let mut version = String::new();
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                pretty = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("NAME=") {
                name = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("VERSION=") {
                version = val.trim_matches('"').to_string();
            }
        }
        if !pretty.is_empty() {
            // Append arch
            let arch = run_cmd("uname", &["-m"]);
            if !arch.is_empty() {
                return format!("{} {}", pretty, arch);
            }
            return pretty;
        }
        if !name.is_empty() {
            let arch = run_cmd("uname", &["-m"]);
            let base = if !version.is_empty() {
                format!("{} {}", name, version)
            } else {
                name
            };
            if !arch.is_empty() {
                return format!("{} {}", base, arch);
            }
            return base;
        }
    }

    // lsb_release fallback
    let lsb = run_cmd("lsb_release", &["-d", "-s"]);
    if !lsb.is_empty() {
        return lsb;
    }

    // uname fallback
    let uname = run_cmd("uname", &["-o"]);
    if !uname.is_empty() {
        return uname;
    }

    "Unknown".to_string()
}

fn get_host() -> String {
    // DMI: product_name + product_version
    let name = read_first_line("/sys/class/dmi/id/product_name");
    let version = read_first_line("/sys/class/dmi/id/product_version");

    if !name.is_empty() {
        let name = filter_oem(&name);
        if name.is_empty() {
            return "Unknown".to_string();
        }
        let version = filter_oem(&version);
        if !version.is_empty() {
            return format!("{} {}", name, version);
        }
        return name;
    }

    // dmidecode fallback
    let dmi = run_cmd("dmidecode", &["-s", "system-product-name"]);
    let dmi = filter_oem(&dmi);
    if !dmi.is_empty() {
        return dmi;
    }

    "Unknown".to_string()
}

fn filter_oem(s: &str) -> String {
    let s = s.trim();
    let lc = s.to_lowercase();
    if lc.is_empty()
        || lc == "to be filled by o.e.m."
        || lc == "not applicable"
        || lc == "system product name"
        || lc == "type1productconfigid"
        || lc == "default string"
        || lc == "undefined"
        || lc == "not specified"
        || lc == "none"
    {
        return String::new();
    }
    s.to_string()
}

fn get_kernel() -> String {
    let k = run_cmd("uname", &["-r"]);
    if k.is_empty() {
        "Unknown".to_string()
    } else {
        k
    }
}

fn get_uptime() -> String {
    // /proc/uptime: "seconds.frac idle_seconds.frac"
    let content = read_file("/proc/uptime");
    if let Some(secs_str) = content.split_whitespace().next() {
        if let Ok(secs_f) = secs_str.parse::<f64>() {
            let secs = secs_f as u64;
            return format_uptime(secs);
        }
    }
    // Fallback
    let up = run_cmd("uptime", &["-p"]);
    if !up.is_empty() {
        return up.trim_start_matches("up ").to_string();
    }
    "Unknown".to_string()
}

fn format_uptime(total_secs: u64) -> String {
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(if days == 1 {
            "1 day".to_string()
        } else {
            format!("{} days", days)
        });
    }
    if hours > 0 {
        parts.push(if hours == 1 {
            "1 hour".to_string()
        } else {
            format!("{} hours", hours)
        });
    }
    if mins > 0 || parts.is_empty() {
        parts.push(if mins == 1 {
            "1 min".to_string()
        } else {
            format!("{} mins", mins)
        });
    }
    parts.join(", ")
}

fn get_packages() -> String {
    let mut managers: Vec<String> = Vec::new();

    // pacman
    if Path::new("/var/lib/pacman/local").exists() {
        if let Ok(entries) = std::fs::read_dir("/var/lib/pacman/local") {
            let count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .count()
                .saturating_sub(1); // subtract the ALPM_DB_VERSION dir
            if count > 0 {
                managers.push(format!("{} (pacman)", count));
            }
        }
    }

    // dpkg
    if Path::new("/var/lib/dpkg/status").exists() {
        let content = read_file("/var/lib/dpkg/status");
        let count = content
            .split("\n\n")
            .filter(|block| {
                block
                    .lines()
                    .any(|l| l.starts_with("Status: install ok installed"))
            })
            .count();
        if count > 0 {
            managers.push(format!("{} (dpkg)", count));
        }
    }

    // rpm
    if command_exists("rpm") {
        let out = run_cmd("rpm", &["-qa", "--last"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (rpm)", count));
            }
        }
    }

    // portage (Gentoo)
    if Path::new("/var/db/pkg").exists() && command_exists("qlist") {
        let out = run_cmd("qlist", &["-I"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (portage)", count));
            }
        }
    }

    // xbps (Void)
    if command_exists("xbps-query") {
        let out = run_cmd("xbps-query", &["-l"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (xbps)", count));
            }
        }
    }

    // apk (Alpine)
    if command_exists("apk") {
        let out = run_cmd("apk", &["info"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (apk)", count));
            }
        }
    }

    // nix
    if command_exists("nix-store") {
        let out = run_cmd("nix-store", &["-qR", "/run/current-system/sw"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (nix)", count));
            }
        }
    }

    // flatpak
    if command_exists("flatpak") {
        let out = run_cmd("flatpak", &["list"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (flatpak)", count));
            }
        }
    }

    // snap
    if command_exists("snap") {
        let out = run_cmd("snap", &["list"]);
        if !out.is_empty() {
            let count = out.lines().filter(|l| !l.starts_with("Name")).count();
            if count > 0 {
                managers.push(format!("{} (snap)", count));
            }
        }
    }

    // brew
    if command_exists("brew") {
        let out = run_cmd("brew", &["list", "--formula", "-1"]);
        if !out.is_empty() {
            let count = out.lines().count();
            if count > 0 {
                managers.push(format!("{} (brew)", count));
            }
        }
    }

    if managers.is_empty() {
        return "Unknown".to_string();
    }

    // Format like neofetch's 'on' mode: "142 (pacman), 12 (flatpak)"
    if managers.len() == 1 {
        managers[0].clone()
    } else {
        managers.join(", ")
    }
}

fn get_shell() -> String {
    let shell_path = env_or("SHELL");
    if shell_path.is_empty() {
        return "Unknown".to_string();
    }

    let shell_name = shell_path.split('/').last().unwrap_or("Unknown");

    // Get version
    let version = match shell_name {
        "bash" => {
            // Use single-quoted command to avoid expanding $BASH_VERSION in the parent shell
            let v = run_cmd("bash", &["-c", "printf '%s' \"$BASH_VERSION\""]);
            if !v.is_empty() {
                // Strip everything after first '-' (like neofetch: ${BASH_VERSION/-*})
                v.split('-').next().unwrap_or(&v).to_string()
            } else {
                run_cmd("bash", &["--version"])
                    .lines()
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .nth(3)
                    .unwrap_or("")
                    .split('-')
                    .next()
                    .unwrap_or("")
                    .to_string()
            }
        }
        "zsh" => run_cmd("zsh", &["--version"])
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .to_string(),
        "fish" => run_cmd("fish", &["--version"]).replace("fish, version ", ""),
        "ksh" | "mksh" => run_cmd(shell_name, &["-c", "echo $KSH_VERSION"]),
        "tcsh" => run_cmd("tcsh", &["--version"])
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .to_string(),
        "nu" | "nushell" => run_cmd("nu", &["--version"]),
        _ => String::new(),
    };

    if version.is_empty() {
        shell_name.to_string()
    } else {
        format!("{} {}", shell_name, version)
    }
}

fn get_resolution() -> String {
    // Try hyprctl first for Hyprland (Wayland)
    if command_exists("hyprctl") {
        let out = run_cmd("hyprctl", &["monitors", "-j"]);
        if !out.is_empty() && out.starts_with('[') {
            // Parse JSON-like output for width/height
            let mut resolutions: Vec<String> = Vec::new();
            // Simple extraction: find "width": N and "height": N pairs
            let mut i = 0;
            let chars: Vec<char> = out.chars().collect();
            while i < chars.len() {
                if out[i..].starts_with("\"width\":") {
                    let w_start = i + 8;
                    let w_str: String = out[w_start..].chars().take_while(|c| c.is_ascii_digit() || *c == ' ').collect();
                    let width = w_str.trim().parse::<u32>().unwrap_or(0);
                    // Find the next "height":
                    if let Some(h_pos) = out[w_start..].find("\"height\":") {
                        let h_start = w_start + h_pos + 9;
                        let h_str: String = out[h_start..].chars().take_while(|c| c.is_ascii_digit() || *c == ' ').collect();
                        let height = h_str.trim().parse::<u32>().unwrap_or(0);
                        if width > 0 && height > 0 {
                            resolutions.push(format!("{}x{}", width, height));
                        }
                    }
                }
                i += 1;
            }
            if !resolutions.is_empty() {
                return resolutions.join(", ");
            }
        }

        // Non-JSON fallback: parse text output
        let out = run_cmd("hyprctl", &["monitors"]);
        if !out.is_empty() {
            let mut resolutions: Vec<String> = Vec::new();
            let mut found_monitor = false;
            for line in out.lines() {
                let trimmed = line.trim();
                // Monitor header lines don't start with whitespace
                if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                    found_monitor = true;
                    continue;
                }
                // The resolution line typically contains "at WxH@rate"
                // or just "WxH@rate" as the first data after the monitor name
                if found_monitor && trimmed.contains('@') && trimmed.contains('x') {
                    if let Some(res) = trimmed.split('@').next() {
                        let res = res.trim();
                        if res.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                            resolutions.push(res.to_string());
                            found_monitor = false; // Only first resolution per monitor
                        }
                    }
                }
            }
            if !resolutions.is_empty() {
                return resolutions.join(", ");
            }
        }
    }

    // Try xrandr (X11 only — skip on pure Wayland)
    let wayland = env_or("WAYLAND_DISPLAY");
    if command_exists("xrandr") && wayland.is_empty() {
        let out = run_cmd("xrandr", &["--current"]);
        if !out.is_empty() {
            let mut resolutions: Vec<String> = Vec::new();
            for line in out.lines() {
                if line.contains(" connected") {
                    // Find resolution pattern like "1920x1080+0+0"
                    for word in line.split_whitespace() {
                        if word.contains('x')
                            && word.chars().next().map_or(false, |c| c.is_ascii_digit())
                        {
                            let res = word.split('+').next().unwrap_or(word);
                            resolutions.push(res.to_string());
                            break;
                        }
                    }
                }
            }
            if !resolutions.is_empty() {
                return resolutions.join(", ");
            }
        }
    }

    // Try wlr-randr (Wayland)
    if command_exists("wlr-randr") {
        let out = run_cmd("wlr-randr", &[]);
        if !out.is_empty() {
            let mut resolutions: Vec<String> = Vec::new();
            for line in out.lines() {
                let trimmed = line.trim();
                if trimmed.contains("current") {
                    // "1920x1080 px, 60.000000 Hz (current)"
                    if let Some(res) = trimmed.split_whitespace().next() {
                        if res.contains('x') {
                            resolutions.push(res.to_string());
                        }
                    }
                }
            }
            if !resolutions.is_empty() {
                return resolutions.join(", ");
            }
        }
    }

    // /sys/class/drm fallback (like neofetch)
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        let mut resolutions: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let modes_path = entry.path().join("modes");
            if modes_path.exists() {
                let mode = read_first_line(modes_path.to_str().unwrap_or(""));
                if !mode.is_empty() && mode.contains('x') {
                    resolutions.push(mode);
                }
            }
        }
        if !resolutions.is_empty() {
            // Deduplicate
            resolutions.sort();
            resolutions.dedup();
            return resolutions.join(", ");
        }
    }

    "Unknown".to_string()
}

fn get_de() -> String {
    let de = env_or("XDG_CURRENT_DESKTOP");
    let de_name = if !de.is_empty() {
        de
    } else {
        let session = env_or("DESKTOP_SESSION");
        if !session.is_empty() {
            session
        } else {
            return "Unknown".to_string();
        }
    };

    // Try to get version for known DEs
    let version = get_de_version(&de_name);
    if version.is_empty() {
        de_name
    } else {
        format!("{} {}", de_name, version)
    }
}

/// Detect DE version for known desktop environments (like neofetch).
fn get_de_version(de: &str) -> String {
    let de_lower = de.to_lowercase();
    if de_lower.contains("gnome") || de_lower.contains("unity") {
        let out = run_cmd("gnome-shell", &["--version"]);
        // "GNOME Shell 45.2" → "45.2"
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    if de_lower.contains("kde") || de_lower.contains("plasma") {
        let out = run_cmd("plasmashell", &["--version"]);
        // "plasmashell 5.27.10" or "plasmashell 6.0.2"
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    if de_lower.contains("xfce") {
        let out = run_cmd("xfce4-session", &["--version"]);
        for line in out.lines() {
            if line.contains("xfce4-session") || line.contains("xfce") {
                if let Some(v) = line.split_whitespace().last() {
                    if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        return v.to_string();
                    }
                }
            }
        }
    }
    if de_lower.contains("cinnamon") {
        let out = run_cmd("cinnamon", &["--version"]);
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    if de_lower.contains("mate") {
        let out = run_cmd("mate-session", &["--version"]);
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    if de_lower.contains("lxqt") {
        let out = run_cmd("lxqt-about", &["--version"]);
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    if de_lower.contains("budgie") {
        let out = run_cmd("budgie-desktop", &["--version"]);
        if let Some(v) = out.split_whitespace().last() {
            if v.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                return v.to_string();
            }
        }
    }
    String::new()
}

fn get_wm() -> String {
    // Check for Wayland compositors
    let wayland = env_or("WAYLAND_DISPLAY");
    if !wayland.is_empty() {
        let xdg = env_or("XDG_CURRENT_DESKTOP").to_lowercase();
        if xdg.contains("hyprland") {
            return "Hyprland".to_string();
        }
        if xdg.contains("sway") {
            return "Sway".to_string();
        }
        if xdg.contains("river") {
            return "river".to_string();
        }
    }

    // Try xprop for X11
    if command_exists("xprop") {
        let out = run_cmd("xprop", &["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"]);
        if !out.is_empty() {
            // Gets window ID, then query _NET_WM_NAME on it
            if let Some(wid) = out.split_whitespace().last() {
                let name_out = run_cmd(
                    "xprop",
                    &["-id", wid, "-notype", "-f", "_NET_WM_NAME", "8t"],
                );
                if let Some(line) = name_out.lines().find(|l| l.contains("_NET_WM_NAME")) {
                    if let Some(val) = line.split('=').nth(1) {
                        let wm = val.trim().trim_matches('"');
                        if !wm.is_empty() {
                            return wm.to_string();
                        }
                    }
                }
            }
        }
    }

    // wmctrl fallback
    if command_exists("wmctrl") {
        let out = run_cmd("wmctrl", &["-m"]);
        for line in out.lines() {
            if let Some(name) = line.strip_prefix("Name:") {
                let name = name.trim();
                if !name.is_empty() {
                    return name.to_string();
                }
            }
        }
    }

    let session = env_or("XDG_SESSION_DESKTOP");
    if !session.is_empty() {
        return session;
    }

    "Unknown".to_string()
}

fn get_wm_theme() -> String {
    // GNOME / GTK-based
    let theme = run_cmd(
        "gsettings",
        &["get", "org.gnome.desktop.wm.preferences", "theme"],
    );
    let theme = theme.trim_matches('\'').trim().to_string();
    if !theme.is_empty() && theme != "No such schema" && !theme.starts_with("Error") {
        return theme;
    }

    // KDE Plasma
    let kde_config = format!("{}/.config/kwinrc", crate::utils::home_dir());
    if Path::new(&kde_config).exists() {
        let content = read_file(&kde_config);
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("theme=") {
                if !val.is_empty() {
                    return val.to_string();
                }
            }
        }
    }

    "Unknown".to_string()
}

fn get_theme() -> String {
    // GTK theme
    let theme = run_cmd(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "gtk-theme"],
    );
    let theme = theme.trim_matches('\'').trim().to_string();
    if !theme.is_empty() && theme != "No such schema" && !theme.starts_with("Error") {
        return theme;
    }

    // Check GTK3 settings file
    let gtk3_settings = format!("{}/.config/gtk-3.0/settings.ini", crate::utils::home_dir());
    if Path::new(&gtk3_settings).exists() {
        let content = read_file(&gtk3_settings);
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("gtk-theme-name=") {
                return val.trim().to_string();
            }
        }
    }

    "Unknown".to_string()
}

fn get_icons() -> String {
    let icons = run_cmd(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "icon-theme"],
    );
    let icons = icons.trim_matches('\'').trim().to_string();
    if !icons.is_empty() && icons != "No such schema" && !icons.starts_with("Error") {
        return icons;
    }

    let gtk3_settings = format!("{}/.config/gtk-3.0/settings.ini", crate::utils::home_dir());
    if Path::new(&gtk3_settings).exists() {
        let content = read_file(&gtk3_settings);
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("gtk-icon-theme-name=") {
                return val.trim().to_string();
            }
        }
    }

    "Unknown".to_string()
}

fn get_terminal() -> String {
    // Walk up the process tree to find the terminal emulator
    // This mirrors neofetch's PPID walking approach
    let mut pid = std::process::id();

    for _ in 0..10 {
        let stat_path = format!("/proc/{}/status", pid);
        let content = read_file(&stat_path);
        if content.is_empty() {
            break;
        }

        let mut name = String::new();
        let mut ppid = 0u32;

        for line in content.lines() {
            if let Some(val) = line.strip_prefix("Name:\t") {
                name = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("PPid:\t") {
                ppid = val.trim().parse().unwrap_or(0);
            }
        }

        let lc = name.to_lowercase();
        // Skip shells, init, login
        let skip = [
            "bash", "zsh", "fish", "sh", "dash", "ksh", "tcsh", "csh", "nu", "nushell", "login",
            "init", "systemd", "sshd", "su", "sudo", "fetchx", "neofetch", "screen", "tmux",
            "code", "node", "electron", "cargo", "rustc",
        ];
        if skip.iter().any(|s| lc == *s) {
            pid = ppid;
            if ppid <= 1 {
                break;
            }
            continue;
        }

        // Found a non-shell, non-init process — this is likely the terminal
        return name;
    }

    // Fallback to env vars
    let term_program = env_or("TERM_PROGRAM");
    if !term_program.is_empty() {
        return term_program;
    }

    let colorterm = env_or("COLORTERM");
    if !colorterm.is_empty() {
        return colorterm;
    }

    env_or("TERM")
}

fn get_term_font() -> String {
    let terminal = get_terminal().to_lowercase();

    // Alacritty
    if terminal.contains("alacritty") {
        let paths = [
            format!(
                "{}/.config/alacritty/alacritty.toml",
                crate::utils::home_dir()
            ),
            format!(
                "{}/.config/alacritty/alacritty.yml",
                crate::utils::home_dir()
            ),
        ];
        for p in &paths {
            if Path::new(p).exists() {
                let content = read_file(p);
                for line in content.lines() {
                    let trimmed = line.trim();
                    if let Some(val) = trimmed.strip_prefix("family") {
                        let val = val
                            .trim()
                            .trim_start_matches(['=', ':'])
                            .trim()
                            .trim_matches('"');
                        if !val.is_empty() {
                            return val.to_string();
                        }
                    }
                }
            }
        }
    }

    // Kitty
    if terminal.contains("kitty") {
        let path = format!("{}/.config/kitty/kitty.conf", crate::utils::home_dir());
        if Path::new(&path).exists() {
            let content = read_file(&path);
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("font_family") {
                    let val = val.trim();
                    if !val.is_empty() {
                        return val.to_string();
                    }
                }
            }
        }
    }

    // Konsole
    if terminal.contains("konsole") {
        let profile_dir = format!("{}/.local/share/konsole", crate::utils::home_dir());
        if let Ok(entries) = std::fs::read_dir(&profile_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |e| e == "profile") {
                    let content = read_file(entry.path().to_str().unwrap_or(""));
                    for line in content.lines() {
                        if let Some(val) = line.strip_prefix("Font=") {
                            if let Some(font) = val.split(',').next() {
                                return font.to_string();
                            }
                        }
                    }
                }
            }
        }
    }

    "Unknown".to_string()
}

fn get_cpu() -> String {
    // /proc/cpuinfo
    let content = read_file("/proc/cpuinfo");
    if !content.is_empty() {
        let mut model = String::new();
        let mut cores = 0u32;

        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(val) = line.split(':').nth(1) {
                    model = val.trim().to_string();
                }
            }
            if line.starts_with("processor") {
                cores += 1;
            }
        }

        // Get CPU speed from cpufreq sysfs (like neofetch)
        // Fallback chain: bios_limit -> scaling_max_freq -> cpuinfo_max_freq -> /proc/cpuinfo
        let speed_dir = "/sys/devices/system/cpu/cpu0/cpufreq";
        let mut speed_khz: u64 = 0;

        if Path::new(speed_dir).is_dir() {
            for file in &[
                "bios_limit",
                "scaling_max_freq",
                "cpuinfo_max_freq",
            ] {
                let path = format!("{}/{}", speed_dir, file);
                let val = read_first_line(&path);
                if let Ok(khz) = val.parse::<u64>() {
                    if khz > 0 {
                        speed_khz = khz;
