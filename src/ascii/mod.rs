//! ASCII art logos for distros — mirrors neofetch's get_distro_ascii().
//!
//! Each logo uses color placeholders {c1}..{c6} which get replaced at render time.
//! Colors are specified as neofetch color numbers (0-7 = ANSI, 8+ = 256-color).

/// A distro ASCII art entry.
pub struct AsciiArt {
    /// Color numbers for c1-c6 (neofetch set_colors values)
    pub colors: Vec<u8>,
    /// The art lines, with {c1}..{c6} placeholders
    pub lines: Vec<&'static str>,
}

/// Get the ASCII art for a distro by its ID (lowercase).
/// Returns the art entry, or a generic Linux/Tux if not found.
pub fn get_ascii(distro: &str) -> AsciiArt {
    match distro {
        "arch" | "archlinux" | "arch linux" | "arcolinux" | "arco" => arch(),
        "ubuntu" | "i3buntu" | "kubuntu" | "xubuntu" | "lubuntu" => ubuntu(),
        "debian" => debian(),
        "fedora" => fedora(),
        "gentoo" => gentoo(),
        "opensuse" | "opensuse-tumbleweed" | "opensuse-leap" | "suse" => opensuse(),
        "manjaro" | "manjaro-arm" => manjaro(),
        "void" | "voidlinux" => void_linux(),
        "nixos" | "nix" => nixos(),
        "pop" | "pop_os" | "pop!_os" | "popos" => pop_os(),
        "linuxmint" | "linux mint" | "mint" | "lmde" => linux_mint(),
        "alpine" | "alpinelinux" => alpine(),
        "cachyos" => cachyos(),
        "endeavouros" | "endeavour" => endeavouros(),
        "artix" | "artixlinux" => artix(),
        "garuda" | "garudalinux" => garuda(),
        "kali" | "kalilinux" => kali(),
        "centos" => centos(),
        "rocky" | "rockylinux" | "rocky linux" => rocky(),
        "alma" | "almalinux" | "almalinux os" => alma(),
        "slackware" => slackware(),
        "zorin" | "zorinos" | "zorin os" => zorin(),
        "elementary" | "elementaryos" | "elementary os" => elementary(),
        "mx" | "mxlinux" | "mx linux" => mx(),
        "solus" => solus(),
        "deepin" => deepin(),
        "raspbian" => raspbian(),
        "freebsd" => freebsd(),
        "openbsd" => openbsd(),
        "macos" | "darwin" | "mac os x" | "mac" => macos(),
        "windows" | "msys2" | "cygwin" => windows(),
        "android" => android(),
        "rhel" | "red hat" | "redhat" | "red hat enterprise linux" => rhel(),
        "amazon" | "amzn" | "amazon linux" => amazon_linux(),
        "chromeos" | "chrome os" | "chromiumos" => chromeos(),
        "bedrock" | "bedrocklinux" => bedrock(),
        "clear" | "clearlinux" | "clear linux" => clearlinux(),
        "crux" => crux(),
        "devuan" => devuan(),
        "dragonfly" | "dragonflybsd" => dragonfly(),
        "endless" | "endlessos" => endless(),
        "exherbo" => exherbo(),
        "guix" | "guixsd" => guix(),
