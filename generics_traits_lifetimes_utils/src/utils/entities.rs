use std::fmt::{Display, Formatter, Result};

// Here we define a struct with a generic type
// This one here will be created by using the same method of creating a new struct
// But this time, instead of just putting whatever we want, the moment we assign a value to a property
// in the creation of the struct, we are restricting the type of the other properties that also
// are of the generic Type
// Such as:
// Correct: let integer = Point { x: 5, y: 10 };
// Incorrect: let integer = Point { x: 5, y: 1.0 };
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

// If you want different generic types for structs, you can also declare more than one generic type argument
// and assign it properly to each required property
pub struct SecondPoint<T, U> {
  pub x: T,
  pub y: U,
}

// We can also use generics over methods on types to avoid duplication of implementations as well
impl<T, U> SecondPoint<T, U> {
  // This definition will allow us to return whatever type was assigned for our SecondPoint impl
  fn x(&self) -> &T {
    &self.x
  }
}

impl Display for Point<i32> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "x info:{}, y info: {}", self.x, self.y)
  }
}

// We can use concrete definitions at the same type, so, that is not restricted
// Just keep in mind that we are restricting the type used for the actual implementation
impl Point<i32> {
  pub fn abs_x_diff_from_point(&self, another_point: &Point<i32>) -> i32 {
    let diff: i32 = self.x - another_point.x;
    diff.abs()
  }
}

impl PointInfo for Point<i32> {
  fn info(&self) {
    println!("Point<i32> info - x:{}, y: {}", self.x, self.y);
  }
}

impl PointInfo for Point<char> {
  fn info(&self) {
    println!("Point<char> info - x:{}, y: {}", self.x, self.y)
  }
}

// There's also the possibility to mix generic types due to the fact that
// it's possible to declare impl generic types and type method generic types as follows:

pub struct PointMixCreator<T1, U1> {
  pub x: T1,
  pub y: U1,
}

impl<T1, U1> PointMixCreator<T1, U1> {
  // This definition will allow us to return whatever type was assigned for our SecondPoint impl
  pub fn new_from_another_point<T2, U2>(
    self,
    another_point: PointMixCreator<T2, U2>,
  ) -> PointMixCreator<T1, U2> {
    PointMixCreator {
      x: self.x,
      y: another_point.y,
    }
  }
}

// impl<T, U> PointInfo<T, U> for PointMixCreator<T, U> {
//   fn info(&self) {
//     println!("PointMixCreator info - x:{}, y: {}", self.x, self.y);
//   }
// }

// The same as above happens with the Option and Result types
// Both have generic types over their assigned value definitions, so you can, practically speaking,
// Use whatever value you'd like:
//
// enum Option<T,E> {
//  Some(T),
//  None,
//}
//
//
//
// enum Result<T,E> {
//  Ok(T),
//  Err(E),
//}
//
//

// There is also a way for us to define shared behavior for different types
// That feature is called "Trait"
// Very similar to impl, but this time, we do not define the behavior until the actual implementation on a type
pub trait PointInfo {
  // fn info(&self) -> ()
  // We can also define a default behavior for an existing function, but that does not prevent
  // us from writing behavior on our implementation. The impl one will just override the default one
  // Just keep in mind that if we override the default one, we cannot call it unless we remove the override part
  fn info(&self) {
    println!("I'm a PointInfo!")
  }
  // We can also define the behavior right away into our trait instead of defining it into their implementations
  // This helps on creating a generalized behavior of the function
  fn description(&self) {
    println!("I'm a PointInfo!")
  }
}

// We can also conditionally implement traits for generic types by setting a trait bound to the generic type
// Take the following example: we can call the method "to_string()" on any type as long as they implement the Display
// trait. This is implemented in the standard library, and these implementations are known as blanket implementations.
// These are widely used in the Rust standard library.
//
// impl<T: Display> ToString for T {
// --snip--
//
// Hence, we can do the following on, say, integers:
// let s = 3.to_string();

// Going further, if you were to try to return a reference of a function's local value
// even when specifying lifetime generic parameters, the compiler won't let you pass
// since the lifetime of the returned value is not related with the lifetime of the references
// passed into the function, because the reference gets dropped as soon as the function ends
// pub fn longest_2<'a>(str1: &'a str) -> &'a str {
//   let a = String::from("a");
//   a.as_str()
// }
// Remember, we cannot modify the lifetime of references, just let the compiler know how the references
// relate to each other in terms of their own lifetime
// Lifetime Generic parameters are more a way to ensure operations on lifetimes rather
// than being able to modify lifetimes.

// The same logic happens for lifetimes in structs
pub struct Super<'a> {
  pub message: &'a String,
}

// For structs, showcasing the third rule of lifetimes,
// here we add a Struct that has a lifetime parameter in the type declaration, which is needed for declaration.
// Then we see two references, &self and message, and one reference in the return type
// Because of the third rule of the lifetime mechanism (ellision), we do not need to indicate
// the lifetime parameters for this method, as the lifetime parameter for the return types is the
// same as the &self lifetime.
// TODO: Add the static lifetime lesson next
impl<'a> Super<'a> {
  pub fn super_info(&self, message: &str) -> &str {
    println!("{}", message);
    self.message
  }
}

// So, by specifying lifetimes in struct, we ensure that the references in the struct live as long as the
// structs themselves.
