// Also, we'll explore Rust's object-oriented features in regard with trait objects constraining implementations while
// leaving implementation details outside of the abstraction

// We'll start with a Screen implementation that will draw components represented by the Control object, implementing
// the Component trait's method "draw"

// Then we'll attempt to draw all the components with an iteration of the current screen components and calling "draw" on each one of them
// This will represent an abstract enough implementation of a screen that can draw components regardless of their type. This way, the screen does
// not need to know of what type any given component is, as long as it implements the Component trait, "draw" will be called on it, since it will work
// as a constraint for the implementation of the Component trait, so, every control must implement the "draw" method

// Implementations of trait objects have a trade-off.

// Rust has something called monomorphization, which, in short, means that the compiler generates code for each concrete type used in the code depending on
// the generic types used. This is a great feature that allows Rust to be a compiled language with no runtime cost, because concrete types are known at compile time, and the
// necessary code for the execution is generated.

// Trait objects, on the other hand, are a way to abstract over types, so, the compiler does not know what concrete type will be used at runtime, so, it cannot generate the necessary
// code for the execution. Instead, Rust uses pointers inside the trait object to know what concrete type is being used at runtime, and then, call on the required method.
// This is known as dynamic dispatch.

use oop_features_gui_crate::gui::control::{Control, ControlType};
use oop_features_gui_crate::gui::screen::Screen;
use std::boxed::Box;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Control::new(ControlType::Button)),
            Box::new(Control::new(ControlType::Select)),
        ],
    };

    screen.run();
}
