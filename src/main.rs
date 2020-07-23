use std::env;

mod card;
mod print;
mod colours;

use card::Card;
use card::Sites;
use print::print;
use colours::colours;

// output card
fn card () {
  let card = Card {
    name: "Ricky Miller",
    title: "Software Developer",
    handle: "rickycodes",
    company: "MetaMask",
    sites: Sites {
      twitter: "twitter.com/",
      github: "github.com/",
      npm: "npm.im/",
      crates: "crates.io/users/"
    }
  };

  print(card);
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
