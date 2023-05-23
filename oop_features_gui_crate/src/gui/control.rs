use super::component::Component;

pub struct Control {}

impl Control {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Control {
    fn draw(&self) {
        println!("Draw control");
    }
}
