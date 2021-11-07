pub struct App {
    pub input: String,
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
        }
    }

    // pub fn get_input(self: &Self) -> &str {
    //     &self.input
    // }
}
