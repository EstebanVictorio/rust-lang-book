use packages_crates_modules::shape::shape_type::Type;
fn main() {
    let triangle = Type::create("triangle").unwrap();
    println!("Triangle: {}", triangle);
}
