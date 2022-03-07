use std::fs::File;
use std::io::{BufRead, BufReader};

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
  // using the proper syntax, ofc. Also, you can mutate the value by using a mutable reference, not involving
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

  // Things to denote here is that, if we wanted to use a mutable reference to s2
  // at some point, and then, try to create a new mutable reference, in the same runtime context
  // (that is, create a new mutable reference while we still have another mutable reference in play), our code won't compile.
  // The same happens when we have 'n' immutable references to s2 and then try tp create a mutable one from s2 as well.
  // Compilation simply won't work cause:
  // 1. In the first case, neither of the mutable references are capable of preventing the mutation from one another. It is not expected and therefore, not allowed.
  // 2. In the second case, immutable references are supposed to be read-only, and these also do not prevent the mutations given in the mutable reference of s2.
  // These cases are known as "data races", which are very like "race conditions", but with data changing unexpectedly.
  // Rust prevents this from happening by not allowing more than one mutable reference and only one type of reference between immutable and mutable references.
  // On the flip-side, you can have as many as immutable references as you want, since these do not change data unexpectedly cause these are read-only.

  // OTOH, take a look at this:

  // fn dangle() -> &usize {
  //   let a: usize = 5;
  //   &a
  // }

  // The example above is a function that returns a reference to a value that was declared inside a function,
  // which owns the value, and is deallocated or freed when the function ends and the variable goes out of scope.
  // These are known as dangling references, which mean that a reference points to a value whose may have already been freed
  // or allocated somewhere else for another resource use.
  // Rust's compiler is smart enough to prevent this from happening, saving you from a fatal failure.

  // One last type that works as reference is the Slice.
  // A slice is no more than a reference that lets you pick and borrow a selection of contiguous sequence of elements in a collection,
  // rather than using the whole collection.

  // This case prints from index 0 to 1. In slice syntax (n..m), if you add a number at the beginning,
  // that'll be the starting index, while the number at the end will take into account one previous to that one.
  // That means it works from "start" to "excluding end".
  let nums: [usize; 5] = [1, 2, 3, 4, 5];
  println!("Printing from index 0 to 1");
  for elem in &nums[0..2] {
    println!("{}", elem);
  }

  // This case prints from index 2 to 5. In slice syntax (n..), if you add a number at the beginning,
  // but skip the end, it'll go through until the last element.
  println!("Printing from index 2 to 4");
  for elem in &nums[2..] {
    println!("{}", elem);
  }

  // This case prints from index 2 to 3. In slice syntax (n..m), you do not necessarily
  // need to begin from the first element, you can pick whatever section you want.
  println!("Printing from index 2 to 3");
  for elem in &nums[2..4] {
    println!("{}", elem);
  }

  // This case prints from index 2 to 1. In slice syntax (n..=m), you explicitly
  // state that the "end" element should be included in the iteration. So, not "exclusive end".
  println!("Printing all elements");
  for elem in &nums[0..=4] {
    println!("{}", elem);
  }

  // This case prints from index 0 to 5. In slice syntax (..), you
  // state that you just want all elements from start to end, including the final element.
  println!("Printing all elements again");
  for elem in &nums[..] {
    println!("{}", elem);
  }

  // Slices are not reserved for numbers. You can also use them in strings, and provide
  // a quite powerful mechanism to prevent you from mutating the data these slices point to,
  // since slices are references too, and therefore, follow the same rules.
  // If you were to have a function that obtains the first and the last word from file sentences for each line,
  // reading thru each line while storing the current line in a variable, then getting the first and the last word from that line,
  // and then tried to modify or clean the line from where the words were read by a slice, you'd get a compiler error, since
  // immutable references do not expect the data they point too to change (mutable reference/variable).
  // The following example demonstrates this clearly:
  let file = File::open("file.txt").unwrap();
  let mut buffer = BufReader::new(file);
  let mut line = String::new();
  let mut read = buffer.read_line(&mut line).unwrap();
  let mut line_num = 1;
  while read != 0 {
    let mut first_word = "";

    let bytes = line.as_bytes();

    for (index, &elem) in bytes.iter().enumerate() {
      if elem == b' ' {
        first_word = &line[0..index];
        break;
      }
    }

    let mut counter = bytes.len() - 1;
    let mut space_found: bool = bytes[counter] == b' ';
    let mut last_word = String::new();
    while !space_found && counter > 0 {
      last_word.push(bytes[counter] as char);
      counter -= 1;
      space_found = bytes[counter] == b' ';
    }

    let mut last_word_bytes = last_word.into_bytes();
    last_word_bytes.reverse();

    last_word = String::new();
    for elem in last_word_bytes {
      last_word.push(elem as char);
    }
    // line.clear(); <-- This line borrows `line` as a mutable borrow,...
    // ...which is not possible having two immutable borrows when mutating `first_word` assigned...
    // ...to the immutable reference in line 149

    println!("First word on line {}: {}", line_num, first_word);
    println!("Last word on line {}: {}", line_num, last_word);
    line.clear(); // However, it is safe to use it here, since there are no read-only...
                  // ... immutable references here.
    read = buffer.read_line(&mut line).unwrap();
    line_num += 1;
  }

  // There's also the feature of using string slices as parameters for receiving both a slice from a string literal or a String object,
  // like this:
  fn print_any_string_related_content(s: &str) {
    println!("String slice: {}", s);
  }

  print_any_string_related_content(&String::from("Hey! I'm a string!")[..]);
  print_any_string_related_content(&"Hey! I'm a string literal"[..]);
}
