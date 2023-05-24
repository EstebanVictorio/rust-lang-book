pub struct Select {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Select {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            options: vec![],
        }
    }
}
