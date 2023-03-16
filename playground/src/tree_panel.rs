use morphorm::Node;
use vizia::prelude::*;

use crate::{
    icons::{ICON_CHEVRON_DOWN, ICON_LAYOUT_LIST, ICON_ROW_INSERT_BOTTOM, ICON_SQUARE_PLUS, ICON_TRASH},
    AppData, AppEvent,
};

pub struct TreePanel {}

impl TreePanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            // Toolbar
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::AddChildNode),
                    |cx| Label::new(cx, ICON_SQUARE_PLUS).class("icons"),
                )
                .class("add_button");
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::AddSiblingNode),
                    |cx| Label::new(cx, ICON_ROW_INSERT_BOTTOM).class("icons"),
                )
                .class("add_button");
                Button::new(cx, |cx| cx.emit(AppEvent::DeleteNodes), |cx| Label::new(cx, ICON_TRASH).class("icons"))
                    .class("add_button")
                    .class("danger");
            })
            .class("action_bar");
            Element::new(cx).class("divider");
            // Treeview
            VStack::new(cx, |cx| {
                Binding::new(cx, AppData::tree_changed, |cx, _| {
                    let children = if let Some(data) = cx.data::<AppData>() {
                        data.root_node.children(&data.world.tree).copied().collect::<Vec<_>>()
                    } else {
                        panic!();
                    };
                    for child in children.iter() {
                        let subchildren = if let Some(data) = cx.data::<AppData>() {
                            child.children(&data.world.tree).copied().collect::<Vec<_>>()
                        } else {
                            vec![]
                        };
                        build_tree(cx, *child, subchildren, 0);
                    }
                });
            })
            .class("treeview");
        })
    }
}

impl View for TreePanel {
    fn element(&self) -> Option<&'static str> {
        Some("treepanel")
    }
}

fn build_tree(cx: &mut Context, node: morphorm_ecs::Entity, children: Vec<morphorm_ecs::Entity>, level: usize) {
    HStack::new(cx, |cx| {
        Label::new(cx, if children.is_empty() { "" } else { ICON_CHEVRON_DOWN }).class("icons").width(Pixels(12.0));
        Label::new(cx, ICON_LAYOUT_LIST).class("icons");
        Label::new(cx, &format!("Entity {}", node.0)).width(Stretch(1.0)).child_left(Pixels(4.0)).on_press(move |ex| {
            if ex.modifiers().contains(Modifiers::CTRL) {
                ex.emit(AppEvent::MultiSelectNode(node));
            } else {
                ex.emit(AppEvent::SelectNode(Some(node)));
            }
        });
    })
    .child_left(Pixels(14.0 * level as f32 + 4.0))
    .class("tree_item")
    .on_press(move |ex| {
        if ex.modifiers().contains(Modifiers::CTRL) {
            ex.emit(AppEvent::MultiSelectNode(node));
        } else {
            ex.emit(AppEvent::SelectNode(Some(node)));
        }
    })
    .checked(AppData::selected_nodes.map(move |selected| {
        if let Some(selected_nodes) = selected {
            selected_nodes.iter().any(|s| *s == node)
        } else {
            false
        }
    }));
    for child in children.iter() {
        let subchildren = if let Some(data) = cx.data::<AppData>() {
            child.children(&data.world.tree).copied().collect::<Vec<_>>()
        } else {
            vec![]
        };
        build_tree(cx, *child, subchildren, level + 1);
    }
}
