use ansi_term::Colour;

// output ansi colours
pub fn colours () {
  for x in 1..255 {
    let s = format!("{} {}", x, ["█"; 60].join(""));
    println!("{}", Colour::Fixed(x).paint(s));
  }
}
