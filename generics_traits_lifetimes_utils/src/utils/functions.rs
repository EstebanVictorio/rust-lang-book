use std::cmp::PartialOrd;
use std::fmt::Display;

// We wrote a function to avoid duplication on finding the largest number in a series of i32 numbers
pub fn get_largest(iter: &Vec<u32>) -> u32 {
  let mut largest = iter[0];
  for elem in iter {
    if elem > &largest {
      largest = *elem;
    }
  }
  largest
}

// Next, we'll write one that does the largest number thing above AND find the largest item in a list of chars
pub fn get_largest_item<T: PartialOrd + Copy>(iter: &[T]) -> T {
  let mut largest = iter[0];
  for &elem in iter {
    if elem > largest {
      largest = elem;
    }
  }
  largest
}

// Once again, we get the largest item in a number or char collection, but this time, using references
pub fn get_largest_item_ref<T: PartialOrd + Copy>(iter: &[T]) -> &T {
  let mut index = 0;
  let mut largest = &iter[index];
  for &elem in iter {
    if elem > *largest {
      largest = &iter[index];
    }
    index += 1;
  }
  largest
}

// Since we are receiving references and returning a reference, the compiler does not know which
// reference you are referring to when using these values and returning a reference.
// The compiler needs a generic lifetime parameter to know that the references we are using live long enough within
// the lifetime we indicate.
// By this, we do not modify the lifetime of the references we pass.
// Instead, we let the compiler know that we want to constraint the references that
// can be used in this function, so, when trying to use "longest()" with references that
// no longer exist is caught at compile time. The compiler first points out that it does not
// know when all the references are valid and enforces you to point out the lifetime of the references
// Then, when using your function, the compiler will let you know when the usage of your function
// is wrong according to the contract of the lifetime specified. This means that you will not be able to
// use this function with invalid references.
// This has to do more with the relationship between the references in the parameters in a function
// with the reference returned by the function.
// The relationship that exists here is described by the rules of lifetimes
// Historically, between Rust versions, there was a time where all lifetime parameters had to
// be explicit. But Rust maintainers found out that developers were writing the same lifetime code
// over and over again, so they created an inference mechanism to acknowledge when was this necessary
// and when not.
// Lifetime parameters are called input and output lifetimes, corresponding respectively to
// parameters and return types, so...
// Basically, as lifetime parameters apply fo "fn" and "struct" stagements,
// there are three rules that compose the explicit requirement for lifetime parameters:
// 1.- If there are multiple references, each reference gets its own lifetime parameter
// 2.- If there's one input lifetime, the output lifetime is assigned the same lifetime parameter
// 3.- If there are multiple input parameters, but one of them is &self or &mut self, that same lifetime
// is assigned to all output parameters. This one is cool because the method writing of impl blocks are
// cleaner. Not much lifetime symbols are needed.
pub fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  if str1.len() > str2.len() {
    return str1;
  }

  if str2.len() > str1.len() {
    return str2;
  }

  "both"
}

pub fn longest_with_custom_msg<'a, T>(str1: &'a str, str2: &'a str, custom_msg: T) -> &'a str
where
  T: Display,
{
  if str1.len() > str2.len() {
    println!("{}: {}", custom_msg, str1);
    return str1;
  }

  if str2.len() > str1.len() {
    println!("{}: {}", custom_msg, str2);
    return str2;
  }

  println!("{}: {}", custom_msg, "both");
  "both"
}
