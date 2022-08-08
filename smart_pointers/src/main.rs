use std::boxed::Box;
use std::ops::Deref;

// In this lesson, we'll work with smart pointers
// These aren't hugely different from usual references that work as pointers to memory addresses as well
// But have some extra capabilities that let you work in a variety of diverse ways different from normal reference pointers

// One of the smart pointers that exist is a Box, which is a type written in the standard Rust library
// It basically allows to store values directly on the heap rather than the stack
// And what remains in the stack is the address that a specific value is located on

enum List {
    Cons(i32, Box<List>),
    Null,
}

fn print_cons(list: &List) {
    match list {
        List::Cons(integer, nested_list) => {
            println!("Integer: {}", integer);
            print_cons(nested_list);
        }
        List::Null => println!("End Of List!"),
    }
}

fn main() {
    // This is a trivial example, but helps us understand how we can manipulate the storing mechanism used in the Box
    // type directly storing something in the heap, like the number "5" for "b"
    let b = Box::new(5);
    println!("b = {}", b);

    // Another example is recursive types, where you'd like to contain a data structure or a type inside another one
    // This is more commonplace in structs, so, we'll write an example of that sort.
    // The following is a cons list, a data structure from Lisp, which means a constructor list that allows
    // to build a data structure that builds from a pair of values, which the first one is from a normal type and the
    // second one is a cons list, continuing so on and so forth recursively, stopping only by the definition of a "null"
    // type.
    // Let's use the definition of the Cons list above in the following example
    let list = List::Cons(
        5,
        Box::new(List::Cons(6, Box::new(List::Cons(7, Box::new(List::Null))))),
    );

    match list {
        List::Null => (),
        value => print_cons(&value),
    }

    // In the example above, we used "Box" which is a type of smart pointer that allows to store data in the heap
    // rather than the stack
    // Rust needs to know at compile time the size of the data types we're using
    // If we were to recursively use List only, it would go trying to allocate data for such
    // type that the compile process would break and the compiler would throw an error
    // that you are recursively trying to type it

    // By using a Box, you wrap the List type from the Cons enum type value that allows to tell the compiler
    // that we would like a pointer to a List type rather than a List, which is different, because the compilers
    // knows that the data in memory would sit "next to the integer" rather than "inside the" Cons type
    // The "Box" type also allows us,  by implementing the Deref trait, to use the dereference operator, because
    // we're creating, in the end, a pointer to some data. So, the following code snippets are equivalent:
    //
    //
    //
    // 1st block
    // let x = 5;
    // let y = &x;
    //
    // assert_eq(5,x);
    // assert_eq(5,*y);
    // ====================
    // 2nd block
    // let x = 5;
    // let y = Box::new(x);
    //
    // assert_eq(5,x);
    // assert_eq(5,*y);
    //

    // Now we use our custom type that works with the dereference operator is that
    // it implements the Deref trait, so it knows what to return as a reference from the value that our
    // CustomBox is holding
    let x = CustomBox::new(5);
    println!("{:?}", *x);
    // It's worth noting that the compiler will transform these deref expressions to the following:
    // NOTE: *(x.deref());
    // The reason the compilers does this has to do with ownership mechanics
    // Our CustomBox is a custom type that can, at reading level, seem to behave just like references,
    // but in reality, the compiler reads this and has the support of a custom type implementing the Deref
    // trait to know that this is a custom type implementing Deref. By this, it ensures that code
    // is interoperable between normal references and custom smart pointer references.
    // Since these are custom, in the end, these are not normal and has to transform these to this kind of statements
    // where they return a reference to the actual value using what we learned in chapter 5, where we use the first value
    // in a struct that has no named property, in this case, the first value "0" in "self".
    // Once the first value in a struct is obtained when no named property is defined, then the compilers dereferences it by calling
    // the "deref" method implemented, and that's how it obtains the actual value.
    // If this didn't happen, ownership of the value from the struct would be moved and we wouldn't be able to use the deref operator in a
    // custom smart pointer type.
    // The compiler is smart enough to infer this, so, the deref syntax in types that implement the Deref trait is possible
    // and interoperable to use.
}
// We can also create custom types that can work with the dereference operator!
// Let's do that:
struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> CustomBox<T> {
        CustomBox(x)
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
