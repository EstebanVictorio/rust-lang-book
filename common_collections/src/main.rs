// In this lesson, we'll learn about collections
use packages_crates_modules::shape_type::Type;

/**
 * The first one being the Vector, we can create vectors either with the
 * new method from the Vec type or use the vec![] macro.
 * Typically you would initialize the vectors with some values, but let's do that from iteration
 * to be a little bit more explicit.
 */
use std::collections::HashMap;

fn main() {
    /**
     * Here we take advantage of our Shape models to get shapes so we can fill
     * our vector with shapes.
     */
    let mut triangles: Vec<Option<Type>> = Vec::new();
    let arr = 0..2;
    for i in arr {
        triangles.push(Type::create("triangle"));
        match &triangles[i] {
            Some(Type::Triangle(attrs)) => println!("w: {}, h:{}", attrs.width, attrs.height),
            _ => (),
        }
    }

    /** As usual, vectors go out of scope and drop once the scope is done. */
    {
        let mut circles = vec![];
        let arr = 0..2;
        for i in arr {
            circles.push(Type::create("circle"));
            match &circles[i] {
                // If we'd access this directly (moving ownership here)...
                /** ... we would not be allowed to retrieve the value inside, since we would be transferring ownership without
                 * giving back from previous move first
                 * Also, accessing a vector's item via index syntax gives back a reference, so we need to reference it first
                 */
                Some(Type::Circle(attrs)) => println!("w: {}, h:{}", attrs.width, attrs.height),
                _ => (),
            }
        }
    }

    /**
     * Let's move onto strings, since those also behave like collections in a great variety of aspects
     */
    let mut new_string = print_string_collections();
    new_string.push_str(" that's longer");
    println!("Longer string: {}", new_string);
    let another_string = concatenation(" than you think");
    println!("Another string: {}", another_string);

    /** We can do the same with the "format!" macro as well, in an easier to read format: */
    let s1 = "Im a";
    let s2 = "longer string";
    let s3 = "than the previous one";
    let s4 = format!("{} {} {}", s1, s2, s3);
    println!("{}", s4);
    let utf_8_bytes = utf_8_values();
    println!("UTF 8 String bytes: {}", utf_8_bytes);
    /** Here we introduce the use of hashmaps, which are collections that have values in the form of "key,value" pairs */
    let mut map: HashMap<&str, Type> = HashMap::new();
    // To insert values into the map, you specify its key and its value with the method ".insert(key, value)"
    map.insert("triangle", Type::create("triangle").unwrap());
    map.insert("circle", Type::create("circle").unwrap());
    map.insert("square", Type::create("square").unwrap());
    map.insert("rectangle", Type::create("rectangle").unwrap());
    println!("Map shape: {}", get_shape(&map, "this_shape"));
    println!("Map shape: {}", get_shape(&map, "triangle"));

    /**
     * We can also build hash maps from some other collections.
     */
    let shape_names = vec!["triangle", "circle", "square", "rectangle", "rhomboid"];
    let shapes = vec![
        Type::create("triangle").unwrap(),
        Type::create("circle").unwrap(),
        Type::create("square").unwrap(),
        Type::create("rectangle").unwrap(),
        Type::create("rhomboid").unwrap_or(Type::Unknown),
    ];
    // Underscores are needed for the type of "another_map" so Rust can infer types from the collections defined above
    let mut another_map: HashMap<_, _> = shape_names.into_iter().zip(shapes.into_iter()).collect();
    println!("Map shape: {}", get_shape(&another_map, "triangle"));
    print_all_map(&another_map);
    println!("====="); // Separation line :P
                       // We can also say that a square kind of a rectangle, so, we might as well let it happen and update our square property
    another_map.insert("square", Type::create("rectangle").unwrap());
    // ..And we print it again to verify
    print_all_map(&another_map);
    // We can also update values via the .entry() method in HashMap by a given property. If it exists, it will return a reference to the value.
    // If it doesn't exist, we can call .or_insert() from the .entry() call and assign a value to that new property
    let entry: &mut Type = another_map
        .entry("square")
        .or_insert(Type::create("square").unwrap());
    match entry {
        Type::Square(attrs) => {
            attrs.width = 50;
        }
        _ => (),
    }
    println!("====="); // Separation line :P
                       // And we print again after accesing a possible entry and updating its width to 50
    print_all_map(&another_map);
    let six_items = vec![1, 2, 3, 4, 5];
    let median_of_a_six_items_vec = median_of(&six_items);
    let vec_len = median_of_a_six_items_vec.len();
    println!("Median of a six items vec:");
    for (index, item) in median_of_a_six_items_vec.into_iter().enumerate() {
        if index == (vec_len - 1) {
            println!("{}", item);
            break;
        }
        print!("{}, ", item);
    }
    /** You can also have a tuple with key and value using ".collect()" from the iterator you get from a map */
    let iter_key_value_map: HashMap<_, _> = map.into_iter().collect();
    for (key, value) in iter_key_value_map {
        println!("Key: {}, Value: {}", key, value);
    }

    let another_vec = vec![1, 1, 2, 2, 3, 3, 3];
    println!("Mode of another vec: {}", mode_of(&another_vec));
    println!("\"asd\" to pig latin: {}", to_pig_latin(&"asd"));
    println!("\"first\" to pig latin: {}", to_pig_latin(&"first"));
    println!("\"apple\" to pig latin: {}", to_pig_latin(&"apple"));
}

