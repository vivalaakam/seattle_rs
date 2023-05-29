#[derive(Clone)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
