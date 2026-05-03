#[derive(Clone, Copy, Debug)]
pub struct Link<'a> {
    pub label: &'a str,
    pub url: &'a str,
}

pub struct Card<'a> {
    pub name: &'a str,
    pub handle: &'a str,
    pub title: &'a str,
    pub company: &'a str,
    pub links: [Link<'a>; 3],
}

pub const CARD: Card = Card {
    name: "Ricky Miller",
    handle: "rickycodes",
    title: "Software Developer",
    company: "MetaMask",
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
