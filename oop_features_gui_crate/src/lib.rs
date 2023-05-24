pub mod gui;

#[cfg(test)]
mod tests {
    use super::gui::component::Component;
    use super::gui::control::{Control, ControlType};
    use super::gui::screen::Screen;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_draws_simple_button() {
        let control = Control::new(ControlType::Button);
        control.draw();
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn it_draws_multiple_components() {
        let screen = Screen {
            components: vec![
                Box::new(Control::new(ControlType::Button)),
                Box::new(Control::new(ControlType::Select)),
            ],
        };
        screen.run();
    }
}
