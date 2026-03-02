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
    }
}

fn linux_mint() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c2}             ...-:::::-...",
            "{c2}          .-MMMMMMMMMMMMMMM-.",
            "{c2}      .-MMMM{c1}`..-:::::::-..`{c2}MMMM-.",
            "{c2}    .:MMMM{c1}.:MMMMMMMMMMMMMMM:.{c2}MMMM:.",
            "{c2}   -MMM{c1}-M---MMMMMMMMMMMMMMMMMMM.{c2}MMM-",
            "{c2} `:MMM{c1}:MM`  :MMMM:....::-...-MMMM:{c2}MMM:`",
            "{c2} :MMM{c1}:MMM`  :MM:`  ``    ``  `:MMM:{c2}MMM:",
            "{c2}.MMM{c1}.MMMM`  :MM.  -MM.  .MM-  `MMMM.{c2}MMM.",
            "{c2}:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{c2}MMM:",
            "{c2}:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM:{c2}MMM:",
            "{c2}:MMM{c1}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{c2}MMM:",
            "{c2}.MMM{c1}.MMMM`  :MM:--:MM:--:MM:  `MMMM.{c2}MMM.",
            "{c2} :MMM{c1}:MMM-  `-MMMMMMMMMMMM-`  -MMM-{c2}MMM:",
            "{c2}  :MMM{c1}:MMM:`                `:MMM:{c2}MMM:",
            "{c2}   .MMM{c1}.MMMM:--------------:MMMM.{c2}MMM.",
            "{c2}     '-MMMM{c1}.-MMMMMMMMMMMMMMM-.{c2}MMMM-'",
            "{c2}       '.-MMMM{c1}``--:::::--``{c2}MMMM-.'",
            "{c2}            '-MMMMMMMMMMMMM-'",
            "{c2}               ``-:::::-``",
        ],
    }
}

fn alpine() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 5, 7, 6],
        lines: vec![
            "{c1}       .hddddddddddddddddddddddh.",
            "{c1}      :dddddddddddddddddddddddddd:",
            "{c1}     /dddddddddddddddddddddddddddd/",
            "{c1}    +dddddddddddddddddddddddddddddd+",
            "{c1}  `sdddddddddddddddddddddddddddddddds`",
            "{c1} `ydddddddddddd++hdddddddddddddddddddy`",
            "{c1}.hddddddddddd+`  `+ddddh:-sdddddddddddh.",
            "{c1}hdddddddddd+`      `+y:    .sddddddddddh",
            "{c1}ddddddddh+`   `//`   `.`     -sddddddddd",
            "{c1}ddddddh+`   `/hddh/`   `:s-    -sddddddd",
            "{c1}ddddh+`   `/+/dddddh/`   `+s-    -sddddd",
            "{c1}ddd+`   `/o` :dddddddh/`   `oy-    .yddd",
            "{c1}hdddyo+ohddyosdddddddddho+oydddy++ohdddh",
            "{c1}.hddddddddddddddddddddddddddddddddddddh.",
            "{c1} `yddddddddddddddddddddddddddddddddddy`",
            "{c1}  `sdddddddddddddddddddddddddddddddds`",
            "{c1}    +dddddddddddddddddddddddddddddd+",
            "{c1}     /dddddddddddddddddddddddddddd/",
            "{c1}      :dddddddddddddddddddddddddd:",
            "{c1}       .hddddddddddddddddddddddh.",
        ],
    }
}

fn cachyos() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 6, 7],
        lines: vec![
            "{c1}           .-/+oss/`",
            "{c1}        .:+osssssso-",
            "{c1}       -osssssssso/.`     `.-::.",
            "{c1}      /ossssssso/.     `-+ossss+",
            "{c1}     /ossso+:-.`   `.:+ossssso-",
            "{c1}    +osss/`      .:+ossssssso-",
            "{c1}   .osso-     .:+ossssssssso-",
            "{c1}   :oss/   `-+osssssssssso/.",
            "{c1}   -oss/ `-+ossssssssssso/.",
            "{c1}    +oss+ossssssssssso+:`",
            "{c1}     -+osssssssssso+:`",
            "{c1}       `-/+ooo+/-`",
        ],
    }
}

fn endeavouros() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 5, 4],
        lines: vec![
            "                     {c1}./o.",
            "                   {c1}./sssso-",
            "                 {c1}`/osssssss+-",
            "                {c1}/ossssssssssso/",
            "              {c1}-osssssssssssssss+-",
            "            {c1}.+ossssssssssssssssso/.",
            "           {c1}/osssssssssssssssssssss+-",
            "         {c1}.+ossssssssssssss{c2}oooo{c1}sssssso.",
            "        {c1}/osssssssssssss{c2}ooooooo{c1}sssssss+.",
            "       {c1}+osssssssssss{c2}ooooooooooo{c1}ssssssso\\",
            "     {c1}`+osssssssss{c2}oooooooooooooo{c1}ssssssso/",
            "    {c1}`/osssssss{c2}oooooooooooooooooo{c1}sssssss-",
            "    {c1}+osssss{c2}oooooooooooooooo{c3}++++{c2}o{c1}sssssso+`",
            "   {c1}/osss{c2}oooooooooooooo{c3}++++++++++{c1}ssssso\\",
            "  {c1}/oss{c2}ooooooooooo{c3}++++++++++++++++{c1}ssso-",
            " {c1}+os{c2}oooooooo{c3}++++++++++++++++++++{c1}sso/`",
            "{c1}`/o{c2}ooooo{c3}++++++++++++++++++++++++{c1}s+-`",
            "{c1}-o{c3}++++++++++++++++++++++++++++{c1}/`",
            "{c1}-{c3}+++++++++++++++++++++++++{c1}/-`",
            " {c3}`-/++++++++++++++++/-.`",
        ],
    }
}

fn artix() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 4, 2],
        lines: vec![
            "{c1}                   /\\",
            "{c1}                  /  \\",
            "{c1}                 /`'.,\\",
            "{c1}                /     ',",
            "{c1}               /      ,`\\",
            "{c1}              /   ,.'`.  \\",
            "{c1}             /.,'`     `'.\\",
        ],
    }
}

fn garuda() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 4, 6],
        lines: vec![
            "            {c2}.:+syyyyyys+:.",
            "         {c2}.+hhhhhhhhhhhhhhhh+.",
            "       {c2}:shhhhhhhhhhhhhhhhhhhs:",
            "      {c2}ohhhhhh{c1}ssssssssss{c2}hhhhhho",
            "     {c2}shhhhh{c1}s          {c2}hhhhhhs",
            "    {c2}shhhhh{c1}s     {c3}.::.  {c2}shhhhs",
            "   {c2}/hhhhhh{c1}s    {c3}`+hhh-  {c2}shhhh\\",
            "  {c2}.hhhhhhh{c1}s   {c3}`+hhhh+  {c2}shhhh.",
            "  {c2}:hhhhhhh{c1}ss{c3}`+hhhhhh+  {c2}shhhh:",
            "  {c2}:hhhhhhh{c3}`+hhhhhhhh+  {c2}shhhh:",
            "  {c2} hhhhhhh{c3}`+hhhhhhhh+  {c2}shhhh",
            "   {c2}ohhhhhh{c3}`+hhhhhh+`  {c2}shhhho",
            "    {c2}`ohhhhh{c3}`+hhh+`   {c2}shhho`",
        ],
    }
}

