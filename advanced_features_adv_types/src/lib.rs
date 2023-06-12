#[cfg(test)]
mod tests {
    // #[test]
    // fn it_equal_to_never() {
    //     let result = return_never().err().unwrap(); // <-- here we are introduced a new type called never. Never is a type considered when a function or a returning nature mechanic of the language should never return something.
    //                                                 // right now, the never type is still experimental as in so there's an open issue where it's being discussed whether to promote it to an actual type or not.
    //                                                 // but it is usable among the language so, we can use it to explicitly state when a function should never return something.
    //     assert_eq!(result, !);
    // }

    // fn return_never() -> Result<i32, !> {
    //     println!("This function never returns.");
    //     let result = Ok(3);
    //     result
    // }
    use std::boxed::Box;
    trait Matter {
        fn state(&self) -> String;
    }

    struct Ice {
        state: String,
        degrees: i32,
    }

    struct Water<'a> {
        state: String,
        temperature: &'a i32,
    }

    struct Vapor {
        state: String,
        condensation_point: Box<i32>,
    }

    impl Matter for Ice {
        fn state(&self) -> String {
            String::from(&self.state)
        }
    }

    impl<'a> Matter for Water<'a> {
        fn state(&self) -> String {
            String::from(&self.state)
        }
    }

    impl Matter for Vapor {
        fn state(&self) -> String {
            String::from(&self.state)
        }
    }

    #[test]
    fn it_types_with_a_dst_trait() {
        let result = returns_generic_type::<&str>(&"Hello, World!");
        assert_eq!(result, &"Hello, World!");
    }

    #[test]
    fn it_knows_every_matter_state() {
        let ice = Ice {
            state: String::from("solid"),
            degrees: 0,
        };

        let water_temperature: i32 = 20;

        let water = Water {
            state: String::from("liquid"),
            temperature: &water_temperature,
        };

        let vapor = Vapor {
            state: String::from("gas"),
            condensation_point: Box::new(100),
        };

        assert_eq!(get_matter_state(&ice), "solid");
        assert_eq!(get_matter_state(&water), "liquid");
        assert_eq!(get_matter_state(&vapor), "gas");

        assert_eq!(get_number_type(&ice.degrees), &0);
        assert_eq!(get_number_type(water.temperature), &20);
        assert_eq!(get_number_type(&(*vapor.condensation_point)), &100);
    }

    fn get_matter_state<T: Matter>(matter: &T) -> String {
        matter.state()
    }

    fn get_number_type<T: ?Sized>(number: &T) -> &T {
        number
    }

    // This function returns the type of the parameter that is passed to it.
    // The key differente here is that we are using a dynamically sized parameter from dynamically sized types concept.
    // This is allowed in Rust by a variety of types and factors in your implementation details that we've seen before in previous lessons, such as dynamic dispatch, trait objects, smart pointers, dynamic trait syntax, wrapping values in Boxes, and so on.
    // This is just a so far unexplored concept: Sized trait. In fact, all generic parameter functions are Sized by default.
    // But we can allow a dynamically sized parameter on traits using the syntax below. Usually we must enforce the syntax on traits used as objects by putting them behind a reference or a Box.
    // Here we can allow the parameter to be dynamically sized by using the ?Sized syntax.
    fn returns_generic_type<T: ?Sized>(t: &T) -> &T {
        t
    }
}
