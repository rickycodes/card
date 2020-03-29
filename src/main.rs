extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Colour::Fixed;
use std::env;

// output ansi colours
fn colours () {
  for x in 1..255 {
    let s = format!("{} {}", x, ["█"; 60].join(""));
    println!("{}", Colour::Fixed(x).paint(s));
  }
}

// output card
fn card () {
  let name = "Ricky Miller";
  let title = "Software Developer";
  let handle = "rickycodes";
  let company = "MetaMask";
  let protocol = "https://";
  let twitter = "twitter.com/";
  let npm = "npm.im/";
  let github = "github.com/";

  let npm_path = ["~", &handle].concat();
  let instructions = ["npx ", &handle].concat();

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
  fixed_blue_bold.paint(name),
  fixed_pink_bold.paint(handle),
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("Work:"),
  title,
  company,
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("Twitter:"),
  protocol,
  twitter,
  fixed_pink.paint(handle),
  edge,
  edge,
  fixed_blue_bold.paint("npm:"),
  protocol,
  npm,
  fixed_pink.paint(npm_path),
  edge,
  edge,
  fixed_blue_bold.paint("GitHub:"),
  protocol,
  github,
  fixed_pink.paint(handle),
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

fn main () {
  // collect args
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => {
      card();
    }
    2 => {
      let arg = &args[1];
      match arg.as_str() {
        "colours" => colours(),
        "colors" => colours(),
        _ => println!("This is not a valid argument. please use none or `colours`"),
      }
    }
    _ => {
      card();
    }
  }
}
