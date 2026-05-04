use ratatui::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Link<'a> {
    pub label: &'a str,
    pub url: &'a str,
}

pub struct Card<'a> {
    pub name: &'a str,
    pub handle: &'a str,
    pub title: &'a str,
    pub _company: &'a str,
    pub links: [Link<'a>; 3],
}

pub const CARD: Card = Card {
    name: "Ricky Miller",
    handle: "rickycodes",
    title: "Software Developer",
    _company: "MetaMask",
    links: [
        Link {
            label: "Website",
            url: "https://ricky.codes",
        },
        Link {
            label: "GitHub",
            url: "https://github.com/rickycodes",
        },
        Link {
            label: "bsky",
            url: "https://bsky.app/profile/ricky.codes",
        },
    ],
};

const LEN: usize = 3;
pub const CONTENT: [&str; LEN] = [
    "I write software on a clicky keyboard",
    "Sometimes fun TUI things like this!",
    "Learn more:",
];

pub fn colours() {
    for x in 1..255 {
        println!("\x1b[38;5;{x}m{x:>3} {}\x1b[0m", ["█"; 40].join(""));
    }
}

pub const HELLO: [&str; 4] = [
    "    |   |     |    |",
    "    |---|,---.|    |    .---.",
    "    |   ||---'|    |    |   |",
    "    '   '`---'`---'`---'`---',",
];

pub const BG: Color = Color::Rgb(18, 23, 27);
pub const SURFACE: Color = Color::Rgb(25, 31, 36);
pub const TEXT: Color = Color::Rgb(227, 222, 211);
pub const MUTED: Color = Color::Rgb(138, 150, 156);
pub const PRIMARY: Color = Color::Rgb(0, 216, 146);
pub const PRIMARY_ACTIVE: Color = Color::Rgb(0, 232, 158);
pub const SECONDARY: Color = Color::Rgb(0, 116, 85);
pub const ALERT: Color = Color::Rgb(255, 96, 96);

pub const LOL_BUTTON_LABEL: &str = "lol";
pub const DEFAULT_LOL_DIALOG_COUNT: usize = 32;
pub const MAX_LOL_DIALOG_COUNT: usize = 64;

pub const LOL_TITLES: [&str; 8] = [
    "intrusion.exe",
    "mainframe",
    "elite shell",
    "neural uplink",
    "cyber police",
    "darknet prompt",
    "gibson access",
    "trace route",
];

pub const LOL_MESSAGES: [&str; 9] = [
    "Proxy chain engaged.",
    "Decrypting the planet...",
    "Rerouting through Tokyo.",
    "Handshake with the mainframe.",
    "Satellite uplink is unstable.",
    "Downloading classified vibes.",
    "Backdoor installed successfully.",
    "The Gibson is under pressure.",
    "Someone set up us the bomb.",
];

pub const LOL_CONFIRM_TEXT: [&str; 10] = [
    "We are Anonymous",
    "We are Legion",
    "We do not forgive",
    "We do not forget",
    "Expect us",
    "Mess with the best",
    "Hack the planet",
    "Enhance that matrix",
    "Access granted",
    "Die like the rest",
];
