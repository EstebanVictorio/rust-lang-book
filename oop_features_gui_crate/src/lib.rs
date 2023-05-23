pub mod gui;

#[cfg(test)]
mod tests {
    use super::gui::component::Component;
    use super::gui::control::Control;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn it_draws_simple_form() {
        let control = Control::new();
        control.draw();
        assert_eq!(1 + 1, 2);
    }
}
