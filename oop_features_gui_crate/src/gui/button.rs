pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            label: String::from(""),
        }
    }
}
