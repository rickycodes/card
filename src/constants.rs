#[derive(Debug)]
pub struct Sites<'a> {
    pub web: &'a str,
    pub twitter: &'a str,
    pub npm: &'a str,
    pub github: &'a str,
    pub crates: &'a str,
}

impl Sites<'_> {
    pub fn as_formatted_list(&self, handle: &str) -> Vec<String> {
        let mut a = vec![];
        [self.web, self.twitter, self.github, self.npm, self.crates]
            .iter()
            .enumerate()
            .for_each(|(index, url)| {
                let mut owned: String = "https://".to_owned();
                owned.push_str(url);
                if index != 0 {
                    // owned.push('/');
                    owned.push_str(handle);
                }
                a.push(owned)
            });

        a
    }
}

pub struct Card<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub handle: &'a str,
    pub company: &'a str,
    pub sites: Sites<'a>,
}

pub const CARD: Card = Card {
    name: "Ricky Miller",
    title: "Software Developer",
    handle: "rickycodes",
    company: "MetaMask",
    sites: Sites {
        web: "ricky.codes",
        twitter: "twitter.com/",
        github: "github.com/",
        npm: "npm.im/~",
        crates: "crates.io/users/",
    },
};

const LEN: usize = 4;
pub const CONTENT: [&str; LEN] = [
    "I write software on a clicky keyboard",
    "Sometimes fun CLI things like this!",
    "I currently work as a",
    "    Learn more:\n",
];

use ansi_term::Colour;

// output ansi colours
pub fn colours() {
    for x in 1..255 {
        let s = format!("{} {}", x, ["█"; 60].join(""));
        println!("{}", Colour::Fixed(x).paint(s));
    }
}

pub const FOUR: &str = "    ";
pub const NL: &str = "\n";

pub const HELLO: [&str; 4] = [
    "    |   |     |    |",
    "    |---|,---.|    |    .---.",
    "    |   ||---'|    |    |   |",
    "    '   '`---'`---'`---'`---',",
];
