

/// Builder for describing the properties of a window
pub struct WindowDescription {
    pub title: String,
    pub resizable: bool,
    pub maximized: bool,
}

impl Default for WindowDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowDescription {
    pub fn new() -> Self {
        Self {
            title: "Morphorm Application".to_string(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();

        self
    }

    // Sets whether the window is resizable or not
    pub fn with_resizable(mut self, resizable: bool) -> Self {

    }
}