// For testing, we have macros that help up use the "cargo test" command and let Rust
// know we'd like to test the code in the module "tests"
// This is done by using the #[cfg(test)] macro for the module
// and the #[test] macro for the function performing the test

#[cfg(test)]
mod tests {
  // The "tests" module follows the same convention we've seen in the previous lessons for modules
  // We can use "super" to refer to the main scope here and refer to the "shapes" module to use it.
  use super::guessing;
  use super::shapes;
  #[test]
  fn another_name() {
    // The assert_eq! macro helps us asserting if a value equals another
    assert_eq!(2 + 2, 4);
  }

  // #[test]
  // fn panic_test() {
  //   panic!("I died!");
  // }

  #[test]
  fn test_rects() {
    let rect1 = shapes::Rectangle { x: 5, y: 5 };
    let rect2 = shapes::Rectangle { x: 3, y: 3 };
    // Another useful macro for asserting truthiness is "assert!()", which will panic if the operation result is "false"
    assert!(shapes::can_hold(&rect1, &rect2));
    assert!(!shapes::can_hold(&rect2, &rect1));
    // One thing about the assert_eq! macro (and it's counterpart "assert_ne!") is that, if the test fails, it will
    // indicate the values that were compared in both sides of the equation:
    // This test will fail and let you know what value did the left and right side had
    // assert_eq!(shapes::can_hold(&rect1, &rect2), false);
    // This is useful to get insights as to how our code got to those values in the first place so you'll be able
    // to track the value back to its origin and fix the bugs appropriately
    // The values for the equation in Rust are known as "left" and "right"
    // This seems much more straightforward as to how are we thinking and how we wrote the test in the first place,
    // so our mental model expectancies are in the order we expect them to be.
    // One thing to notice is that, the failing scenario of assertion will print the values using debug formatting,
    // which means that, if you intend to compare struct and enum values in your tests, they'll need to implement
    // the "PartialEq" and the "Debug" traits before actually being able to use them

    // You can also add custom messages to your assertions
    // The second argument of the assertion macros in the same fashion as the "format!" macro does,
    // And the arguments following that are the values placed in the formatting string
    // The following test will fail
    // assert_eq!(
    //   true, false,
    //   "This is not truthy comparison! Value on the right: {}",
    //   false
    // );
  }

  // You can also test for failure with the "#[should_panic]" attribute on test functions
  // The following tests the panic scenario for our Guess game lessons back in the past
  // Checking that we are not able to create a struct of our Guess struct with a value no
  // less than 1 and no greater than 100
  #[test]
  #[ignore]
  #[should_panic]
  fn test_guess() {
    let guess = guessing::Guess::new(200);
  }
  // However, this test is not as helpful
  // If we encountered with a panic situation but was not the one we were expecting, we wouldn't know for sure

  // So, our "#[should_panic]" attribute accepts and argument called "expected" that will let us know with a message
  // what is the context under which we expected the test to fail
  // In this case, an expected value in between and equal to 1 or 100
  #[test]
  #[ignore]
  #[should_panic(expected = "Should contain a value between 1 and 100")]
  fn test_guess_with_expected_msg() {
    // If this test was to panic for another reason than the one expected (in terms of the panic! macro message),
    // the test will fail, regardless of the test panicking.
    let guess = guessing::Guess::new(200);
    println!("Guess value: {}", guess.value);
  }

  #[test]
  fn test_guess_print_output() {
    let guess = guessing::Guess::new(100);
    println!("Guess value: {}", guess.value);
  }

  // We can also write tests that return "Result" instead of panicking in the test
  // or using the assertion macros
  // These type of tests allow us to use the question mark operation to early fail on the test if
  // any operation of the test fails
  // However, we cannot use the "#[should_panic]" macro for these
  // Instead, we should use the assertion macro to check if the "Result" type value of the operations
  // contains the "Err" variant, with "value.is_err()"
  #[test]
  #[ignore]
  fn test_with_result() -> Result<(), String> {
    if 2 + 3 == 4 {
      Ok(())
    } else {
      Err(String::from("Sum does not equal 4"))
    }
  }
}

pub mod shapes {
  pub struct Rectangle {
    pub x: i32,
    pub y: i32,
  }

  pub fn can_hold(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.x > rect2.x && rect1.y > rect2.y
  }
}

pub mod guessing {
  pub struct Guess {
    pub value: i32,
  }

  impl Guess {
    pub fn new(value: i32) -> Self {
      if value < 1 || value > 100 {
        panic!("Should contain a value between 1 and 100");
      }
      Guess { value }
    }
  }
}

pub mod file_check {
  use std::fs::File;
  use std::io::Error;
  // pub fn read_from_file() -> Result<(), Error> {
  //   let handle = File::open("test.txt")?;
  //   Err("Could not read from file")
  // }

  // pub fn write_to_file(string: &str) -> Result<(), Error> {
  //   Err("Could not write to file")
  // }
}
