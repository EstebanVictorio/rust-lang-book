use std::boxed::Box;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

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

enum RcList {
    Cons(i32, Rc<RcList>),
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

    // Another term that's powerful for interoperable types is deref coercion
    // Putting it simply, Deref coercion is returning a reference from a type
    // that implements the Deref trait and returns a reference to another type.
    // Imagine using a "from_ref" function that receives a string slice.
    // We can wrap a string with our custom box and use the reference to that string
    // as the value passed in to that reference, for example:
    let hello = CustomBox::new(String::from("Hey!"));
    from_ref(&hello);
    // If deref coercion didn't exist, we would have to write multiple dereferencing until we got to the actual
    // reference we wanted to use. In the example above, that'd be:
    let hello = CustomBox::new(String::from("Hey!"));
    from_ref(&(*hello)[..]);
    // With deref coercion, the compiler will analyze the types and use "deref()" as many times as necessary to
    // to get to the actual reference to match the parameter's type for the type reference we are using. Since this
    // calls are resolved at compile time, there is no runtime penalty for taking advantage of this Rust's feature.

    // We can also obtain mutable references by implementing the DerefMut trait instead
    let mut hello_2 = CustomBox::new(String::from("Hey 2!"));
    from_ref_mut(&mut hello_2);

    // Another that's important to note in the pointers topic is the Drop trait.
    // This trait allows you to customize what happens when a value is about to go out of scope.
    // Rust places resource freeing code automatically, so you don't have to worry about memory leaks.
    // In other languages, this has to be done manually.
    // In the following example, we'll set some structs that will have some code executed due to our implementation
    // of such struct with the Drop trait. We'll print what data is about to go out of scope and the program will eventually end
    let c = CustomSmartPointer {
        data: String::from("String one"),
    };
    let d = CustomSmartPointer {
        data: String::from("String two"),
    };
    println!("Custom smart pointer created");
    // When dropping values, the code that is implemented by the "drop" function will have its executions run in reverse order of variable creation,
    // so, code inside of the "drop" implementation will run first with the "d" variable data and then the "c" variable data.

    // There's a function to force drop values manually, which is found in std::mem::drop crate.
    // This function explicitly drops a value.
    // We cannot use the "drop" implementation directly because we cannot avoid that the compiler will always place "drop" calls
    // at the end of the main function.

    // Another type of smart pointer is the Rc<T> type, which allows multiple ownership
    // of a resource by creating a counter of references that point to the same data
    // The Rc<T> type only works for single threaded scenarios.
    // A non-compile example for multiple owners of a single piece of data would be the following (using our List type):
    //     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //     let b = Cons(3, Box::new(a));
    //     let c = Cons(4, Box::new(a));
    // Obviously this is not allowed, since "a" would be owned by "b", so, the compiler lets you know that "c" cannot own
    // "a" because "b" owns it

    // Instead, we use an Rc to wrap a resource, and then increase the number of owners of such data by using Rc::clone which increases
    // the counter of references pointing to that piece of data
    // Sure, we could add the appropriate references and lifetimes to avoid this scenario, and accounting this use-case.
    // But, not every scenario is like that, sometimes its usage will outlive the use-case, so instead, we'll use
    // Rc here to allow the resource to live up to when there are 0 references pointing to it so it gets dropped automatically.
    // Also, Rc::clone does not "deep-copy"  the data, as opposed to use, say, "list_a.clone()". It just increases the reference counter by 1.
    let list_a = Rc::new(RcList::Cons(
        4,
        Rc::new(RcList::Cons(
            5,
            Rc::new(RcList::Cons(6, Rc::new(RcList::Null))),
        )),
    ));
    let list_b = RcList::Cons(7, Rc::clone(&list_a));
    let list_c = RcList::Cons(8, Rc::clone(&list_a));
    println!(
        "RC Count before list_d is created: {}",
        Rc::strong_count(&list_a)
    );
    {
        let list_d = RcList::Cons(8, Rc::clone(&list_a));
        println!(
            "RC Count while list_d exists: {}",
            Rc::strong_count(&list_a)
        );
    }
    println!(
        "RC Count after list_d is dropped: {}",
        Rc::strong_count(&list_a)
    );
}

fn from_ref(msg: &str) {
    println!("From ref: {}", msg);
}

// In the end, there are different ways that deref coercion works:
// 1.- From a type to another when the target type implementation of Deref is of the second type
// 2.- From a mutable type to another another mutable type when the target type implementation of DerefMut is of the second type
// 3.- From a mutable type to another when the target type implementation of Deref  is of the second type
//
fn from_ref_mut(msg: &mut str) {
    for mut item in msg.bytes() {
        item = 98; // we change it and then print the mutated values
        print!("{}", item as char);
    }
    println!();
}

// We can also create custom types that can work with the dereference operator!
// Let's do that:
struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> CustomBox<T> {
        CustomBox(x)
    }
}

impl<T> DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}
