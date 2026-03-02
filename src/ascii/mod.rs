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
            "{c1}            .-/+oossssoo+/-.",
        ],
    }
}

fn debian() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7, 3],
        lines: vec![
            "{c2}       _,met$$$$$gg.",
            "{c2}    ,g$$$$$$$$$$$$$$$P.",
            r#"{c2}  ,g$$P"        """Y$$."."#,
            "{c2} ,$$P'              `$$$.",
            "{c2}',$$P       ,ggs.     `$$b:",
            "{c2}`d$$'     ,$P\"'   {c1}.{c2}    $$$",
            "{c2} $$P      d$'     {c1},{c2}    $$P",
            "{c2} $$:      $$.   {c1}-{c2}    ,d$$'",
            "{c2} $$;      Y$b._   _,d$P'",
            "{c2} Y$$.    {c1}`.{c2}`\"Y$$$$P\"'",
            "{c2}  `$$b      {c1}\"-.__",
            "{c2}   `Y$$",
            "{c2}    `Y$$.",
            "{c2}      `$$b.",
            "{c2}        `Y$$b.",
            "{c2}           `\"Y$b._",
            "{c2}               `\"\"\"",
        ],
    }
}

fn fedora() -> AsciiArt {
    AsciiArt {
        colors: vec![12, 7],
        lines: vec![
            "{c1}             .',;::::;,'.",
            "{c1}         .';:cccccccccccc:;,.",
            "{c1}      .;cccccccccccccccccccccc;.",
            "{c1}    .:cccccccccccccccccccccccccc:.",
            "{c1}  .;ccccccccccccc;{c2}.:dddl:.{c1};ccccccc;.",
            "{c1} .:ccccccccccccc;{c2}OWMKOOXMWd{c1};ccccccc:.",
            "{c1}.:ccccccccccccc;{c2}KMMc{c1};cc;{c2}xMMc{c1};ccccccc:.",
            "{c1},cccccccccccccc;{c2}MMM.{c1};cc;{c2};WW:{c1};cccccccc,",
            "{c1}:cccccccccccccc;{c2}MMM.{c1};cccccccccccccccc:",
            "{c1}:ccccccc;{c2}oxOOOo{c1};{c2}MMM0OOk.{c1};cccccccccccc:",
            "{c1}cccccc;{c2}0MMKxdd:{c1};{c2}MMMkddc.{c1};cccccccccccc;",
            "{c1}ccccc;{c2}XM0'{c1};cccc;{c2}MMM.{c1};cccccccccccccccc'",
            "{c1}ccccc;{c2}MMo{c1};ccccc;{c2}MMW.{c1};ccccccccccccccc;",
            "{c1}ccccc;{c2}0MNc.{c1}ccc{c2}.xMMd{c1};ccccccccccccccc;",
            "{c1}cccccc;{c2}dNMWXXXWM0:{c1};cccccccccccccc:,",
            "{c1}cccccccc;{c2}.:odl:.{c1};cccccccccccccc:,.",
            "{c1}:cccccccccccccccccccccccccccc:'.",
            "{c1}.:cccccccccccccccccccccc:;,..",
            "{c1}  '::cccccccccccccc::;,.",
        ],
    }
}

