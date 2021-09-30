// NOTE: Variables can indeed live in the global scope
const SPEED_OF_LIGHT_IN_METERS_PER_SEC: u32 = 300_000_000;

fn variables_and_mutability() {
    // NOTE: Variables and mutability
    // let x = 5; // NOTE: If you were to do this without `mut` before the variable's name...
    // x = 6; // ...this cannot happen, you have to explicitly state if a variable is mutable thru the `mut` reserved keyword, like the example below...
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn constants_vs_variables() {
    // NOTE: Constants can also be inside a function
    const KM: u32 = 1_000;
    // NOTE: Variables cannot be in the global scope
    let speed_of_light_in_kms_per_sec = SPEED_OF_LIGHT_IN_METERS_PER_SEC / KM;
    println!(
        "The speed of light in kms is: {}",
        speed_of_light_in_kms_per_sec
    );
}

fn shadowing() {
    // NOTE: Shadowing allows you to redeclare variables using the same names as before, like recycling names...
    // ... in certain cases it avoids confusion as to when you need the same name for different values...
    // ... and prevents you from creating bugs if you need the same value and encounter yourself in the situation where...
    // ... a different value is needed

    // NOTE: For example, the variable `guess` starts as a string...
    let guess = String::from("I'm a string now!");
    println!(
        "Hi! I'm the variable `guess`, and I've the following value: {}",
        guess
    );
    // ... and ends up as a number
    let guess: u8 = 5;
    println!(
        "I'm still the variable `guess`, and now I've the following value: {}",
        guess
    );
}

fn data_types() {
    // NOTE: Rust is a statically typed language, so, it must know every type of all variables at compile time
    // This plays a role in calling some functions that need to infer the conversion or parsing needed, such as `.parse()` from, say, a `str` or `String`...
    // ... just like the example below
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);

    // Scalar types
    // NOTE: Rust has 4 primary scalar types, constituted by the following:
    let integer: usize = 5;
    //  NOTE: This is called integer overflow...
    // ... Rust will behave in 2 different ways depending on the mode the final binary is executing
    // If in Debug mode, Rust will add overflow checks and panic if it finds itself in an integer overflow
    // If in Release mode, Rust will add two's complement wrapping, which will...
    // ... go around to the first value of the range if overflow happens. Considering the variable in the line below...
    // ... the next value will wrap around and, instead of being 256, it will become 0. If it were 257, it will become 1, and so on...
    // let integer: u8 = 256;
    let floating_point: f32 = 3.0;
    let boolean = true; // NOTE: This one is inferred from value. This can be done with some other types as well
    let character: char = 'a';
    println!(
        "
        Integer: {},
        Floating point: {},
        Boolean: {},
        Char: {},
    ",
        integer, floating_point, boolean, character
    );

    // Compound types
    // Tuples: These exist in Rust, and are considered compound due to the capability of holding multiple values, which are not constrained to hold values of one type.
    let tuple: (usize, String) = (8, String::from("Hello! I'm a string!"));
    println!("Tuple value 1: {}, Tuple value 2: {}", tuple.0, tuple.1);

    // Arrays: Contrary to tuples, arays can only hold values of the same type
    // Usually, you would write them as follows in the line below...:
    let _arr = [1, 2, 3, 4, 5];
    // ... but they also can be of explicit type and size like:
    let arr: [usize; 5] = [1, 2, 3, 4, 5];
    // NOTE: The following is a control flow block: a for loop, which basically helps us run a segment of code multiple times given a condition expression
    for (i, item) in arr.iter().enumerate() {
        println!("Array item {}: {}", i, item);
    }

    // You can also access array items by using an index, and a program will compile if you try to access an unexistent index...
    // ... such case can be using `arr` like: arr[5]. Program will panic with the `out of bounds` error.
}

fn func_1() {
    println!("Hello! I'm function 1");
}

fn func_2() {
    println!("Hello! I'm function 2 and I'm calling 1");
    func_1();
}

fn func_3() {
    println!("Hello! I'm function 3 and I'm calling 2");
    func_2();
}

// NOTE: Functions are also a thing in Rust, and they have to explicitly state the return and argument types
fn functions(callees: usize) {
    println!(
        "Hello! I'm function `functions` and I'm calling {}",
        callees
    );
    func_3();
    println!("Is valid: {}", ternary_func(true));
}

// NOTE: Functions also have the possibility to contain expression. Expressions, unlike statement, don't end in semicolons and...
// ... return a value as a result.
// This one in particular, also has control flow like the for loop in the `data_types` functions, but instead uses an `if... else` block
fn ternary_func(valid: bool) -> String {
    if valid {
        String::from("Valid!")
    } else {
        String::from("Invalid!")
    }
}

fn main() {
    variables_and_mutability();
    constants_vs_variables();
    shadowing();
    data_types();
    functions(3);
}
