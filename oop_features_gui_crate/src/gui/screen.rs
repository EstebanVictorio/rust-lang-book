use super::component::Component;

pub struct Screen {
    pub components: Vec<Box<dyn Component>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
