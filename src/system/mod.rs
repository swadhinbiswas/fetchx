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
