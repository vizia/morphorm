use vizia::prelude::*;

use crate::AppEvent;

pub struct TreePanel {}

impl TreePanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            // Toolbar
            Button::new(cx, |cx| cx.emit(AppEvent::AddChildNode), |cx| Label::new(cx, "Add Child")).class("add_button");
            Button::new(cx, |cx| cx.emit(AppEvent::AddSiblingNode), |cx| Label::new(cx, "Add Sibling"))
                .class("add_button");
            Button::new(cx, |cx| cx.emit(AppEvent::DeleteNodes), |cx| Label::new(cx, "Delete Nodes"))
                .class("add_button")
                .class("danger");
        })
    }
}

impl View for TreePanel {
    fn element(&self) -> Option<&'static str> {
        Some("treepanel")
    }
}