fn kali() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 8],
        lines: vec![
            "{c1}...............",
            "{c1}            ..,;:ccc,.",
            "{c1}          ......''';lxO.",
            "{c1}.....''''..........,:ld;",
            "{c1}           .';;;:::;,,.x,",
            "{c1}      ..'''.            0Wmmm:;'.",
            "{c1}    ....                .:...Xmmm0;",
            "{c1}   .;'                ..;'. dxkOKMW0.",
            "{c1}  'l.  .;'         ...''   ..    .0MMKX;",
            "{c1} .0'  ;'    .:c:. l'..       .'dNMMMMMKx.",
            "{c1}  o.     'KNMMMWd. .'     .c0WMMMMMMKo",
            "{c1}    ...   .0MMMMMMWc.     .lMMMMMMMK;",
            "{c1}      '.   ;MMMMMMMW,    'lMMMMMMMd,.",
            "{c1}        .   .XMMMMMMX     .;MMMMMM0.",
            "{c1}          .   :MMMMMMO       'dMMMX;",
            "{c1}            .   .dWMMMo         ;N;",
            "{c1}              .    :WMM;            .",
            "{c1}                    :Md.",
        ],
    }
}

fn centos() -> AsciiArt {
    AsciiArt {
        colors: vec![3, 2, 4, 5],
        lines: vec![
            "                 {c1}..",
            "               {c1}.PLTJ.",
            "              {c1}<><><><>",
            "     {c2}KKSSV' 4KKK {c1}LJ{c4} Kdistribution,",
            "     {c2}KKV' 4KKKKK {c1}LJ{c4} KK V{c3}KKKKKBS",
            "     {c2}V' ' 'VKKKK {c1}LJ{c4} KKKS{c3}VKBSV",
            "     {c2}.   =.VKKK {c1}LJ{c4} KKKK{c3}.'",
            "     {c2}|{c1}     KKKKK {c1}LJ{c4} KKKK{c3}.  |",
            "     {c2}|{c1}   . 'VKKK {c1}LJ{c4} KKKK{c3}.. |",
            "     {c2}|{c1}     .KKKK {c1}LJ{c4} KKKKV{c3}.. |",
            "     {c2}|{c1}       KKV {c1}LJ{c4} KKV   {c3}  |",
            "     {c2}|{c1}         ' {c1}LJ{c4}        {c3}  |",
        ],
    }
}

fn rocky() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}         __wgliliiligw_,",
            "{c1}      _williiiiiiiiiiiiiilw,",
            "{c1}    _iiiiiiiiiiiiiiiiiiiiiiii_",
            "{c1}   ,iiiiiiiiiiiiiiiiiiiiiiiiiii.",
            "{c1}  _iiiiiiiiiii{c2}AAAAAAA{c1}iiiiiiiiiii_",
            "{c1} diiiiiiiii{c2}AAAAAAAAA{c1}iiiiiiiiiiii.",
            "{c1} iiiiiiiiiii{c2}AAAAAA{c1}iiiiiiiiiiiiiii.",
            "{c1} iiiiiiiiii{c2}AAAAAA{c1}iiiiiiiiiiiiiiii.",
            "{c1} iiiiiii{c2}AAAAAAAAAAAAAAA{c1}iiiiiiiiii.",
            "{c1} .iiiii{c2}AAAAAAAAAA{c1}iiiiiiiiiiiiiii",
            "{c1}  .iiii{c2}AAA{c1}iiiiiiiiiiiiiiiiiiiii.",
            "{c1}    .iiiiiiiiiiiiiiiiiiiiiiiii.",
            "{c1}      .iiiiiiiiiiiiiiiiii.",
        ],
    }
}

fn alma() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 3, 4, 2, 6],
        lines: vec![
            "{c1}         'c:.",
            "{c1}        lkkkx, ..       {c2}..   ,cc,",
            "{c1}        okkkk:ckkx'  {c2}.lxkkx.okkkkd",
            "{c1}        .:llcokkx'  {c2}:kkkxkko:xkkd,",
            "{c1}      .xkkkkdood:  {c2};kx,  .lkxlll;",
            "{c1}       xkkx.       {c2}xk'     xkkkkk:",
            "{c1}       'xkx.       {c2}xd      .....,.",
            "{c3}      .. {c1}:xkl'     {c2}:c      ..''...",
            "{c3}    .dkx'  {c1}.:ldl:'. {c2}'  {c4}':lollldkkxo;",
            "{c3}  .''lkkko'                     {c4}ckkkx.",
            "{c3}'xkkkd:kkd.       ..  {c5};'        {c4}:kkxo.",
            "{c3},xkkkd;kk'      ,d;    {c5}ld.   {c4}':dkd::cc,",
            "{c3} .,,.;xkko'.';lxo.      {c5}dx,  {c4}:kkk'xkkkkc",
            "{c3}     'dkkkkkxo:.        {c5};kx  {c4}.kkk:;xkkd.",
            "{c3}       .....   {c5}.;dk:.   {c5}lkk.  {c4}:;,",
            "             {c5}:kkkkkkkdoxkkx",
            "{c5}              ,c,,;;;:xkkd.",
            "{c5}                ;kkkkl...",
            "{c5}                ;kkkkl",
            "{c5}                 ,od;",
        ],
    }
}

fn slackware() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7],
        lines: vec![
            "{c1}                  ::::::::",
            "{c1}            :::::::::::::::::::",
            "{c1}         :::::::::::::::::::::::::",
            "{c1}       :::::::::{c2}cllcccccllllllll{c1}::::::",
            "{c1}    :::::::::{c2}lc               dc{c1}:::::::",
            "{c1}   ::::::::{c2}cl   clccclcc    oc{c1}:::::::::",
            "{c1}  ::::::::::{c2}o   lc{c1}::::::::{c2}co   oc{c1}::::::::::",
            "{c1}  :::::::::::{c2}o    cccclc{c1}:::::{c2}clcc{c1}::::::::::::",
            "{c1}  ::::::::::::{c2}lc        cclccclc{c1}:::::::::::::::",
            "{c1}  ::::::::::::::{c2}lcclcc          lc{c1}::::::::::::",
            "{c1}  ::::::::::{c2}cclcc{c1}:::::{c2}lccclc     oc{c1}::::::::::",
            "{c1}   :::::::::{c2}co{c1}::::::::{c2}clc    lc{c1}::::::::::",
            "{c1}    ::::::::::{c2}c     lcccc   co{c1}::::::::::::",
            "{c1}     :::::::::::{c2}lc           oc{c1}:::::::::",
            "{c1}       :::::::::::{c2}lccclclccclc{c1}::::::::::",
            "{c1}         :::::::::::::::::::::::::::::",
            "{c1}            ::::::::::::::::::::::::",
            "{c1}                ::::::::::::::::",
        ],
    }
}

