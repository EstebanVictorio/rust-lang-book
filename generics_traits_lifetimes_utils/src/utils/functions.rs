use std::cmp::PartialOrd;

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
