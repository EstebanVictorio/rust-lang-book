pub struct Attributes {
  pub width: usize,
  pub height: usize,
}

impl Default for Attributes {
  fn default() -> Attributes {
    Attributes {
      width: 0,
      height: 0,
    }
  }
}