fn zorin() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 6],
        lines: vec![
            "{c1}        `osssssssssssssssssssso`",
            "{c1}       .osssssssssssssssssssssso.",
            "{c1}      .+oooooooooooooooooooooooo+.",
            "",
            "{c1}  `://////////////+////////////////`",
            "{c1} `+ssssssssssssssssso{c2}+ssssssssssssssss+`",
            "{c1}.ossssssssssssssssss{c2}+osssssssssssssssso.",
            "{c1}ssssssssssssssso{c2}+/::+ossssssssssssssssss",
            "{c1}ssssssssss{c2}+/:::::::/+osssssssssssssssssss",
            "{c1}sssss{c2}+/::::::::::/+osssssssssssssssssssss",
            "{c1}sssssso{c2}+/:::::::/+ossssssssssssssssssssss",
            "{c1}ssssssssso{c2}+/:::/+osssssssssssssssssssssss",
            "{c1}sssssssssssso{c2}+/+ossssssssssssssssssssssss",
            "{c1}.ossssssssssssssssssssssssssssssssssssso.",
            "{c1} `+ssssssssssssssssssssssssssssssssssss+`",
            "{c1}  `://////////////////////////////////////////`",
        ],
    }
}

fn elementary() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7],
        lines: vec![
            "{c1}         eeeeeeeeeeeeeeeee",
            "{c1}      eeeeeeeeeeeeeeeeeeeeeee",
            "{c1}    eeeee  eeeeeeeeeeee   eeeee",
            "{c1}  eeee   eeeee       eee     eeee",
            "{c1} eeee   eeee          eee     eeee",
            "{c1}eee    eee            eee       eee",
            "{c1}eee   eee            eee        eee",
            "{c1}ee    eee           eeee       eeee",
            "{c1}ee    eee         eeeee      eeee",
            "{c1}ee    eee       eeeee      eeee",
            "{c1}eee   eeee   eeeeee      eeee",
            "{c1}eee    eeeeeeeeee      eeee",
            "{c1} eeee    eeeeeee     eeeee",
            "{c1}  eeee               eeee",
            "{c1}    eeeee          eeeee",
            "{c1}      eeeeeeeeeeeeeee",
            "{c1}         eeeeeeeeeee",
        ],
    }
}

fn mx() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 6],
        lines: vec![
            "{c1}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNM",
            "{c1}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMd..dM",
            "{c1}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNMMMo",
            "{c1}MMMMNddddddddddddddddddddddddo",
            "{c1}MMMh{c2}MMMMMMMMMMMMMMMMMMMMMMMMMM{c1}N.",
            "{c1}MMM{c2}MMMMMMMMMMMMMMMMMMMMMMMMMMMM{c1}d.",
            "{c1}MMM{c2}MMMM{c1}MN{c2}MMMMMMM{c1}MN{c2}MMMMMMMMMM{c1}d.",
            "{c1}MM{c2}MMMMM{c1}MMMN{c2}MMM{c1}MNN{c2}MMMMMMMMMMM{c1}d.",
            "{c1}MM{c2}MMMMM{c1}MMMMN{c2}M{c1}MNNM{c2}MMMMMMMMMM{c1}d.",
            "{c1}MM{c2}MMMMM{c1}MMMMNN{c2}{c1}MNNNM{c2}MMMMMMMMM{c1}d.",
            "{c1}MM{c2}MMMMM{c1}MMM{c2}M{c1}NNNNNNN{c2}MMMMMMMM{c1}d.",
            "{c1}MM{c2}MMMM{c1}M{c2}MMM{c1}NN{c2}M{c1}NNNNN{c2}MMMMMMM{c1}d.",
            "{c1}MMM{c2}MMMMMMM{c1}NNNM{c2}M{c1}NNNN{c2}MMMMMM{c1}d.",
        ],
    }
}

fn solus() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7, 1],
        lines: vec![
            "            {c2}-```````````",
            "          {c2}`+/----------.`",
            "         {c2}/+-            `.+/`",
            "        {c2}/s:              `.+/",
            "       {c2}/s-       ..       `/s",
            "      {c2}/s.     `.:++-`      .+s`",
            "     {c2}/s`    .:+++++++:`     `+s`",
            "    {c2}/s`   .:/++++++++++:`    `+s`",
            "   {c2}:s`   `:++++++++++++++:`   .+s:",
            "  {c2}:s`  `-/++++++++++++++++/-`  .oo:",
            " {c2}/s:  `-:++++++++++++++++++++:-` :ss:",
            " {c2}ss: `-/++++++++++++++++++++++++/-`.ss`",
            " ss` `-/+++++++++++++++++++++++++/-` ss`",
            " ss  `:++++++++++++++++++++++++++:`  ss",
        ],
    }
}

fn deepin() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}             ............",
            "{c1}         .';;;;;.       .,;,.",
            "{c1}      .,;;;;;;;.       ';;;;;;;.",
            "{c1}    .;::::::::'     .,::;;,'''',.",
            "{c1}   ,'.::::::::    .;;'.          ';",
            "{c1}  ;'  'cccccc,  .: ..;googl;.    ;'",
            "{c1} ,,    :ccccc.  ;: .;'googl';.   ';",
            "{c1}.l.     ;;;;;   ;. ;.googl  ;;   ;.",
            "{c1}.c       ,,,,,  ;. ;  ''''  ;;   ;.",
            "{c1}.l.          .  ;:  ;......;;.  ;'",
            "{c1} .;.     ...    ;,   '';;;;'.  ;'",
            "{c1}  ';.  ;'.      ;'          ,;'",
            "{c1}    ';'googl   ;',       ,;'",
            "{c1}      ';:;; ,googl   ,;;'",
            "{c1}          ';;googl:;;'",
        ],
    }
}

fn raspbian() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 1],
        lines: vec![
            "{c1}  `.::///+:/-.        --///+//-:`",
            "{c1} `+oooooooooooo:   `+oooooooooooo:",
            "{c1}  /oooo++//ooooo:  ooooo+//+oooo+.",
            "{c1}  `+ooooooo:-:oo-  +o+:ooooooo:``",
            "{c1}   `:oooooooo+``    `.oooooooo+-",
            "{c1}     `:++ooo/.        :+ooo+/.`",
            "{c2}        ...`googl      ..../googl",
            "{c2}     .googl        .googl",
            "{c2}    :googl        :googl",
            "{c2}   .googl        .googl",
            "{c2}   :oooo`        :oooo:",
            "{c2}   .oooo:        .oooo/",
            "{c2}    :oooo+      /oooo/",
            "{c2}     /oooo:.  .+oooo/",
            "{c2}      :ooooo++ooooo:`",
            "{c2}       `/ooooooooo+.",
            "{c2}         ./oooooo+`",
            "{c2}          ``-:/-``",
        ],
    }
}

