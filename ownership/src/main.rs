fn main() {
  // Contrary to string literals (which are immutable), values from the String type are, since they are allocated in the heap
  // as such, you can modify the value in the heap
  let mut s = String::from("Hello, ");
  s.push_str("world!");
  println!("{}", s);

  // In Rust, the Copy trait is automatically implemented for types that have a known fixed size
  // Here, number values are automatically copied from one variable to another if you assign one variable to another
  let x = 5;
  let y = x;
  println!("{}", y);
  // On the contrary, String values don't work quite the same
  // Here, you get a reference or pointer to a memory address, and variable s2 points to the same address as s1
  let s1 = String::from("Hello");
  let mut s2 = s1;

  // Rust frees memory when a variable goes out of scope.
  // In the example above, as the memory address reference gets copied into the stack for s2,
  // if s1 goes out of scope, Rust would have to free the same memory twice. This is known as a
  // double free error. But, Rust has something called a "move" in the ownership system for managing
  // memory. A "move" is basically a way that Rust uses to make a variable invalid and usable no longer. This way,
  // the compiler knows that the memory address "moved" to another variable makes such memory freed only when the current owner
  // of that address goes out of scope, not freed the number of times that memory address gets copied. This way, you do not
  // have to worry about freeing resources the right amount of times.

  // For functions, ownership also has a stake.
  // When you use a variable that has the Drop trait as an argument for a function (which is something that surely is stored in the heap and therefore,
  // has a memory address), that memory address is "moved" to the function, so the function owns it, rather than the variable.
  // Contrary to invalidation, for these cases, the function has the capability to return the ownership.
  // Move can be reversed when the current owner returns the ownership to its original owner.
  // The following example on functions showcases it pretty well
  fn takes_ownership_and_gives_back(mut string: String) -> String {
    println!("I'm taking ownership of \"{}\" as a string!", string);
    string.push_str(" I've been modified!");
    return string;
  }

  s2 = takes_ownership_and_gives_back(s2);
  println!("{}", s2);

  // Obviously, working in this way can be cumbersome or tedious sometimes.
  // Rust has references as well, and using references does not take ownership of a variable's value in any way.
  // That means no move mechanics come into play, and you can keep using the variable's value at please,
  // using the proper syntax, ofc. Also, you can mutate the value by dereferencing using the dereference operator, not involving
  // move mechanics as well. This is known as borrowing.
  fn calc_str_len(string: &String) {
    println!("String's length is {}", string.len());
  }

  fn check_str(string: &mut String) {
    string.push_str(" Checked!");
  }

  calc_str_len(&s2);
  s2 = String::from("I got read without losing ownership and got modified!");
  check_str(&mut s2);
  println!("{}", s2);
}
