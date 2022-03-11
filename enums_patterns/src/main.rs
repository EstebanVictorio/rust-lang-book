use std::fmt;
use std::num;

/**
 * The concept of enums also exists in Rust
 * Here, for example, we have an enum that, contrary to other enum implementations,
 * has the capability of building an enum with a value inside, rather than assigning a direct
 * value to them.
 * IpAddress is an enum type which values can be set to, either a string or a collection of "usize" numbers
 * This is good cause we are allowed to be even more complex or meaningful with our enums.
 * These can be, depending on the situation, as powerful as structs are.
 */
enum IpAddress {
    V4(String),
    V6(String),
    V4SPLIT(usize, usize, usize, usize),
}

/** Here we take our shapes example and take advantages of struct in conjunction
 * with enums, wrapping shapes inside the shape enum */
struct ShapeAttributes {
    width: usize,
    height: usize,
}

/** We then create a way to apply default values to our shapes */
impl Default for ShapeAttributes {
    fn default() -> ShapeAttributes {
        ShapeAttributes {
            width: 0,
            height: 0,
        }
    }
}

/** After that, we define our valid shapes into our program */
enum Shape {
    Triangle(ShapeAttributes),
    Rectangle(ShapeAttributes),
    Circle(ShapeAttributes),
    Square(ShapeAttributes),
    Unknown,
}

/** And finally we create an implementation to accept a shape string which will tell us which shape we can create with the provided value */
/** If no valid value is provided, we'll get a None value */
impl Shape {
    fn create(shape: &str) -> Option<Shape> {
        match shape {
            "triangle" => Some(Shape::Triangle(ShapeAttributes {
                ..Default::default()
            })),
            "circle" => Some(Shape::Circle(ShapeAttributes {
                ..Default::default()
            })),
            "rectangle" => Some(Shape::Square(ShapeAttributes {
                ..Default::default()
            })),
            "square" => Some(Shape::Rectangle(ShapeAttributes {
                ..Default::default()
            })),
            _ => None,
        }
    }

    /** Here we created a function that allows to check if a shape is a quadrangle,
     * but without caring for the exact values in the shape, we just want to know
     * if a shape belongs to two particular variants of the Shape enum */
    fn is_quadrangle(shape: &Option<Shape>) -> bool {
        match shape {
            Some(Shape::Rectangle(_)) => return true,
            Some(Shape::Square(_)) => return true,
            _ => return false,
        }
    }

    /** We can also use if let control flow to create our shapes */
    fn non_guaranteed_shape(shape: &str) -> Option<Shape> {
        if let "triangle" = shape {
            return Some(Shape::Triangle(ShapeAttributes {
                ..Default::default()
            }));
        }

        if let "rectangle" = shape {
            return Some(Shape::Rectangle(ShapeAttributes {
                ..Default::default()
            }));
        }
        if let "circle" = shape {
            return Some(Shape::Circle(ShapeAttributes {
                ..Default::default()
            }));
        }

        if let "square" = shape {
            return Some(Shape::Square(ShapeAttributes {
                ..Default::default()
            }));
        }

        Some(Shape::Unknown)
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attrs = match self {
            Shape::Circle(shape_attrs) => format!(
                "Width: {}, Height: {}",
                shape_attrs.width, shape_attrs.height
            ),
            Shape::Triangle(shape_attrs) => format!(
                "Width: {}, Height: {}",
                shape_attrs.width, shape_attrs.height
            ),
            Shape::Square(shape_attrs) => format!(
                "Width: {}, Height: {}",
                shape_attrs.width, shape_attrs.height
            ),
            Shape::Rectangle(shape_attrs) => format!(
                "Width: {}, Height: {}",
                shape_attrs.width, shape_attrs.height
            ),
            _ => format!("Width: Invalid, Height: Invalid"),
        };
        write!(f, "{}", attrs)
    }
}

impl IpAddress {
    // Here, we took advantage of using implementation methods to access the contextual version the IpAddress enum contains
    // and return it depending on which enum we are using
    fn version(&self) -> String {
        match self {
            IpAddress::V4SPLIT(..) => String::from("v4"),
            IpAddress::V4(_) => String::from("v4"),
            IpAddress::V6(_) => String::from("v6"),
        }
    }

    fn address(&self) -> String {
        // Same as above, we used the supported values contained in the enums
        // and return it so we can finally use the value as a string
        // We could just also return a tuple for the split version, but we'll keep it simple here
        match self {
            IpAddress::V4(value) => String::from(value),
            IpAddress::V4SPLIT(byte_1, byte_2, byte_3, byte_4) => {
                String::from(format!("{},{},{},{}", byte_1, byte_2, byte_3, byte_4))
            }
            IpAddress::V6(value) => String::from(value),
        }
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address: {}, Version: {}",
            self.address(),
            self.version()
        )
    }
}

fn main() {
    let v4_ip_address = IpAddress::V4(String::from("127.0.0.1"));
    let v6_ip_address = IpAddress::V6(String::from("::1"));
    let v4_ip_address_split = IpAddress::V4SPLIT(127, 0, 0, 1);
    println!("First address: {}", v4_ip_address);
    println!("Second address: {}", v6_ip_address);
    println!("Third address: {}", v4_ip_address_split);

    let triangle_by_factory = Shape::create("triangle").unwrap_or(Shape::Unknown);
    let circle_by_factory = Shape::create("circle").unwrap_or(Shape::Unknown);
    let rectangle_by_factory = Shape::create("rectangle").unwrap_or(Shape::Unknown);
    let square_by_factory = Shape::create("square").unwrap_or(Shape::Unknown);
    let rhomboid_by_factory = Shape::create("rhomboid").unwrap_or(Shape::Unknown);
    println!("Triangle -> {}", triangle_by_factory);
    println!("Circle -> {}", circle_by_factory);
    println!("Rectangle -> {}", rectangle_by_factory);
    println!("Square -> {}", square_by_factory);
    println!("Rhomboid -> {}", rhomboid_by_factory);

    let unknown_shape = Shape::non_guaranteed_shape("diamond").unwrap();
    let another_triangle = Shape::create("triangle");
    let another_square = Shape::create("square");
    println!("Unknown shape -> {}", unknown_shape);
    println!(
        "Is triangle a quadrangle: {}",
        Shape::is_quadrangle(&another_triangle)
    );
    println!(
        "Is square a quadrangle: {}",
        Shape::is_quadrangle(&another_square)
    );
}