fn freebsd() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7, 3],
        lines: vec![
            "   {c2}```                        {c1}`",
            "  {c2}` `.....---...{c1}....--.```   -/",
            "  {c2}+o   .--`         {c1}/y:`      +.",
            "  {c2} yo`:.            {c1}:o      `+-",
            "    {c2}y/               {c1}-/`   -o/",
            "   {c2}.-                  {c1}::/sy+:.",
            "   {c2}/                     {c1}`--  /",
            "  {c2}`:                          {c1}:`",
            "  {c2}`:                          {c1}:`",
            "   {c2}/                          {c1}/",
            "   {c2}.-                        {c1}-.",
            "    {c2}--                      {c1}-.",
            "     {c2}`:`                  {c1}`:`",
            "       .--             `--.",
            "          .---.....----.",
        ],
    }
}

fn openbsd() -> AsciiArt {
    AsciiArt {
        colors: vec![3, 7, 6, 1, 8],
        lines: vec![
            "                                     {c3}_",
            "                                    (_)",
            "{c1}              |    .",
            "{c1}          .   |L  /|   .          {c3}   _",
            "{c1}      _ . |\\ _| \\--+._/| .       {c3}  (_)",
            "{c1}     / ||\\| Y J  )   / |/| ./",
            "{c1}    J  |)'( |        ` F`.'/        {c3}_",
            "{c1}  -<|  F         __     .-<        {c3}(_)",
            "{c1}    | /       .-'{c3}. {c1}`.  /{c3}-. {c1}L___",
            "{c1}    J \\      <    {c3}\\ {c1} | | {c5}O{c3}\\{c1}|.-' {c3} _",
            "{c1}  _J \\  .-    \\{c3}/ {c5}O {c3}| {c1}| \\  |{c1}F   {c3}(_)",
            "{c1} '-F  -<_.     \\   .-'  `-' L__",
            "{c1}__J  _   _.     >-'  ){c1}._.   |-'",
            "{c1} `-|.'   /_.          \\_|   F",
            "{c1}   /.-   .                _.<",
            "{c1}  /'    /.'             .'  `\\",
            "{c1}   /L  /'   |/      _.-'-\\",
            "{c1}  /'J       ___.---'\\|",
        ],
    }
}

fn macos() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 3, 1, 5, 4, 6],
        lines: vec![
            "{c1}                 'c.",
            "{c1}                ,xNMM.",
            "{c1}              .OMMMMo",
            "{c1}              OMMM0,",
            "{c1}    .;loddo:' loolloddol;.",
            "{c1}  cKMMMMMMMMMMNWMMMMMMMMMM0:",
            "{c2} .KMMMMMMMMMMMMMMMMMMMMMMMWd.",
            "{c2} XMMMMMMMMMMMMMMMMMMMMMMMX.",
            "{c3};MMMMMMMMMMMMMMMMMMMMMMMM:",
            "{c3}:MMMMMMMMMMMMMMMMMMMMMMMM:",
            "{c4}.MMMMMMMMMMMMMMMMMMMMMMMMX.",
            "{c4} kMMMMMMMMMMMMMMMMMMMMMMMMWd.",
            "{c5} .XMMMMMMMMMMMMMMMMMMMMMMMMk",
            "{c5}  .XMMMMMMMMMMMMMMMMMMMMK.",
            "{c6}    kMMMMMMMMMMMMMMMMMMd.",
            "{c6}     ;KMMMMMMMWXXWMMMKo.",
        ],
    }
}

fn windows() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 7],
        lines: vec![
            "{c1}        ,.=:!!t3Z3z.,",
            "{c1}       :tt:::tt333EE3",
            "{c1}       Et:::ztt33EEEL{c2} @Ee.,      ..,",
            "{c1}      ;tt:::tt333EE7{c2} ;EEEEEEttt:::z.",
            "{c1}     :Et:::zt333EEQ.{c2} $EEEEEttt:::z.",
            "{c1}     it::::tt333EE7{c2} @EEEEEEttt:::z.",
            "{c1}    ;3=*^```\"*4EEV{c2} :EEEEEEttt:::z.",
            "{c1}    ,.=::::!t=., `{c2} @EEEEEttt:::z.",
            "{c1}   ;::::::::zt33){c2}   '4EEEttt:::z.",
            "{c1}  :t]]]]]]]]]33){c2}     $EEEEttt:::z.",
            "{c1}  :Et:::::::zt33){c2}      $EEEEttt:::z.",
            "{c1}  ;t:::::::tt333){c2}       $EEEEEttt;:z.",
            "{c1}  :::::::::zt33) {c2}         `4$$$$$$$",
        ],
    }
}

fn android() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}         -o          o-",
            "{c1}          +hydNNNNdyh+",
            "{c1}        +mMMMMMMMMMMMMm+",
            "{c1}      `dMM{c2}m:{c1}NMMMMMMN{c2}:m{c1}MMd`",
            "{c1}      hMMMMMMMMMMMMMMMMMMh",
            "{c1}  ..  yMMMMMMMMMMMMMMMMMMy  ..",
            "{c1}.mMMm`+MMMMMMMMMMMMMMMMMM+`mMMm.",
            "{c1}:MMMM-`MMMMMMMMMMMMMMMMMM`-MMMM:",
            "{c1}:MMMM-`MMMMMMMMMMMMMMMMMM`-MMMM:",
            "{c1}:MMMM-`MMMMMMMMMMMMMMMMMM`-MMMM:",
            "{c1}`mMMm   MMMMMMMMMMMMMMMMMM   mMMm`",
            "{c1}  ``  `MMMMMMMMMMMMMMMMMM`  ``",
            "{c1}      `MMMMMMMMMMMMMMMMMM`",
        ],
    }
}

