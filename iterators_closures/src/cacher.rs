use std::collections::HashMap;

pub struct Cacher<T>
where
  T: Fn(u8) -> u8,
{
  calculation: T,
  value: HashMap<u8, u8>,
}

impl<T> Cacher<T>
where
  T: Fn(u8) -> u8,
{
  pub fn new(calculation: T) -> Self {
    Cacher {
      calculation,
      value: HashMap::new(),
    }
  }

  pub fn value(&mut self, arg: u8) -> u8 {
    let exists = self.value.get(&arg);
    match exists {
      // TODO: verify in the hashmap if the value has been calculating thru its key
      Some(v) => *v,
      None => {
        let v = (self.calculation)(arg);
        self.value.entry(arg).or_insert(v);
        v
      }
    }
  }
}
