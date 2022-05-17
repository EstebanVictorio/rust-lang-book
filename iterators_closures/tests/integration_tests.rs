use iterators_closures::cacher::Cacher;
use std::iter::Sum;
#[test]
fn it_checks_equality_on_memoization_successfully() {
  let mut cacher = Cacher::new(|val| val);
  let value_1 = cacher.value(1);
  let value_2 = cacher.value(2);
  assert_eq!(value_2, 2);
}

// For closures, we have an additional capability that differs from normal
// functions: they capture the environment they are defined at.
// What this means is that you can use variables from the scope these were
// defined, like the following test example:
#[test]
fn it_passes_equality_by_closure_env_capture() {
  let x = 4;
  let func = |z| z == x;
  let y = 4;
  assert!(func(y));
}

// Another feature that we have at our disposal is iterators
// Iterators work quite like what we build in other language in loops,
// such as for, while, do while, etc
// But we have "loop" over Iterators expecting to move forward into the iterator
// elements easily, compared to other languages that do not have them, building
// manually the iteration process over collections or lists, which may end up
// in bugs or unwanted behaviors.
// Here's an easy example of getting the first item of a vector through
// the "next()" method of an iterator created from the vector with the "iter()"
// method.
#[test]
fn it_checks_iter() {
  let vec_1 = vec![1, 2, 3];
  let mut iter = vec_1.iter();
  let value = iter.next().unwrap();
  assert_eq!(vec_1[0], *value);
}

// Apart from the "map()" adaptor, there's another to allow or disallow
// elements in a collection
// Such adaptor is the "filter()" method in an iterator, which takes a closure
// as an argument and should return a boolean value, letting the adaptor know
// if the element in regard is gonna be discarded from the produced iterator or
// not
#[test]
fn it_filters_out_odd_values() {
  let elements = vec![1, 2, 3, 4];
  // Here we produce an iterator that's gonna allow even values only
  let even_iter = elements.iter().filter(|n| *n % 2 == 0);
  let even_elements: Vec<&u8> = even_iter.collect();
  println!("{:?}", even_elements);
}

// You .can also create your own iterators by implementing the Iterator trait.
// The only thing required for the Iterator trait and the compiler to be happy
// with your code is implementing the "next()" method on your type:

struct Counter {
  pub val: u8,
}

impl Iterator for Counter {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    if self.val < 5 {
      self.val += 1;
      Some(self.val)
    } else {
      None
    }
  }
}

#[test]
fn it_counts_to_five_only() {
  let mut counter = Counter { val: 0 };
  let another_counter = Counter { val: 0 };
  let even_elements: Vec<u8> = another_counter.filter(|x| *x % 2 == 0).collect();
  counter.next();
  counter.next();
  counter.next();
  counter.next();
  counter.next();
  let last_val = counter.next();
  println!("Even elements: {:?}", even_elements);
  assert_eq!(last_val, None);
}

// The only requirement for the Iterator trait is the "next()" method
// Once implemented, we can use other methods in the trait
#[test]
fn it_uses_more_iter_methods() {
  let mut counter = Counter { val: 0 };
  let mut another_counter = Counter { val: 0 };

  // Here we use the "zip()" method which returns a tuple of the two iterators we
  // are consuming here, and then multiply both of the iterator values into a
  // single value iterator and get the "sum()" of it.
  let mut sum: u8 = another_counter.zip(counter).map(|(a, b)| a * b).sum();
  println!("Sum of the multiple iterator method usage: {:?}", sum);
  assert_eq!(sum, 55);
}

// For the ownership and borrowing mechanics of this, we can create different
// versions of an iterator that contains different types that can read or write,
// depending on the situation at hand, from the collections or lists:
// - "iter()" creates an iterator of immutable references.
// - "into_iter()" creates an iterator of owned values instead of references.
// - "iter_mut()" creates an iterator of mutable references.

// There is also another notion regarding iterators.
// There are two kinds of known "adaptors", which consume iterators in different
// ways:
// "Consuming adaptors" are plain adaptors that consume the iterators from
// beginning to end without anything else going on, such as "sum()"
// "Iterator adaptors" are  adaptors that create other iterators through some
// specific methods, such as "map(closure_here)".

// For iterator adaptors, we'll get a warning on the compiler telling us that
// there is an unused iterator. This is because iterators are lazy, and are not
// consumed until, well, they've been used, such as looping over them or calling
// "collect()" on them to create a list of the elements on the iterator.

// There are some differences to note in the example above according to the
// types of closures we have at our disposal: Fn, FnOnce and FnMut
// - The "Fn" type borrows the variables in the environment these were defined
// as immutable, so, readonly operations can be performed on these variables
// - The "Fn" type borrows the variables in the environment these were defined
// as immutable, so, readonly operations can be performed on these variables
// - The "FnMut" type borrows the variables in the environment these were defined
// as mutable, so, read and write operations can be performed on these variables
// - The "FnOnce" type takes ownership the variables in the environment these
// were defined in, instead of just borrowing them.

// An example of the move keyword would be the following, which will fail
// cause we are trying to use a variable after the closure took ownership of the
// variable "x":
// fn it_does_not_compile(){
//   let x = vec![1, 2, 3];
//   let func = move |z| z == x;
//   let y = vec![1, 2, 3];
//   println!("X usage: {:?}", x); <- cannot use, "x" belongs to closure "func"
// }