fn gentoo() -> AsciiArt {
    AsciiArt {
        colors: vec![5, 7],
        lines: vec![
            "{c1}         -/oyddmdhs+:.",
            "{c1}     -o{c2}dNMMMMMMMMNNmhy+{c1}-`",
            "{c1}   -y{c2}NMMMMMMMMMMMNNNmmdhy{c1}+-",
            "{c1} `o{c2}mMMMMMMMMMMMMNmdmmmmddhhy{c1}/`",
            "{c1} om{c2}MMMMMMMMMMMN{c1}hhyyyo{c2}hmdddhhhd{c1}o`",
            "{c1}.y{c2}dMMMMMMMMMMd{c1}hs++so/s{c2}mdddhhhhdm{c1}+`",
            "{c1} oy{c2}hdmNMMMMMMMN{c1}dyooy{c2}dmddddhhhhyhN{c1}d.",
            "{c1}  :o{c2}yhhdNNMMMMMMMNNNmmdddhhhhhyym{c1}Mh",
            "{c1}    .:{c2}+sydNMMMMMNNNmmmdddhhhhhhmM{c1}my",
            "{c1}       /m{c2}MMMMMMNNNmmmdddhhhhhmMNh{c1}s:",
            "{c1}    `o{c2}NMMMMMMMNNNmmmddddhhdmMNhs{c1}+`",
            "{c1}  `s{c2}NMMMMMMMMNNNmmmdddddmNMmhs{c1}/.",
            "{c1} /N{c2}MMMMMMMMNNNNmmmdddmNMNdso{c1}:`",
            "{c1}+M{c2}MMMMMMNNNNNmmmmdmNMNdso{c1}/-",
            "{c1}yM{c2}MNNNNNNNmmmmmNNMmhs+/{c1}-`",
            "{c1}/h{c2}MMNNNNNNNNMNdhs++/{c1}-`",
            "{c1}`/{c2}ohdmmddhys+++/:{c1}.`",
            "{c1}  `-//////:--.`",
        ],
    }
}

fn opensuse() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c2}           .;ldkO0000Okdl;.",
            "{c2}       .;d00xl:^''''''^:ok00d;.",
            "{c2}     .d00l'                'o00d.",
            "{c2}   .d0Kd'{c1}  Okxol:;,.          {c2}:O0d.",
            "{c2}  .OK{c1}KKK0kOKKKKKKKKKKOxo:,      {c2}lKO.",
            "{c2} ,0K{c1}KKKKKKKKKKKKKKK0P^{c2},,,{c1}^dx:{c2}    ;00,",
            "{c2}.OK{c1}KKKKKKKKKKKKKKKk'{c2}.oOPPb.{c1}'0k.{c2}   cKO.",
            "{c2}:KK{c1}KKKKKKKKKKKKKKK: {c2}kKx..dd {c1}lKd{c2}   'OK:",
            "{c2}dKK{c1}KKKKKKKKKOx0KKKd {c2}^0KKKO' {c1}kKKc{c2}   dKd",
            "{c2}dKK{c1}KKKKKKKKKK;.;oOKx,..{c2}^{c1}..;kKKK0.{c2}  dKd",
            "{c2}:KK{c1}KKKKKKKKKK0o;...^cdxxOK0O/^^'  {c2}.0K:",
            "{c2} kKK{c1}KKKKKKKKKKKKK0x;,,......,;od  {c2}lKk",
            "{c2} '0K{c1}KKKKKKKKKKKKKKKKKKKK00KKOo^  {c2}c00'",
            "{c2}  'kK{c1}KKOxddxkOO00000Okxoc;''   {c2}.dKk'",
            "{c2}    l0Ko.                    .c00l'",
            "{c2}     'l0Kk:.              .;xK0l'",
            "{c2}        'lkK0xl:;,,,,;:ldO0kl'",
            "{c2}            '^:ldxkkkkxdl:^'",
        ],
    }
}

fn manjaro() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}            \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "{c1}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}  \u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ],
    }
}

fn void_linux() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 8],
        lines: vec![
            "{c1}                __.;=====;.__",
            "{c1}            _.=+==++=++=+=+===;.",
            "{c1}             -=+++=+===+=+=+++++=_",
            "{c1}        .     -=:``     `--==+=++==.",
            "{c1}       _vi,    `            --+=++++:",
            "{c1}      .uvnvi.       _._       -==+==+.",
            "{c1}     .vvnvnI`    .;==|==;.     :|=||=|.",
            "{c2}+QmQQm{c1}pvvnv; {c2}_yYsyQQWUUQQQm #QmQ#{c1}:{c2}QQQWUV$QQm.",
            "{c2} -QQWQW{c1}pvvo{c2}wZ?.wQQQE{c1}==<{c2}QWWQ/QWQW.QQWW{c1}(: {c2}jQWQE",
            "{c2}  -$QQQQmmU'  jQQQ@{c1}+=<{c2}QWQQ)mQQQ.mQQQC{c1}+;{c2}jWQQ@'",
            "{c2}   -$WQ8Y{c1}nI:   {c2}QWQQwgQQWV{c1}`{c2}mWQQ.jQWQQgyyWW@!",
            "{c1}     -1vvnvv.     `~+++`        ++|+++",
            "{c1}      +vnvnnv,                 `-|===",
            "{c1}       +vnvnvns.           .      :=-",
            "{c1}        -Invnvvnsi..___..=sv=.     `",
            "{c1}          +Invnvnvnnnnnnnnvvnn;.",
            "{c1}            ~|Invnvnvvnvvvnnv}+`",
            "{c1}               -~|{*l}*|~",
        ],
    }
}

