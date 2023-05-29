use oop_features_state_pattern_crate::blog::post::Post;

// In this lesson, we'll implement the state pattern to practive OOP features.

// The book implementation uses different entities to represent a Blog Post, State and its properties with trait objects
// and Structs, to discern which step goes to which step.

// Here, we'll use an enum

// The idea is that enums provide an easier to understand codebase, but the mentioned tradeoff is that with enums we would end up
// in a situation where we would have to match on the enum in every method that uses it, which would be a lot of boilerplate code.

// The approach taken here is different: since OOP allows for object mutation side effects, we'll use that to our advantage and
// create a method called transition that will mutate the object's state, and then we'll call that method in every method that needs
// to mutate its state. Therefore, if we need to move its state, transition will do it for us, without ever worrying again of that thing.

// Also, the transition method will allow for state force, in case we need custom states according to business rules.

// Lastly, we'll use a revision property for the post to hold all text from the post and all new additions, and a content property
// which will hold the real content once the revision has been actually reviewed and the post has transitioned to published,
// and as such, we'll always return the content property when it is requested.

// This implementation differs from the one in the book, since the book explained a concrete implementation of types
// that do it a more Rust-y way that ensures properties aren't accessed in the wrong state by not even implementing them,
// as well as using trait objects to ensure that the properties available are acccesible only by the correct state.

// This implementation is more of a "naive" implementation, that implement shallow properties and only transitions into
// different states, and ensure state transition depending on its implementation when creating posts using enum state,
// but it works, and it's easier to understand.

// In this implementation we allowed impossible states, while the book implementation didn't, as we were encouraged/challenged (by the book) to implement
// states by using an enum rather than trait objects and compiler checks to prevent bugs early on.

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.add_text("And another one in the morning");
    post.send_for_review();
    post.approve();
    post.approve();
    println!("{}", post.content());
}
