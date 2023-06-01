use std::fmt::Display;
use std::ops::Add;

#[cfg(test)]
mod tests {
    use crate::GeneralAddress;

    use super::{split, Circle, Delivery, DiameterCircle, Fedex, UPS};

    #[test]
    fn it_splits_a_list_safely_from_an_unsafe_block() {
        let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let (left, right) = split(&mut values, 3);
        println!("Left: {:?}", left);
        println!("Right : {:?}", right);
        assert_eq!(left, &[1, 2, 3]);
    }

    #[test]
    fn it_sums_two_circles_and_both_sum_up_three() {
        let circle1 = DiameterCircle { d: 2 };
        let circle2 = Circle { r: 2 };
        let circle3 = circle1 + circle2;
        assert_eq!(circle3.d, 6);
    }

    #[test]
    fn it_uses_delivery_and_delivery_company_function() {
        let fedex = Fedex {
            address: String::from("1234 Fedex Address"),
        };

        let ups = UPS {
            address: String::from("1234 UPS Address"),
        };

        println!("Fedex: {}", fedex);
        println!("UPS: {}", ups);
        ups.print_address();
        fedex.print_address();

        assert_eq!(<UPS as Delivery>::send(), false);
        assert_eq!(<Fedex as Delivery>::send(), false);
    }
}

struct Circle {
    r: i32,
}

struct DiameterCircle {
    d: i32,
}

impl Add<Circle> for DiameterCircle {
    type Output = DiameterCircle;

    fn add(self, other: Circle) -> DiameterCircle {
        DiameterCircle {
            d: (self.d + other.r * 2),
        }
    }
}

trait GeneralAddress: Display {
    fn print_address(&self) {
        println!("Address: {}", self.to_string());
    }
}

trait Delivery: Display {
    fn send() -> bool;
}

impl Display for UPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[UPS Address]: {}", self.address)
    }
}

impl Display for Fedex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Fedex Address]: {}", self.address)
    }
}

struct UPS {
    address: String,
}

struct Fedex {
    address: String,
}

impl GeneralAddress for UPS {
    fn print_address(&self) {
        println!("Address: {}", self.address);
    }
}

impl GeneralAddress for Fedex {
    fn print_address(&self) {
        println!("Address: {}", self.address);
    }
}

impl Delivery for UPS {
    fn send() -> bool {
        false
    }
}

impl Delivery for Fedex {
    fn send() -> bool {
        false
    }
}

impl UPS {
    fn send() -> bool {
        true
    }
}
impl Fedex {
    fn send() -> bool {
        true
    }
}

use std::slice;

fn split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let pointer = values.as_mut_ptr();

    assert!(mid <= values.len());

    unsafe {
        (
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), values.len() - mid),
        )
    }
}