/** Strings behave like collections too.
 * String literals have the capability to become String values thanks to
 * implementing the Display trait. Any type that implements the trait Display
 * can use the "to_string()" function to become a String
 */
fn print_string_collections() -> String {
    let s = "Some string";
    let string = s.to_string();
    println!("String: {}", string);
    string
}

/**
 * For concatenation of strings, we can use the "+" operator between strings.
 * The add method is used when using this operator and goes like this:
 * For a given reference or actual value, the add method will return the ownership
 * of the newly created string. Values that are not accessed via references are gonna
 * be deprived of their ownership, meaning that we'll actually use the same memory address
 * mutation on the variable we are using for concatenation.
 * Let's see it in action i the following function
 */
fn concatenation(curr: &str) -> String {
    let concat = String::from("Longer string");
    /*
     * The add method taking concat as the actual value and not creating copies
     * Then using the reference curr to append to the new_string
     */
    let new_string = concat + curr;

    new_string
}

/** For UTF-8 encoded values, the story changes
 * Values in UTF-8 usually depend on which character's representation is attempted
 * Let's dive into a very small example
 */
fn utf_8_values() -> String {
    let utf_8_string = "नमस्ते";
    let mut new_string = String::new();
    for char_item in utf_8_string.bytes() {
        new_string.push(char_item as char);
    }
    new_string
}

/** Another collection we have at our disposal is hash map
 * Hash maps are collections that store data in a "key -> value" fashion,
 * so we can access those values via a specific custom property name key.
 */
fn get_shape(map: &HashMap<&str, Type>, shape: &str) -> String {
    let mut shape_vals = String::new();
    match map.get(shape) {
        Some(Type::Triangle(triangle)) => shape_vals.push_str(&format!(
            "Triangle - w: {}, h: {}",
            triangle.width, triangle.height
        )),
        Some(Type::Circle(circle)) => shape_vals.push_str(&format!(
            "Circle - w: {}, h: {}",
            circle.width, circle.height
        )),
        Some(Type::Square(square)) => shape_vals.push_str(&format!(
            "Square - w: {}, h: {}",
            square.width, square.height
        )),
        Some(Type::Rectangle(rectangle)) => shape_vals.push_str(&format!(
            "Triangle - w: {}, h: {}",
            rectangle.width, rectangle.height
        )),
        _ => shape_vals.push_str(&format!("Non existant shape 🤷‍♀️")),
    }
    shape_vals
}

/**
 * We can also iterate over a HashMap
 * Here we print all the keys and values of the Type from the shape module given that it already implements the Display trait
 */
fn print_all_map(map: &HashMap<&str, Type>) {
    for (key, value) in map {
        println!("Key: {}, Value: {}", key, value);
    }
}

/** For practice, we can, with the help of hash maps, get the median of a vector */
fn median_of(list: &Vec<i32>) -> Vec<i32> {
    let len = list.len();

    if len == 0 {
        return vec![-1];
    }

    if len % 2 == 0 {
        let first = list.get((len / 2) - 1).unwrap();
        let second = list.get(len / 2).unwrap();
        return vec![*first, *second];
    }

    let median = list.get(len / 2).unwrap();
    vec![*median]
}

/** For practice, we can, with the help of hash maps, get the mode of a vector */
fn mode_of(iter: &Vec<i32>) -> usize {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for elem in iter {
        let reference = map.entry(*elem).or_insert(0);
        *reference += 1;
    }

    let key_value_map: HashMap<_, _> = map.into_iter().collect();
    let mut max: usize = 0;
    let mut max_value: usize = 0;
    for (key, value) in key_value_map {
        println!("Value: {}, Max: {}", value, max_value);
        if value > max_value {
            max_value = value;
            max = key as usize;
        }
    }

    max
}

/** Here we write a function that will help us determine if a letter is a vowel or not, using ascii number codes and casting the letter to an integer */
fn is_vowel(letter: &char) -> bool {
    let vowels: [i8; 5] = [97, 101, 105, 111, 117];
    let letter_ascii = letter.to_ascii_lowercase() as i8;
    for vowel in vowels {
        if letter_ascii == vowel {
            return true;
        }
    }
    false
}

/** Here we write a function that will convert a word to its pig lating version, depending on which type of letter starts each word with, vowel or consonant */
fn to_pig_latin(word: &str) -> String {
    let mut pig_latin_word = String::new();
    let mut chars = word.chars();
    let first_letter = chars.next();

    match first_letter {
        None => (),
        Some(val) => {
            let starts_with_vowel = is_vowel(&val);
            if !starts_with_vowel {
                for item in chars.into_iter() {
                    pig_latin_word.push(item);
                }
                let string = format!("-{}{}", &first_letter.unwrap(), "ay");
                pig_latin_word.push_str(&string[..]);
                return pig_latin_word;
            }
            let string = format!("{}-{}", word, "hay");
            pig_latin_word.push_str(&string[..]);
        }
    }
    pig_latin_word
}