fn rhel() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7],
        lines: vec![
            "{c1}           .MMM..:MMMMMMM",
            "{c1}          MMMMMMMMMMMMMMMMMM",
            "{c1}          MMMMMMMMMMMMMMMMMMMM.",
            "{c1}         MMMMMMMMMMMMMMMMMMMMMM",
            "{c1}        ,MMMMMMMMMMMMMMMMMMMMMM:",
            "{c1}        MMMMMMMMMMMMMMMMMMMMMMMM",
            "{c1}  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM",
            "{c1} MMMMMM    `MMMMMMMMMMMMMMMMMMMM.",
            "{c1}MMMMMMMM      MMMMMMMMMMMMMMMMMM .",
            "{c1}MMMMMMMMM.       `MMMMMMMMMMMMM' MM",
            "{c1}MMMMMMMMMMM.                     MMMM",
            "{c1}`MMMMMMMMMMMMM.                 ,MMMMM.",
            "{c1} `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.",
            "{c1}    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            "{c1}      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:",
            "{c1}       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            "{c1}         `MMMMMMMMMMMMMMMMMMMMMMMM:",
            "{c1}             ``MMMMMMMMMMMMMMM'",
        ],
    }
}

fn amazon_linux() -> AsciiArt {
    AsciiArt {
        colors: vec![3, 7],
        lines: vec![
            "{c1}             `-/oydNNdyo/-`",
            "{c1}          .+mNNNNNNNNNNNNNNm+.",
            "{c1}        :dNNNNNNNNNNNNNNNNNNNNd:",
            "{c1}      .mNNNNNNNNNNNNNNNNNNNNNNNm.",
            "{c1}     oNNNNNNNNNNNNNNNNNNNNNNNNNNNo",
            "{c1}    +NNNNNNNNNNNNNNNNNms+//+dNNNNN+",
            "{c1}   :NNNNNNNNNNNNNNNm+`       .dNNNN:",
            "{c1}   dNNNNNNNNNNNNNNs`           `sNNNd",
            "{c1}   NNNNNNNNNNNNNNN       {c2}.--.    {c1}NNNNN",
            "{c1}   dNNNNNNNNNNNNNd     {c2}/oooo:\\   {c1}dNNNd",
            "{c1}   :NNNNNNNNNNNNNNo.   {c2}\\oooo/  {c1}.oNNNN:",
            "{c1}    +NNNNNNNNNNNNNNNd+. {c2}`--` {c1}.+dNNNNN+",
            "{c1}     oNNNNNNNNNNNNNNNNNdso++osdNNNNNNo",
            "{c1}      .mNNNNNNNNNNNNNNNNNNNNNNNNNNNm.",
            "{c1}        :dNNNNNNNNNNNNNNNNNNNNNNNd:",
            "{c1}          .+mNNNNNNNNNNNNNNNNNm+.",
            "{c1}             `-/oydNNNNdyo/-`",
        ],
    }
}

fn chromeos() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 1, 3, 4, 7],
        lines: vec![
            "{c2}            .,:loool:,.",
            "{c2}        .,coooooooooooooc,.",
            "{c2}     .,lllllllllllllllllllll,.",
            "{c2}    ;ccccccccccccccccccccccccc;",
            "{c1}  '{c2}ccccccccccccccccccccccccccccc{c1}'",
            "{c1}  OOOOOOOOOOOOo{c2}ccccccc{c3}oOOOOOOOOOOO",
            "{c1}  OOOOOOOOOOOO{c2}lcccccccl{c3}OOOOOOOOOOOO",
            "{c1}  OOOOOOOOOOOO{c2}:ccccccc:{c3}OOOOOOOOOOOO",
            "{c1}  OOOOOOOOOOOOo{c2}ccccccc{c3}oOOOOOOOOOOO",
            "{c1}  OOOOOOOOOOOOOo{c2}ccccc{c3}oOOOOOOOOOOOO",
            "{c1}  OOOOOOOOOOOo{c4}OOOOo{c3}oOOOOOOOOOOO",
            "{c4}  OOOOOOOOOOOOOOOOO{c3}OOOOOOOOOOOO",
            "{c4}  OOOOOOOOOOOOOOOOOO{c3}OOOOOOOOOOO",
            "{c4}  OOOOOOOOOOOOOOOOOOO{c3}OOOOOOOOOO",
            "{c4}  OOOOOOOOOOOOOOOOOOOO{c3}OOOOOOOOO",
            "{c4}  ``oooooooooooooooo{c3}oooooooo``",
            "{c4}           ````````",
        ],
    }
}

fn bedrock() -> AsciiArt {
    AsciiArt {
        colors: vec![8, 2],
        lines: vec![
            "{c1}--------------------------------------",
            "{c1}--------------------------------------",
            "{c1}--------------------------------------",
            "{c1}---{c2}\\\\\\\\\\\\\\\\\\\\\\\\{c1}------{c2}////////////{c1}---",
            "{c1}----{c2}\\\\\\\\\\\\\\\\\\\\\\{c1}----{c2}///////////{c1}----",
            "{c1}-----{c2}\\\\\\\\\\\\\\\\\\\\{c1}--{c2}//////////{c1}-----",
            "{c1}------{c2}\\\\\\\\\\\\\\\\\\\\{c2}/////////{c1}------",
            "{c1}-------{c2}\\\\\\\\\\\\\\\\////////{c1}-------",
            "{c1}--------{c2}\\\\\\\\\\\\///////{c1}--------",
            "{c1}---------{c2}\\\\\\\\//////{c1}---------",
            "{c1}----------{c2}\\\\\\/////{c1}----------",
            "{c1}-----------{c2}\\\\////{c1}-----------",
            "{c1}------------{c2}\\///{c1}------------",
            "{c1}-------------{c2}\\//{c1}-------------",
            "{c1}--------------------------------------",
            "{c1}--------------------------------------",
            "{c1}--------------------------------------",
        ],
    }
}

fn clearlinux() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 3, 6, 2],
        lines: vec![
            "{c1}               ,,,,,,,,,,,,,",
            "{c1}          .,,,,,,,,,,,,,,,,,,,,,.",
            "{c1}       ,,,,,,,,,,,,,,,,,,{c2},,,{c1},,,,,,,",
            "{c1}     ,,,,,,,,,,,,,,,{c2},,,,{c3},,,,{c2},,,,{c1},,,,",
            "{c1}   ,,,,,,,,,,,,,,{c2},,,,,{c3},,,,,{c2},,,,,{c1},,,,",
            "{c1}  ,,,,,,,,,,,,,,{c2},,,,,{c3},,,,,{c2},,,,,{c1},,,,,",
            "{c1} ,,,,,,,,,,,,,{c2},,,,,,{c3},,,,,{c2},,,,,,{c1},,,,,",
            "{c1} ,,,,,,,,,,,,,{c2},,,,,,{c3},,,,{c2},,,,,,,{c1},,,,,",
            "{c1} ,,,,,,,,,,,,,{c2},,,,,,{c3},,,,{c2},,,,,,,{c1},,,,,",
            "{c1}  ,,,,,,,,,,,,{c2},,,,,,{c3},,,{c2},,,,,,,,{c1},,,,",
            "{c1}  ,,,,,,,,,,,,,{c2},,,,,,{c3},{c2},,,,,,,,{c1},,,,,",
            "{c1}   ,,,,,,,,,,,,,{c2},,,,,,,,,,,,,{c1},,,,,",
            "{c1}    ,,,,,,,,,,,,,{c2},,,,,,,,,,{c1},,,,,,",
            "{c1}      ,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "{c1}         ,,,,,,,,,,,,,,,,,,,,",
            "{c1}              ,,,,,,,,,,",
        ],
    }
}

