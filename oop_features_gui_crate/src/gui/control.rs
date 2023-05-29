use core::panic;

use super::button::Button;
use super::component::Component;
use super::select::Select;
pub enum ControlType {
    Button,
    Select,
}

pub enum ControlEntity {
    Button(Button),
    Select(Select),
}

pub struct Control<T> {
    pub control_type: ControlType,
    pub entity: T,
}

impl Control<ControlEntity> {
    pub fn new(control_type: ControlType) -> Self {
        match control_type {
            ControlType::Button => Self {
                control_type,
                entity: ControlEntity::Button(Button::new()),
            },
            ControlType::Select => Self {
                control_type,
                entity: ControlEntity::Select(Select::new()),
            },
            _ => panic!("Not implemented"),
        }
    }
}

impl Component for Control<ControlEntity> {
    fn draw(&self) {
        println!("Draw control");
    }
}
