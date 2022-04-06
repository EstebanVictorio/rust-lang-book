use std::fmt::Display;
use utils::entities::{Point, PointInfo, PointMixCreator};
use utils::functions::{get_largest, get_largest_item, get_largest_item_ref};
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
