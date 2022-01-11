use std::env;

mod constants;

use crate::constants::{colours, CARD, CONTENT, FOUR, HELLO, NL};
use ansi_term::Colour::Fixed;

fn main() {
    // collect arguments
    let args: Vec<String> = env::args().collect();

    let lines: [String; 6] = [
        [NL, &HELLO.join(NL), NL].concat(),
        [FOUR, "My name is ", CARD.name, ".", NL].concat(),
        [FOUR, CONTENT[0], "\n\n", FOUR, CONTENT[1], NL].concat(),
        // [FOUR, CONTENT[2], " ", CARD.title, " @ ", CARD.company, NL].concat(),
        CONTENT[3].to_string(),
        [
            FOUR,
            "  → ",
            &CARD
                .sites
                .as_formatted_list(CARD.handle)
                .join(&["\n\n", FOUR, "  → "].concat()),
            NL,
        ]
        .concat(),
        [FOUR, "[ npx ", CARD.handle, " ]"].concat(),
    ];

    let card = || {
        lines.iter().enumerate().for_each(|(index, line)| {
            if index == 0 {
                println!("{}", Fixed(51).bold().paint(line));
            } else {
                println!("{}", line);
            };
        })
    };

    match args.len() {
        1 => {
            card();
        }
        2 => {
            let arg = &args[1];
            match arg.as_str() {
                "colours" => colours(),
                "colors" => colours(),
                _ => println!(
                    "This is not a valid argument. please use none or `colours` {:?}",
                    arg.as_str()
                ),
            }
        }
        _ => {
            card();
        }
    }
}