fn crux() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 6, 7],
        lines: vec![
            "{c1}         odddd",
            "{c1}      odddddddddd",
            "{c1}    oddxkkkxxddddddd",
            "{c1}   odddddOOOOOOdddddd",
            "{c1}  xddddOOOOOOOOOOOdddd",
            "{c1}  xdddOOOOOkkOOOOOOdddd",
            "{c1}  ddddOOOOOOOOOOOOOOdddd",
            "{c1}  ddddOOOOkOOOOOOkOOOdddd",
            "{c1}  xdddOOOOOOOOOOOOOOOdddd",
            "{c1}  xdddOOOOOxxOOOOOOOOdddd",
            "{c1}   xdddOOOOOOOOOOOOOoddd",
            "{c1}    xddddddOOOOOOdddddd",
            "{c1}     xddddddddddddddd",
            "{c1}       xdddddddddd",
            "{c1}          xdddd",
        ],
    }
}

fn devuan() -> AsciiArt {
    AsciiArt {
        colors: vec![5, 7],
        lines: vec![
            "{c1}   ..,,;;;::;,..                  ",
            "{c1}          `':dkko:'.               ",
            "{c1}              `':okkd;.            ",
            "{c1}                 `':okkkd:.        ",
            "{c1}              `.;:ldkkkkkkkd:.     ",
            "{c1}          `-;dkkkkkkkkkkkkkkkko.   ",
            "{c1}       .;okkkkkkkkkkkkkkkkkkkkkko. ",
            "{c1}     -dkkkkkkkkkkkkkkkkkkkkkkkkkk-.",
            "{c1}   `kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd",
            "{c1}   okkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd",
            "{c1}   okkkkkkkk;'                     ",
            "{c1}   `kkkkkkk;  .. ..                 ",
            "{c1}    :kkkkkk;             .....      ",
            "{c1}     lkkkkk:                        ",
            "{c1}      `dkkk;                        ",
            "{c1}        `'kk.                       ",
            "{c1}           `                        ",
        ],
    }
}

fn dragonfly() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7],
        lines: vec![
            "{c1}              ,--,",
            "{c1}         _ ,'    '.",
            "{c1}  {c2}*{c1}    /\\ '/       \\",
            "{c1}       / /          \\",
            "{c1}       | |           |",
            "{c1}       \\ \\           |",
            "{c1}        '\\          /",
            "{c1}         |         (",
            "{c1}         |          '",
            "{c1}         )           \\",
            "{c1}        /   /`--------'\\",
            "{c1}       /   /            \\",
            "{c1}      /   /              \\",
            "{c1}     '---'                '",
        ],
    }
}

fn endless() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7],
        lines: vec![
            "{c1}         `:+yhmNMMMMNmhy+:`",
            "{c1}      -oNMMMMMMMMMMMMMMMMMMNs-",
            "{c1}    -yNMMMMMMMMMMMMMMMMMMMMMMMMy-",
            "{c1}   oMMMMMMMho/-....-/:shNMMMMMMMMo",
            "{c1}  yMMMMMy/.              `:yNMMMMMs",
            "{c1} yMMMMd:`                   `/NMMMM:",
            "{c1}+MMMMs`                       /NMMMN",
            "{c1}mMMMy`                         :MMMMm",
            "{c1}NMMMd`                          dMMMM",
            "{c1}NMMMh                           yMMMM",
            "{c1}mMMMN/                         /MMMMN",
            "{c1}+MMMMNo.                     .sMMMMM/",
            "{c1} yMMMMMMy:`               `:yMMMMMMd",
            "{c1}  sMMMMMMMMNyo/-.``.-:oydNMMMMMMMMo",
            "{c1}   :dMMMMMMMMMMMMMMMMMMMMMMMMMMMs:",
            "{c1}     :yNMMMMMMMMMMMMMMMMMMMMNy:",
            "{c1}       `:ohNMMMMMMMMMMMMNho:`",
            "{c1}            `-:/+oo+/:-`",
        ],
    }
}

fn exherbo() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7, 1],
        lines: vec![
            "{c2} ,",
            "{c2} OXo.",
            "{c2} NXdX0:",
            "{c2} KX  d0 .",
            "{c2} KX   N0.",
            "{c2} KX    N0.",
            "{c2} KX     O0.",
            "{c2} KX      O0",
            "{c2} KX     d00 ",
            "{c2} KX   .0X0o",
            "{c1}    k{c2}xkOOO0X0KKKK0OOOkxk{c1}k",
            "{c1}    Xd                     dX",
            "{c1}    Xd                     dX",
            "{c1}    Xd                     dX",
            "{c1}    Xd{c2} kx       0k       xk {c1}dX",
            "{c1}    XX{c2}  k0     0k     0k  {c1}XX",
            "{c1}     XX{c2}  k0   0k   0k  {c1}XX",
            "{c1}      XXx{c2} k0 0k k0 {c1}xXX",
            "{c1}         XXXXX",
        ],
    }
}

fn guix() -> AsciiArt {
    AsciiArt {
        colors: vec![3, 7, 6, 4, 1],
        lines: vec![
            "{c1} |.__          __.|",
            "{c1} |__ \\        / __|",
            "{c1}    \\ \\      / /",
            "{c1}     \\ \\    / /",
            "{c1}      \\ \\  / /",
            "{c1}       \\ \\/ /",
            "{c1}        \\__/",
            "{c1}        /  \\",
            "{c1}       / /\\ \\",
            "{c1}      / /  \\ \\",
            "{c1}     / /    \\ \\",
            "{c1}    / /      \\ \\",
            "{c1} .__/ /        \\ \\__.",
            "{c1} |__./          \\.__|",
        ],
    }
}

fn haiku() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c2}          :dc.",
            "{c2}       ,oouoo.",
            "{c2}     ,oouooouoo.",
            "{c2}    ouooo;{c1}oooooo{c2}oo",
            "{c2}   oo{c1}ooooooooooooo{c2}o",
            "{c2}   l{c1}oooooooooooooooo{c2}o",
            "{c2}   '{c1}ooooooooooooooooo{c2}b",
            "{c2}    '{c1}oooooooooooooooooo{c2}b",
            "{c2}     '{c1}ooooooooooooooooooo{c2}b",
            "{c2}       {c1}`oooooooooooooooo",
            "{c2}         {c1}`ooooooooooooo",
            "{c2}           {c1}`ooooooooo",
            "{c2}              {c1}`:oo;",
        ],
    }
}

