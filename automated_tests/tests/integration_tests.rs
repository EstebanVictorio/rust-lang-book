// The "tests" folder serves the purpose of testing your code as if it were used by
// an external app, binary or library. The consumer side of it.
// This is what we know as integration tests.
// We don't need to annotate this code with the #[cfg(test)] attribute
// Rust already considers this as a test suite, so you'd only need to add your testing
// functions.

// If you need any setup initialization values for your code to have available
// when testing, you can add a file under tests/common/mod.rs
// Rust already knows this as a convention and won't take into account its execution output to the
// tests results
use automated_tests::guessing::Guess;

#[test]
#[should_panic]
fn it_fails() {
  let guess = Guess::new(200);
  println!("Invalid value: {}", guess.value)
}
