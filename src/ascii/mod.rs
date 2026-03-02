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
        "haiku" => haiku(),
        "hyperbola" => hyperbola(),
        "instantos" | "instant" => instantos(),
        "mageia" => mageia(),
        "netbsd" => netbsd(),
        "parabola" => parabola(),
        "parrot" | "parrotos" | "parrot os" => parrot(),
        "peppermint" | "peppermintos" => peppermint(),
        "porteus" => porteus(),
        "postmarket" | "postmarketos" => postmarketos(),
        "puppy" | "puppylinux" => puppy(),
        "pureos" | "pure" => pureos(),
        "sabayon" => sabayon(),
        "slitaz" => slitaz(),
        "tails" => tails(),
        "trisquel" => trisquel(),
        _ => linux_generic(),
    }
}

fn arch() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 6, 7, 1],
        lines: vec![
            "{c1}                   -`",
            "{c1}                  .o+`",
            "{c1}                 `ooo/",
            "{c1}                `+oooo:",
            "{c1}               `+oooooo:",
            "{c1}               -+oooooo+:",
            "{c1}             `/:-:++oooo+:",
            "{c1}            `/++++/+++++++:",
            "{c1}           `/++++++++++++++:",
            "{c1}          `/+++o{c2}oooooooo{c1}oooo/`",
            "{c2}         {c1}./{c2}ooosssso++osssssso{c1}+`",
            "{c2}        .oossssso-````/ossssss+`",
            "{c2}       -osssssso.      :ssssssso.",
            "{c2}      :osssssss/        osssso+++.",
            "{c2}     /ossssssss/        +ssssooo/-",
            "{c2}   `/ossssso+/:-        -:/+osssso+-",
            "{c2}  `+sso+:-`                 `.-/+oso:",
            "{c2} `++:.                           `-/+/",
            "{c2} .`                                 `/",
        ],
    }
}

fn ubuntu() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7, 3],
        lines: vec![
            "{c1}            .-/+oossssoo+/-.",
            "{c1}        `:+ssssssssssssssssss+:`",
            "{c1}      -+ssssssssssssssssssyyssss+-",
            "{c1}    .ossssssssssssssssss{c2}dMMMNy{c1}sssso.",
            "{c1}   /sssssssssss{c2}hdmmNNmmyNMMMMh{c1}ssssss/",
            "{c1}  +sssssssss{c2}hm{c1}yd{c2}MMMMMMMNddddy{c1}ssssssss+",
            "{c1} /ssssssss{c2}hNMMM{c1}yh{c2}hyyyyhmNMMMNh{c1}ssssssss/",
            "{c1}.ssssssss{c2}dMMMNh{c1}ssssssssss{c2}hNMMMd{c1}ssssssss.",
            "{c1}+ssss{c2}hhhyNMMNy{c1}ssssssssssss{c2}yNMMMy{c1}sssssss+",
            "{c1}oss{c2}yNMMMNyMMh{c1}ssssssssssssss{c2}hmmmh{c1}ssssssso",
            "{c1}oss{c2}yNMMMNyMMh{c1}sssssssssssssshmmmh{c1}ssssssso",
            "{c1}+ssss{c2}hhhyNMMNy{c1}ssssssssssss{c2}yNMMMy{c1}sssssss+",
            "{c1}.ssssssss{c2}dMMMNh{c1}ssssssssss{c2}hNMMMd{c1}ssssssss.",
            "{c1} /ssssssss{c2}hNMMM{c1}yh{c2}hyyyyhdNMMMNh{c1}ssssssss/",
            "{c1}  +sssssssss{c2}dm{c1}yd{c2}MMMMMMMMddddy{c1}ssssssss+",
            "{c1}   /sssssssssss{c2}hdmNNNNmyNMMMMh{c1}ssssss/",
            "{c1}    .ossssssssssssssssss{c2}dMMMNy{c1}sssso.",
            "{c1}      -+sssssssssssssssss{c2}yyy{c1}ssss+-",
            "{c1}        `:+ssssssssssssssssss+:`",
