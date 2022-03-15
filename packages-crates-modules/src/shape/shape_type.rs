use crate::attributes::Attributes;
use std::fmt::{Display, Formatter, Result};

pub enum Type {
  Triangle(Attributes),
  Rectangle(Attributes),
  Circle(Attributes),
  Square(Attributes),
  Unknown,
}

impl Display for Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let attrs = match self {
      Type::Circle(shape_attrs) => format!(
        "Width: {}, Height: {}",
        shape_attrs.width, shape_attrs.height
      ),
      Type::Triangle(shape_attrs) => format!(
        "Width: {}, Height: {}",
        shape_attrs.width, shape_attrs.height
      ),
      Type::Square(shape_attrs) => format!(
        "Width: {}, Height: {}",
        shape_attrs.width, shape_attrs.height
      ),
      Type::Rectangle(shape_attrs) => format!(
        "Width: {}, Height: {}",
        shape_attrs.width, shape_attrs.height
      ),
      _ => format!("Width: Invalid, Height: Invalid"),
    };
    write!(f, "{}", attrs)
  }
}

impl Type {
  pub fn create(shape: &str) -> Option<Type> {
    match shape {
      "triangle" => Some(Type::Triangle(Attributes {
        ..Default::default()
      })),
      "circle" => Some(Type::Circle(Attributes {
        ..Default::default()
      })),
      "rectangle" => Some(Type::Square(Attributes {
        ..Default::default()
      })),
      "square" => Some(Type::Rectangle(Attributes {
        ..Default::default()
      })),
      _ => None,
    }
  }

  pub fn is_quadrangle(shape: &Option<Type>) -> bool {
    match shape {
      Some(Type::Rectangle(_)) => return true,
      Some(Type::Square(_)) => return true,
      _ => return false,
    }
  }

  pub fn non_guaranteed_shape(shape: &str) -> Option<Type> {
    if let "triangle" = shape {
      return Some(Type::Triangle(Attributes {
        ..Default::default()
      }));
    }

    if let "rectangle" = shape {
      return Some(Type::Rectangle(Attributes {
        ..Default::default()
      }));
    }
    if let "circle" = shape {
      return Some(Type::Circle(Attributes {
        ..Default::default()
      }));
    }

    if let "square" = shape {
      return Some(Type::Square(Attributes {
        ..Default::default()
      }));
    }

    Some(Type::Unknown)
  }
}
