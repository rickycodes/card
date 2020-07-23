extern crate ansi_term;

use ansi_term::Colour::Fixed;

use crate::Card;

pub fn print(card: Card) {
  let protocol = "https://";
  let npm_path = ["~", &card.handle].concat();
  let instructions = ["npx ", &card.handle].concat();

  let blue = 51;
  let pink = 1;
  let fixed_blue = Fixed(blue);
  let fixed_pink = Fixed(pink);
  let fixed_blue_bold = fixed_blue.bold();
  let fixed_pink_bold = fixed_pink.bold();

  let edge = fixed_pink.paint("│");
  let top = format!("{}{}{}", "╭", ["─"; 64].join(""), "╮");
  let bottom = format!("{}{}{}", "╰", ["─"; 64].join(""), "╯");

  println!("
  {}
  {}                                                                {}
  {}                 {} / {}                      {}
  {}                                                                {}
  {}          {}  {} @ {}                  {}
  {}                                                                {}
  {}       {}  {}{}{}                 {}
  {}           {}  {}{}{}                     {}
  {}        {}  {}{}{}             {}
  {}        {}  {}{}{}                  {}
  {}           {}  {}{}                            {}
  {}                                                                {}
  {}          {}  {}                                 {}
  {}                                                                {}
  {}
  ",
  fixed_pink.paint(top),
  edge,
  edge,
  edge,
  fixed_blue_bold.paint(card.name),
  fixed_pink_bold.paint(card.handle),
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("Work:"),
  card.title,
  card.company,
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("Twitter:"),
  protocol,
  card.sites.twitter,
  fixed_pink.paint(card.handle),
  edge,
  edge,
  fixed_blue_bold.paint("npm:"),
  protocol,
  card.sites.npm,
  fixed_pink.paint(npm_path),
  edge,
  edge,
  fixed_blue_bold.paint("Crates:"),
  protocol,
  card.sites.crates,
  fixed_pink.paint(card.handle),
  edge,
  edge,
  fixed_blue_bold.paint("GitHub:"),
  protocol,
  card.sites.github,
  fixed_pink.paint(card.handle),
  edge,
  edge,
  fixed_blue_bold.paint("Web:"),
  protocol,
  fixed_pink.paint("ricky.codes"),
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("Card:"),
  fixed_pink.paint(instructions),
  edge,
  edge,
  edge,
  fixed_pink.paint(bottom)
  );
}
