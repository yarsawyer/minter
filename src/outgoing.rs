use super::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Outgoing {
  Amount(Amount),
  InscriptionId(InscriptionId),
}

impl FromStr for Outgoing {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s.len() >= 66 {
      Self::InscriptionId(s.parse()?)
    } else if s.contains(' ') {
      Self::Amount(s.parse()?)
    } else if let Some(i) = s.find(|c: char| c.is_alphabetic()) {
      let mut s = s.to_owned();
      s.insert(i, ' ');
      Self::Amount(s.parse()?)
    } else {
        println!("s: {}", s);
      Self::Amount(s.parse()?)
    })
  }
}