fn hyperbola() -> AsciiArt {
    AsciiArt {
        colors: vec![8, 8],
        lines: vec![
            "{c1}                    .',",
            "{c1}                 '-:////;.",
            "{c1}              '-:////////;.",
            "{c1}           '-://////////////;.",
            "{c1}         '-://// - - -////////;.",
            "{c1}       '-://// -       -///////;.",
            "{c1}     '-://// -           -/////;.",
            "{c1}   '-://// -               -///;.",
            "{c1}  '-://// -                 -//;.",
            "{c1}  '-://// -                  -;.",
            "{c1}  ;/////;.",
            "{c1}  ;/////;.",
            "{c1}  ;/////;.",
            "{c1}  ;/////;.",
            "{c1}  ;/////;.",
        ],
    }
}

fn instantos() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 6],
        lines: vec![
            "{c1}                  .o.",
            "{c1}                   :oo.",
            "{c1}                 .dMMMd",
            "{c1}                :NMMMm.",
            "{c1}               oMMMMs",
            "{c1}             `dMMMN",
            "{c1}            :NMMMm`",
            "{c2}      .:oydmMMMMNdyo:.",
            "{c2}   :dNMMMMMMMMMMMMMMMNd:",
            "{c2} `sMMMMMMMMMMMMMMMMMMMMMs`",
            "{c2} dMMMMMMMMMMMMMMMMMMMMMMMd",
            "{c2} dMMMMMMMMMMMMMMMMMMMMMMMd",
            "{c2} `sMMMMMMMMMMMMMMMMMMMMMs`",
            "{c2}   :dNMMMMMMMMMMMMMMMNd:",
            "{c2}      .:oydNMMMNdyo:.",
        ],
    }
}

fn mageia() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 7],
        lines: vec![
            "{c1}        .°°.    ",
            "{c1}         °°   .°°.",
            "{c1}         .°°°. °°  ",
            "{c1}          .   .    ",
            "{c1}           °   .°  ",
            "{c1}       .°°°.°°.°°°.",
            "{c1}   .°°°°°°°°°°°°°°°°.",
            "{c1}  .°°°°°°°°°°°°°°°°°°.",
            "{c1}  .°°°°°°°°°°°°°°°°°°.",
            "{c1}   .°°°°°°°°°°°°°°°°.",
            "{c1}    .°°°°°°°°°°°°°°.",
            "{c1}      .°°°°°°°°°°.",
            "{c1}        .°°°°°°.",
            "{c1}           .°.",
        ],
    }
}

fn netbsd() -> AsciiArt {
    AsciiArt {
        colors: vec![5, 7],
        lines: vec![
            "{c1}                                  __,gnnnOCCCCCOonnn;.",
            "{c1}     _.,gnnnnnnnnNNOOO888888OONNnnn,..",
            "{c1}   :OOOOOOOOOOOONNNNNNNNNbbbbNNNNNN\\",
            "{c1}  /OOOOOOOOOOOONNNNNNNNNNbbbbbbbNNNN\\",
            "{c1} /OOOOOOOOOOOONNNNNNNNNNNNbbbbbbbNNNN\\",
            "{c1} |OOOOOOOO{c2}ggggnnnn{c1}NNNNNNNbbbbbbbbNNNN|",
            "{c1} |OOOOO{c2}ggg{c1}NNNNNNNN{c2}nn{c1}NNNNNbbbbbbbbNNNN|",
            "{c1} |OOO{c2}ggg{c1}NNNN  NNNNNN{c2}n{c1}NNNNbbbbbbbNNNN/",
            "{c1} \\OO{c2}gg{c1}NNNN      NNNNNN NNNNbbbbbbNNNN/",
            "{c1}  \\O{c2}g{c1}NNNNN       NNNNN NNNNNbbbbNNNN/",
            "{c1}   \\{c2}{c1}NNNNN        NNNNN  NNNNNNNNNN/",
            "{c1}     NNNNNN       NNNNN NNNNNbbII",
            "{c1}      NNNNNN     NNNNN",
        ],
    }
}

fn parabola() -> AsciiArt {
    AsciiArt {
        colors: vec![5, 7],
        lines: vec![
            "{c1}               `.:/ohmNMh",
            "{c1}            -+dNMMMMMMMMMN",
            "{c1}          -sNMMMMMMMMMMMMMs",
            "{c1}        `+NMMMMMMMMMMMMMMMm`",
            "{c1}       -hMMMMMMMMMMMMMMMMMMN-",
            "{c1}      /mMMMMMMMMMMMMMMMMMMMMs",
            "{c1}     +NMMMMMMMMMMMMMMNhso+:-`",
            "{c1}    /MMMMMMMMMMMMM+:.`",
            "{c1}   :MMMMMMMMMMMh-",
            "{c1}   NMMMMMMMMMd-",
            "{c1}   NMMMMMMMd.",
            "{c1}   NMMMMMN/",
            "{c1}   NMMMMN+",
            "{c1}   mMMMMo",
            "{c1}   yMMMd",
            "{c1}   `dMd`",
        ],
    }
}

fn parrot() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 7],
        lines: vec![
            "{c1}  `:oho/-`   `-/oho:`",
            "{c1} `ohooooho/-`-/ohooooho`",
            "{c1} ohooooooooho/ohooooooooho",
            "{c1} ohoooooooooooooooooooooho",
            "{c1} `ohooooooooooooooooooho`",
            "{c1}  `+hooooooooooooooooh+`",
            "{c1}    `:ohoooooooooooho:`",
            "{c1}       `:ohooooho:`",
            "{c1}          `:oo:`",
            "{c1}     {c2}__{c1}      {c2}_{c1}       {c2}__{c1}",
            "{c2}    / /___ _| |_ ___/ /_",
            "{c2}   / / __ `/  __/  _ / __\\",
            "{c2}  / / /_/ / /  /  __/ /_/ /",
            "{c2} /_/ .___/_/   \\___/\\__,_/",
            "{c2} /_/",
        ],
    }
}

fn peppermint() -> AsciiArt {
    AsciiArt {
        colors: vec![1, 7, 3],
        lines: vec![
            "{c2}              NNNNNNNNN",
            "{c2}         NNNN{c1}NNN{c2}NNNN{c3}NNN{c2}NNNN",
            "{c2}       NNN{c1}NNNNNN{c2}NN{c3}NNNNNN{c2}NNN",
            "{c2}     NNN{c1}NNNNNNNNN{c2}N{c3}NNNNNNNNN{c2}NNN",
            "{c2}    NN{c1}NNNNNNNNNN{c2}NN{c3}NNNNNNNN{c2}NNN",
            "{c2}   NN{c1}NNNNNNNNN{c2}NNNN{c3}NNNNNNN{c2}NNNN",
            "{c2}  N{c1}NNNNNNNN{c2}NNNNNN{c3}NNNNNN{c2}NNNNN",
            "{c2}  NNNNNNNNNNNNNNNNNNNNNNNNNNN",
            "{c2}  N{c3}NNNNNN{c2}NNNNNNNNNNN{c1}NNNNN{c2}N{c1}N{c2}N",
            "{c2}  N{c3}NNNNNNN{c2}NNNNNN{c1}NNNNNNNN{c2}NNN",
            "{c2}   N{c3}NNNNNNN{c2}NNNN{c1}NNNNNNNNN{c2}NN",
            "{c2}    N{c3}NNNNNNNN{c2}NN{c1}NNNNNNNNNN{c2}N",
            "{c2}     NNN{c3}NNNNNNNNN{c2}N{c1}NNNNNNNNN{c2}",
            "{c2}       NNN{c3}NNNNNN{c2}NN{c1}NNNNNN{c2}NNN",
            "{c2}         NNNN{c3}NNN{c2}NNNN{c1}NNN{c2}NNNN",
            "{c2}              NNNNNNNNN",
        ],
    }
}

