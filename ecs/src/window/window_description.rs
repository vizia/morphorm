

// pub struct WindowSize {
//     width: u32,
//     height: u32,
// }

// pub struct WindowPosition {
//     x: u32,
//     y: u32,
// }

// /// Builder for describing the properties of a window
// pub struct WindowDescription {
//     pub title: String,
//     pub inner_size: Option<WindowSize>,
//     pub min_inner_size: Option<WindowSize>,
//     pub max_inner_size: Option<WindowSize>,
//     pub position: Option<WindowPosition>,
//     pub resizable: bool,
//     pub maximized: bool,
// }

// impl Default for WindowDescription {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl WindowDescription {
//     pub fn new() -> Self {
//         Self {
//             title: "Morphorm Application".to_string(),
//         }
//     }

//     pub fn with_title(mut self, title: &str) -> Self {
//         self.title = title.to_owned();

//         self
//     }

//     // Sets whether the window is resizable or not
//     pub fn with_resizable(mut self, resizable: bool) -> Self {

//     }
// }