pub struct Sites<'a> {
  pub twitter: &'a str,
  pub npm: &'a str,
  pub github: &'a str,
  pub crates: &'a str
}

pub struct Card<'a> {
  pub name: &'a str,
  pub title: &'a str,
  pub handle: &'a str,
  pub company: &'a str,
  pub sites: Sites<'a>
}