fn porteus() -> AsciiArt {
    AsciiArt {
        colors: vec![6, 7],
        lines: vec![
            "{c1}     ``````````",
            "{c1}  ```````````````",
            "{c1} `````````````````..",
            "{c1} `````````````````::-",
            "{c1}  ```````````````:::-",
            "{c1}   ``````````````:::-",
            "{c1}    `````:::::::::::-",
            "{c1}     ```::::::::::::-",
            "{c1}      ``::;;;;;;::::-",
            "{c1}       `:;;;;;;;::::-",
            "{c1}        `;;;;;;;;:::-",
            "{c1}         `;;;;;;;;;:-",
            "{c1}          `;;;;::::::",
            "{c1}            `.::::::",
        ],
    }
}

fn postmarketos() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}                 /\\",
            "{c1}                /  \\",
            "{c1}               /    \\",
            "{c1}              /      \\",
            "{c1}             /  {c2}____  {c1}\\",
            "{c1}            / {c2}/    \\  {c1}\\",
            "{c1}           / {c2}/      \\  {c1}\\",
            "{c1}          / {c2}/   /\\   \\  {c1}\\",
            "{c1}         / {c2}/   /  \\   \\  {c1}\\",
            "{c1}        / {c2}/___/    \\___\\  {c1}\\",
            "{c1}       /                  \\",
            "{c1}      /                    \\",
            "{c1}     /                      \\",
            "{c1}    /________________________\\",
        ],
    }
}

fn puppy() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7],
        lines: vec![
            "{c1}           ,xq@@@@@@@px,",
            "{c1}        ,q@@@@@@@@@@@@@@@@p,",
            "{c1}      ,q@@@@@@@@@@@@@@@@@@@@@p,",
            "{c1}     q@@@@@@@@@@@@@@@@@@@@@@@@@@,",
            "{c1}    q@@@@{c2}AAAAA{c1}@@@@@@@@@@{c2}AAAAA{c1}@@@,",
            "{c1}   q@@@@ {c2}AAAA{c1}@@@@@@@@@@ {c2}AAAA{c1}@@@@",
            "{c1}   @@@@@       @@@@@@@@       @@@@@@",
            "{c1}   q@@@@@@@@@@@{c2}######{c1}@@@@@@@@@@@@@",
            "{c1}    q@@@@@@@@@@{c2}######{c1}@@@@@@@@@@@@",
            "{c1}     q@@@@@@@@@@@@@@@@@@@@@@@@@@,",
            "{c1}      'q@@@@@@@@@@@@@@@@@@@@@p'",
            "{c1}        'pq@@@@@@@@@@@@@@@qp'",
            "{c1}            'pq@@@@@@@qp'",
        ],
    }
}

fn pureos() -> AsciiArt {
    AsciiArt {
        colors: vec![2, 7],
        lines: vec![
            "{c1}      dmmmmmmmmmmmmmmmmd",
            "{c1}   dmmmmmmmmmmmmmmmmmmmmmd",
            "{c1}  dmmmmmmmmmmmmmmmmmmmmmmmmd",
            "{c1}  mmmmm{c2}NNNNNNNNNo{c1}mmmmmmmmmmm",
            "{c1}   mmm{c2}NNNNo.  oNNNNm{c1}mmmmmmmm",
            "{c1}    mm{c2}NNNo      dNNNm{c1}mmmmmmm",
            "{c1}    mm{c2}NNN        NNNN{c1}mmmmmmm",
            "{c1}    mm{c2}NNN        NNNN{c1}mmmmmmm",
            "{c1}    mm{c2}NNNo      dNNN{c1}mmmmmmmm",
            "{c1}   mmm{c2}NNNNo.  oNNNN{c1}mmmmmmmmm",
            "{c1}  mmmmm{c2}NNNNNNNNNo{c1}mmmmmmmmmmm",
            "{c1}  dmmmmmmmmmmmmmmmmmmmmmmmd",
            "{c1}   dmmmmmmmmmmmmmmmmmmmmmd",
            "{c1}      dmmmmmmmmmmmmmmmd",
        ],
    }
}

fn sabayon() -> AsciiArt {
    AsciiArt {
        colors: vec![4, 7, 6],
        lines: vec![
            "{c2}            ....",
            "{c2}         ..{c1}MMMMMMM{c2}..",
            "{c2}       .{c1}MMMM{c2};{c1}MMMMMMM{c2}.",
            "{c2}      .{c1}MMM{c2}:{c1}MMMM{c2};{c1}MMMM{c2}.",
            "{c2}     .{c1}MMM{c2}:{c1}MMMM{c2}:{c1}MMMMM{c2}.",
            "{c2}    .{c1}MMM{c2};{c1}:MMMM{c2}:{c1}MMMMMM{c2}.",
            "{c2}    {c1}MMM{c2};{c1};MMMM{c2}:{c1}MMMMMMM{c2}.",
            "{c2}   .{c1}MMM{c2}:{c1};MMMM{c2}:{c1}MMMMMMM{c2}:",
            "{c2}   .{c1}MMMM{c2}:{c1};MMM{c2}:{c1}MMMMMMMM{c2}.",
            "{c2}   .{c1}MMMMMM{c2}::{c1}MMMMMMMMMM{c2}.",
            "{c2}    {c1}.MMMMMMMMMMMMMMMMMM{c2}.",
            "{c2}      {c1}`MMMMMMMMMMMMMM{c2}'",
            "{c2}         {c1}``MMMMMM{c2}''",
        ],
    }
}

fn slitaz() -> AsciiArt {
    AsciiArt {
        colors: vec![3, 3],
        lines: vec![
            "{c1}        @    @(               @",
            "{c1}      @@   @@  @@          @@ @@",
            "{c1}     @@   @@   @@   @@@  @@ @ @@",
            "{c1}    @@  @@    @@  @@   @@   @@ @@",
            "{c1}   @@  @@@@@@@@ @@         @@ @@",
            "{c1}  @@       @@  @@    @@@@  @@ @@",
            "{c1} @@       @@  @@   @@   @@ @  @@",
            "{c1}         @@   @@@@  @@@@ @ @@@@@@",
