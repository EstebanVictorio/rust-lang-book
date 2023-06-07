#[macro_export]
macro_rules! greet_each {
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

    #[test]
    fn it_greets_each() {
        greet_each!("Alice", "Bob", "Carol");
    }
}
