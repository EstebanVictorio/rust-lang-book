use std::fmt;

struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User - User: {}, Email: {}, Sign In Count: {}, Active: {}",
            self.username, self.email, self.sign_in_count, self.active
        )
    }
}

struct RGB(usize, usize, usize);

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Red: {}, Green: {}, Blue: {}", self.0, self.1, self.2,)
    }
}

fn main() {
    println!("Hello, world!");
    /** When creating structs and assigning the value of structs, it's illegal to mark it as partially mutable.
     * You have to comply to either being immutable or mutable, but not mixed mutability capabilities for each field.
     */
    let steve = define_new_user("Steve", "estebanvictorio92@gmail.com");
    /**
     *
     * Also, if you wanted to reuse values from a previously created struct, you can do that by using the "..struct_var"
     * syntax to spread values from that variable to the creation of another struct.
     */
    let mut another_steve = User {
        username: String::from("Steve once more"),
        email: String::from("estebanvictorio92-2@gmail.com"),
        ..steve
    };

    /**
     * Keep in mind that, if your previously created struct contains properties that do not implement the Copy trait,
     * that struct won't be available for other references since those properties were moved to the new struct, which now
     * owns the first struct.
     * For example, printing the struct is not possible in the following scenario since it's already owned by someone else
     * let mut another_steve = User {
        username: String::from("Steve once more"),
        ..steve
    };
     * println!("Immutable user: {}", steve);
     * For the above to work, you'd need to either move it up before the new struct creation or assign all the non-Copyable properties
     * so the remaining ones that do implement the Copy trait make the first struct usable afterwards.
     */

    println!("Immutable user: {}", steve);
    another_steve.username = String::from("Another Steve");
    println!("Mutable user: {}", another_steve);

    /** Structs also can be built upon indices, behaving like tuples, without property names.
     * This gives you two advantages:
     * - No need to define properties and just identify a variable by its struct type
     * - Contextual separation from normal tuples
     *
     * This means that, even if a tuple is "struct-type" like, it's distinguishable and completely different from a tuple
     * of the same types.
     *
     * For example, the following RGB struct is different from a normal three value tuple:
     * RGB(1,2,3) !== (1,2,3)
     */
    let one_two_three_color = RGB(1, 2, 3);
    let three_value_tuple = (1, 2, 3);
    /*
     * The following is invalid due to different types
     */
    // print!("{}", one_two_three_color !== three_value_tuple);
}

/** A function to create a new User */
/**
 * Structs are a custom data type that allows you to create more complex data pieces by bundling
 * together simpler types, or even composing structures from some other Structs, making together a piece
 * of data that constitutes or provides a more meaningful and rich data context.
 *
 * This helps you on grouping pieces of data that are related and provide your program more flexibility than just having a
 * bunch of variables that would otherwise be difficult to spot via your own eyesight, being the situation that you can just
 * group 'em all in a single source of truth.
 */
fn define_new_user(user: &str, email: &str) -> User {
    User {
        active: false,
        email: String::from(email),
        username: String::from(user),
        sign_in_count: 0,
    }
}

fn get_color(red: usize, green: usize, blue: usize) -> RGB {
    RGB(red, green, blue)
}
