/** For modules to exist, we have two variants of crates that can exist in a package */
/** A package is a project constituted by at least one crate, be it binary for execution or library one */
/** A package can only contain one library crate but it can contain many binary crates */
/** Both of the crate types mentioned above are called crate roots */

/**
 * For example, here we are just redeclaring our functionality from our Shapes example in the enums and patterns
 * lesson into a shape module
 */
pub mod shape;
pub use shape::attributes;
pub use shape::shape_type;
