#[cfg(test)]
mod tests {
    #[test]
    fn it_shadows_variables() {
        let x = Some(5);
        let y = 10;

        match x {
            // We can also use ranges to match values.
            // Same goes for char values. Rust infers the type of the range with ASCII code values.
            Some(1..=5) => println!("Got a number between 1 and 5"),
            Some(50) => println!("Got 50"),
            // When we use pattern matching, in match arms, if there's a variable with the same name in an upper scope, and
            // we attempt to redeclare a variable with the same name, it will shadow the variable in the upper scope.
            // For example, this arm will shadow the upper scope's y variable.
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Whatever, x = {:?}", x),
        };
    }

    struct Circle {
        r: u32,
    }

    #[test]
    fn it_destructures_struct_props() {
        // Another pattern is to destructure properties from structs
        // This one here uses the shorthand syntax
        let x = Circle { r: 6 };
        let Circle { r } = x;
        println!("r = {}", r);

        // We can also use pattern matching to discern if some properties have a certain value
        match x {
            // We can also have extra conditions in the match arm, or nested conditions, if you may call them like that
            Circle { r: 0 } => println!("r is 0"),
            Circle { r: 1..=5 } => println!("r is between 1 and 5"),
            Circle { r } if r > 5 => println!("r is greater than 5"), // The downside to this is that the compiler won't check for exhaustiveness when using match guard expressions/conditions
            Circle { r: 8 } | Circle { r: 10 } if r % 2 == 0 => println!("r is 4 or 6 and is even"), // Also, the extra conditions apply to all the arms, not just the one it's in
            Circle { r: radius @ 9..=11 } => println!("r is {}", radius), // We can also bind the value to a variable using the "@" (at) operator, and use it in the match arm so we have a way to capture whatever matched for the r property while matching
            // We can also just try whatever with the range syntax, expanding all props with any value, as long as it's a Circle
            // This is an irrefutable pattern that it's always true if the above arm is not matched
            // Also, this would work as the underscore match arm
            Circle { .. } => println!("A circle with any radius"),
        }

        // Same goes for enums destructuring. The difference lies in that enums will potentially have different values,
        // so you could destructure values on each matching arm according to the values enums hold.
        // This what makes it so powerful.

        // Using an underscore also helps us to ignore values we don't care about.
        // For example, in a pattern matching, we could ignore the value of a property of an Option by using Some(_)
        // In that case, we would only need to know if a value came out as Some, rather than also inspecting the value as well
        // Now, there's an ownership thing we need to take into account when using.

        // We can remove the warning from a variable being unused by prefixing it with an underscore.
        // This is useful when creating PoCs or prototyping products.

        let x = Some(5);
        // let y = Some(String::from("Hello"));

        // The following code won't compile because, even if we are using an underscore, we are still taking ownership of the value.
        // And the value is bound to that variable, so we would be trying to transfer ownership, which is not valid.
        // if let Some(_y) = y {
        //     println!("We have a value");
        // }
        // String does not implement the Copy trait, so, ownership prevents from using y
        // println!("{:?}", y);

        // Here we can use it because i32 implements the Copy trait, so, ownership is not transferred
        if let Some(_x) = x {
            println!("We have a value");
        }
    }
}
