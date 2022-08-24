// Although having the capability of having multiple owners, there's a downside to this.
// We can create reference cycles by pointing one reference to another when using Rc with RefCell.
// By referencing each other, the Rc counter can be in the scenario where the drop count never reaches zero, creating memory leaks.
// These are known as reference cycles.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Null,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Null => None,
        }
    }
}

fn main() {
    // First we create an Rc with a list and an empty null value
    // Then we check its count and the tail of it by the implementation of the ".tail()" method to conveniently get the second element
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Null))));
    println!("a initial rc count: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail());

    // After that, we create a "b" list with another value and a ".tail()" pointing to the first "a" Rc value
    let b = Rc::new(List::Cons(6, RefCell::new(Rc::clone(&a))));

    // Following with printing the count of both Rcs and the "b" tail
    println!("a rc count after b creation: {}", Rc::strong_count(&a));
    println!("b initial rc count: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    // Finally we check if the tail of "a" returns a "Some" value and if so, we make the tail of "a" point to "b"

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // In the end, we print both reference counts and check what their values are:
    println!("a rc count after changing a: {}", Rc::strong_count(&a));
    println!("b rc count after changing a: {}", Rc::strong_count(&b));

    // We can see that, after the program finishes, we end up with two references, effectively creating a memory leak
    // by not dropping the references existing when this "main()" function ends.
    // The dropping process goes as follows:
    // 1.- We have 2 on both reference counts because of their original variable references plus one referring each other.
    // 2.- b is dropped, which decreases one of the the Rc<List> count by 1
    // 3.- a is dropped, which decreases the other Rc<List> count by 1
    // 4.- "main()" ends with 1 reference remaining on both Rc<List> counts

    // If Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
