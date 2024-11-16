pub struct App {
    pub items: Vec<String>,    // Example: A list of items to display
    pub selected_index: usize, // Example: Selected item index
}

impl App {
    pub fn new() -> Self {
        Self {
            items: vec!["Item 1".into(), "Item 2".into(), "Item 3".into()],
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
