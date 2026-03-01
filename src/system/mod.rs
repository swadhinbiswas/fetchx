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

