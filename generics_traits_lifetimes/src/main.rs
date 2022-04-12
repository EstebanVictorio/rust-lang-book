use std::fmt::Display;
use utils::entities::{Point, PointInfo, PointMixCreator, Super};
use utils::functions::{
    get_largest, get_largest_item, get_largest_item_ref, longest, longest_with_custom_msg,
};
// There's a special lifetime called the Static Lifetime
// This lifetime specifies a reference that lives for the entire program's execution lifetime
// There will be times where you might've some references that need the lifetime parameter as
// the compiler will tell you they need it. And sometimes you might encounter the compiler
// suggesting to use the 'static lifetime as the lifetime specified, but often times
// this comes from the perspective that you might be creating dangling references, and
// is often solved by fixing those cases by specifying the correct lifetime or changing your code
// so the reference does not live long enough, rather than just getting away with specifying the 'static
// lifetime parameter
const string: &'static str = "asd";
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let letter_list = vec!['A', 'B', 'C', 'D', 'E'];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let result = get_largest_item(&number_list);
    println!("The largest number is {}", result);

    let result = get_largest_item(&letter_list);
    println!("The largest letter is {}", result);
    println!("Returning refs=====:");
    let result = get_largest_item_ref(&number_list);
    println!("The largest number is {}", result);
    let result = get_largest_item_ref(&letter_list);
    println!("The largest letter is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let point = Point { x: 3, y: 3 };
    let another_point = Point { x: 10, y: 4 };
    let diff = point.abs_x_diff_from_point(&another_point);
    println!(
        "Absolute difference between point and another_point: {}",
        diff
    );

    let mixed_point = PointMixCreator { x: 5, y: 6 };
    let another_mixed_point = PointMixCreator { x: 5, y: 6 };
    let mixed_from_points = another_mixed_point.new_from_another_point(mixed_point);
    println!(
        "Mixed point values - x:{}, y: {} ",
        mixed_from_points.x, mixed_from_points.y
    );
    // Here we use the trait PointInfo implemented for Point<i32>
    point.info();
    point.description();
    describe(&point);
    describe_trait_bound_generics(&point, &another_point);
    describe_pointinfo_display(&point);
    describe_pointinfo_display_with_where(&point);
    describe_multiple_pointinfo_display_with_where(&point, &another_point);
    let built_point = produce_point(1, 2);
    built_point.info();

    // Lifetimes is a concept that allows the borrow checker to determine how long a reference will live.
    // This mechanism prevents for dangling references to exist
    //
    // The following code is invalid, given that the first variable, "x" pretends to hold a value inside a
    // scope that has a definite "lifetime" which is inside the outer block scope and ends before the whole block ends
    // {
    //   let x = 0;
    // {
    //   let y = 5;
    //   x = &y;
    // }
    //
    // println!("x: {}", x);
    // }
    //

    // For example, taking the following block, it's very similar to the previous code comment block
    // let ref_a; // We declare a variable here named "ref_a"
    // {
    //     let str_a = "a"; // Then, in an inner scope, we declare another one;
    //     ref_a = &str_a; // And we assign a reference to "ref_a" that points to "str_a"
    // }
    // println!("The longest string is: {}", longest("aa", ref_a)); //
    // println!("Using ref erroneously due to bigger lifetime: {}", ref_a);

    // The same mechanisms happen with structs in terms of lifetimes and their fields.
    // The following code is valid because a field from Super has a lifetime generic parameter
    // that indicates that, meanwhile the reference that it's field points lives, you can create
    // a struct of Super that does not outlive the reference.
    // let mut ref_b; // NOTE: This code block is invalid
    // {
    //     let a = String::from("ats");
    //     ref_b = &a;
    // }

    // let super_struct = Super { message: ref_b };
    // println!("{}", super_struct.message);
    let mut ref_b = String::from("asd");

    let super_struct = Super { message: &ref_b };
    let message = super_struct.super_info("Hey there! I'm a new super info!");
    println!("And I have the following message!: {}", message);
    longest_with_custom_msg("String 1", "String two", "The winner is");
}

/** We can also set parameter types as trait */
/** We do that by using the "impl Trait" syntax */
/** Then, we can use whatever methods are defined in the Trait */
fn describe(point: &impl PointInfo) {
    point.info();
}

/** The above syntax works for simple cases, or, more straightforward, which is sugar syntax */
/** But there's the longer format of it called trait bounds */
/** Which looks a little bit more complex to write */
/** This is used if you would like to force many arguments to a single type without rewriting the entire type */
/** Using generics, we can assign PointInfo to a Generic P type and use it as the type for all the
 * arguments in the following function
 */
fn describe_trait_bound_generics<P: PointInfo>(p1: &P, p2: &P) {
    p1.info();
    p2.info();
}

/** We are also able to throw multiple types in the generic for the trait bound to the generic
 * This way we ensure that the parameter implements must implement both traits to be considered valid
 */
fn describe_pointinfo_display<P: PointInfo + Display>(p: &P) {
    println!("{}", p);
}

/** Since trait bound generics can end up bloated with so much types
 * "where" clasues can help. This is a syntax to do the same but in a much clearer way
*/
fn describe_pointinfo_display_with_where<P>(p: &P)
where
    P: PointInfo + Display,
{
    println!("{}", p);
}

/** This works for multiple generics too */
fn describe_multiple_pointinfo_display_with_where<P, U>(p: &P, p2: &U)
where
    P: PointInfo + Display,
    U: PointInfo + Display,
{
    println!("{}", p);
    println!("{}", p2);
}

// We can also define a trait as the return type for a function with the impl trait syntax
// This is useful cause we can statically type the return type but we do not return the
// concrete type and still return a value that's valid for the trait
// But obviously, it has to implement the trait to be valid
fn produce_point(x: i32, y: i32) -> impl PointInfo {
    Point { x, y }
}

fn produce_char_point(x: char, y: char) -> impl PointInfo {
    Point { x, y }
}
