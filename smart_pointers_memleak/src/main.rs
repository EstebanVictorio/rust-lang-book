// Although having the capability of having multiple owners, there's a downside to this.
// We can create reference cycles by pointing one reference to another when using Rc with RefCell.
// By referencing each other, the Rc counter can be in the scenario where the drop count never reaches zero, creating memory leaks.
// These are known as reference cycles.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

    // If we uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // A way to protect your programs from reference cycles is strongly related to the count
    // of references, in this case, the Rc type. Resources cannot be dropped unless the strong_count
    // from an Rc reaches zero. So, for that matter, Rust also has another smart pointer called Weak<T>.
    // Weak<T> are weak references, where once they are created, they do not increase the strong count
    // of an Rc. Instead, as these do not express any ownership relationship, they increase only the weak count
    // of an Rc. These type of smart pointer references are created when you call the "downgrade()" method on an Rc,
    // passing down a reference to the Rc.
    // The main difference of a weak reference from a normal Rc reference is that the weak count of a weak reference
    // does not need to be zero to be dropped and cleaned up once a program finishes.
    println!("---------");
    // To start an example of this, we'll create a tree with nodes that hold child nodes.
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()), // we'd also like to know the parent of a leaf, so we add it here starting as a weak reference pointing to nowhere
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // In this part, we set a parent to leaft, which is branch, and now it'll point to branch, not owning it, but by having a weak reference to it

    // And finally printing the references here we notice that we no longer have an infinite output,
    // but a finite one instead where we state the relationships of the proper ownerships
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    // Now, we'll explore how the different counts, both strong and weak ones, change depending on the references linked.

    // We'll create another another branch node which will be the new parent of the leaf node, but this time we'll create it in a scope
    {
        let branch_2 = Rc::new(Node {
            value: 6,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch_2);

        println!(
            "branch_2 strong count: {} | weak count: {}",
            Rc::strong_count(&branch_2),
            Rc::weak_count(&branch_2)
        );

        println!(
            "leaf | strong count: {}, weak count: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!(
        "leaf | strong count: {}, weak count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    // We should keep in mind that this exercise creates two references. This means that "children" are the strong references
    // While "parent" being the weak one. At some point in this specific scenario in the code (and not in the book),
    // The leaf variable is left with no "parent", but "branch" has "leaf" as part of its "children"

    // In the end, the weak references will be droppped regardless of their count, but ruled by the strong count of the Rc's involved
    // Even if we have, say, a weak count of a Weak reference of 3, if the main Rc reference gets to 0, those weak references will be dropped.
    // This way we ensure we do not have memory leaks and we're safe from overflowing the stack!
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
