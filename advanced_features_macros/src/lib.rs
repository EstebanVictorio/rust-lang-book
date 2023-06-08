#[macro_export]
macro_rules! greet_each {
    // This example showcases a declarative macro
    ( $( $x: expr ),*  ) => {
        let mut v = Vec::new();

        $(
            v.push($x);
        )*

        let mut greet = String::from("Hello ");
        let comma = String::from(", ");
        let and = String::from(" and ");

        for (index, value) in v.iter().enumerate() {
            let mut string = String::from(*value) + &comma[..];
            println!("index: {}", index);
            println!("index: {}", index);
            if index == v.len() - 1 {
                greet.pop();
                greet.pop();
                string = String::from(&and[..]) + *value;
            }

            greet.push_str(&string[..]);
        }

        println!("{}", greet);

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advanced_features_macros_attr::property;
    use advanced_features_macros_derive::Greet;
    // For procedural macros, implementations don't rely on expressions as in declarative macros.
    // Below we have a simple implementation of a trait that prints its own message
    // However, we don't provide a default implementation for the trait
    // So we'll do that with a procedural macro
    // struct RestaurantServer;

    // impl Greet for RestaurantServer {
    //     fn greet(&self) {
    //         println!("Hello, I'll be your server today.");
    //     }
    // }

    // struct PoliceOfficer;

    // impl Greet for PoliceOfficer {
    //     fn greet(&self) {
    //         println!("Hello, I'm here to protect and serve.");
    //     }
    // }

    #[property(Message)]
    struct Mail;

    #[property(Number)]
    struct PostalCode;

    trait Greet {
        fn greet();
    }

    #[derive(Greet)]
    struct RestaurantServer;

    #[derive(Greet)]
    struct PoliceOfficer;

    #[test]
    fn it_greets_each() {
        RestaurantServer::greet();
        PoliceOfficer::greet();
        let mail = Mail {
            message: String::from("Mail for person!"),
        };

        let postal_code = PostalCode { number: 21378 };

        println!("Message: {}", mail.message);
        println!("Number: {}", postal_code.number);

        greet_each!("Alice", "Bob", "Carol");
    }
}
