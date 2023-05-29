use oop_features_avg_collection::AveragedCollection;

// In this lesson we'll explore Rust's object-oriented features
// We'll start with a struct containing a collection of values and a value that summarizes the average of such values

// We import from a module that contains an struct which implements methods to add and remove numbers from a list
// As these numbers are either added or removed, a new average is calculated and stored in the struct

fn main() {
    // We create our new struct
    let mut collection = AveragedCollection::new();
    // Then add two numbers
    collection.add(1);
    collection.add(2);
    // Expect a correct average
    println!("Average: {}", collection.average());
    // Remove one element we previously added
    collection.remove();
    // And finally expect a correct average
    println!("Average: {}", collection.average());
}