fn nixos() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 6],
        lines: vec![
            "{c1}          \u{2597}\u{2584}\u{2584}\u{2584}       {c2}\u{2597}\u{2584}\u{2584}\u{2584}\u{2584}    \u{2584}\u{2584}\u{2584}\u{2596}",
            "{c1}          \u{259C}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}  \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c1}           \u{259C}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}\u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c1}            \u{259C}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c1}     \u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2599} {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}     {c1}\u{259F}\u{2599}",
            "{c1}    \u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2599} {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}    {c1}\u{259F}\u{2588}\u{2588}\u{2599}",
            "{c2}           \u{2584}\u{2584}\u{2584}\u{2584}\u{2596}           \u{259C}\u{2588}\u{2588}\u{2588}\u{2599}  {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}          \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}             \u{259C}\u{2588}\u{2588}\u{259B} {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}         \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}               \u{259C}\u{259B} {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}\u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}                  {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2599}",
            "{c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}                  {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}      \u{259F}\u{2588}\u{2588}\u{2588}\u{259B} {c1}\u{259F}\u{2599}               \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}     \u{259F}\u{2588}\u{2588}\u{2588}\u{259B} {c1}\u{259F}\u{2588}\u{2588}\u{2599}             \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}    \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}  {c1}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}           \u{259D}\u{2580}\u{2580}\u{2580}\u{2580}",
            "{c2}    \u{259C}\u{2588}\u{2588}\u{259B}    {c1}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599} {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c2}     \u{259C}\u{259B}     {c1}\u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2599} {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259B}",
            "{c1}           \u{259F}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}",
            "{c1}          \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}",
            "{c1}         \u{259F}\u{2588}\u{2588}\u{2588}\u{259B}  \u{259C}\u{2588}\u{2588}\u{2588}\u{2599}       {c2}\u{259C}\u{2588}\u{2588}\u{2588}\u{2599}",
            "{c1}         \u{259D}\u{2580}\u{2580}\u{2580}    \u{2580}\u{2580}\u{2580}\u{2580}\u{2580}\u{2580}\u{2598}       {c2}\u{2580}\u{2580}\u{2580}\u{2598}",
        ],
    }
}

fn pop_os() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 7],
        lines: vec![
            "{c1}             /////////////",
            "{c1}         /////////////////////",
            "{c1}      ///////{c2}*767{c1}////////////////",
            "{c1}    //////{c2}7676767676*{c1}//////////////",
            "{c1}   /////{c2}76767{c1}//{c2}7676767{c1}//////////////",
            "{c1}  /////{c2}767676{c1}///{c2}*76767{c1}///////////////",
            "{c1} ///////{c2}767676{c1}///{c2}76767{c1}.///{c2}7676*{c1}///////",
            "{c1}/////////{c2}767676{c1}//{c2}76767{c1}///{c2}767676{c1}////////",
            "{c1}//////////{c2}76767676767{c1}////{c2}76767{c1}/////////",
            "{c1}///////////{c2}76767676{c1}//////{c2}7676{c1}//////////",
            "{c1}////////////,{c2}7676{c1},///////{c2}767{c1}///////////",
            "{c1}/////////////*{c2}7676{c1}///////{c2}76{c1}////////////",
            "{c1}///////////////{c2}7676{c1}////////////////////",
            "{c1} ///////////////{c2}7676{c1}///{c2}767{c1}////////////",
            "{c1}  //////////////////////'{c1}////////////",
            "{c1}   //////{c2}.7676767676767676767,{c1}//////",
            "{c1}    /////{c2}767676767676767676767{c1}/////",
            "{c1}      ///////////////////////////",
            "{c1}         /////////////////////",
            "{c1}             /////////////",
        ],
