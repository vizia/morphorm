use morphorm as morph;
use morphorm::*;
use morphorm_ecs as ecs;
use vizia::prelude::*;

mod tree_panel;
use tree_panel::*;

mod canvas;
use canvas::*;

mod properties;
use properties::*;

pub enum AppEvent {
    Relayout,
    SetCanvasSize(f32, f32),

    AddChildNode,
    AddSiblingNode,
    DeleteNodes,
    SelectNode(Option<ecs::Entity>),
    MultiSelectNode(ecs::Entity),

    SetLeft(morph::Units),
    SetWidth(morph::Units),
    SetRight(morph::Units),

    AlignLeft,
    AlignCenter,
    AlignRight,
    FillWidth,

    SetTop(morph::Units),
    SetHeight(morph::Units),
    SetBottom(morph::Units),

    AlignTop,
    AlignMiddle,
    AlignBottom,
    FillHeight,

    SetLayoutType(&'static str),
    SetPositionType(&'static str),

    SetChildLeft(morph::Units),
    SetColBetween(morph::Units),
    SetChildRight(morph::Units),

    AlignChildLeft,
    AlignChildCenter,
    AlignChildRight,

    SetChildTop(morph::Units),
    SetRowBetween(morph::Units),
    SetChildBottom(morph::Units),

    AlignChildTop,
    AlignChildMiddle,
    AlignChildBottom,

    SetMinLeft(morph::Units),
    SetMinWidth(morph::Units),
    SetMinRight(morph::Units),
    SetMaxLeft(morph::Units),
    SetMaxWidth(morph::Units),
    SetMaxRight(morph::Units),
}

#[derive(Lens)]
pub struct AppData {
    canvas_width: f32,
    canvas_height: f32,

    world: ecs::World,
    root_node: ecs::Entity,
    selected_nodes: Option<Vec<ecs::Entity>>,

    left: morph::Units,
    width: morph::Units,
    right: morph::Units,

    top: morph::Units,
    height: morph::Units,
    bottom: morph::Units,

    layout_type: morph::LayoutType,
    layout_type_list: Vec<&'static str>,

    position_type: morph::PositionType,
    position_type_list: Vec<&'static str>,

    child_left: morph::Units,
    col_between: morph::Units,
    child_right: morph::Units,

    child_top: morph::Units,
    row_between: morph::Units,
    child_bottom: morph::Units,

    min_left: morph::Units,
    min_width: morph::Units,
    min_right: morph::Units,
    max_left: morph::Units,
    max_width: morph::Units,
    max_right: morph::Units,
}

impl Default for AppData {
    fn default() -> Self {
        AppData::new()
    }
}

impl AppData {
    pub fn new() -> Self {
        let mut world = ecs::World::default();
        let root_node = world.add(None);
        world.set_width(root_node, morph::Units::Pixels(600.0));
        world.set_height(root_node, morph::Units::Pixels(600.0));
        world.set_child_left(root_node, morph::Units::Stretch(1.0));
        world.set_child_right(root_node, morph::Units::Stretch(1.0));
        world.set_child_top(root_node, morph::Units::Stretch(1.0));
        world.set_child_bottom(root_node, morph::Units::Stretch(1.0));
        world.set_col_between(root_node, morph::Units::Stretch(1.0));
        world.set_row_between(root_node, morph::Units::Stretch(1.0));

        root_node.layout(&mut world.cache, &world.tree, &world.store);

        Self {
            canvas_width: 600.0,
            canvas_height: 600.0,

            world,
            root_node,
            selected_nodes: Some(vec![root_node]),

            left: morph::Units::Auto,
            width: morph::Units::Pixels(600.0),
            right: morph::Units::Auto,

            top: morph::Units::Auto,
            height: morph::Units::Pixels(600.0),
            bottom: morph::Units::Auto,

            layout_type: morph::LayoutType::Column,
            layout_type_list: vec!["Row", "Column"],

            position_type: morph::PositionType::ParentDirected,
            position_type_list: vec!["Parent Directed", "Self Directed"],

            child_left: morph::Units::Stretch(1.0),
            col_between: morph::Units::Stretch(1.0),
            child_right: morph::Units::Stretch(1.0),

            child_top: morph::Units::Stretch(1.0),
            row_between: morph::Units::Stretch(1.0),
            child_bottom: morph::Units::Stretch(1.0),

            min_left: morph::Units::Auto,
            min_width: morph::Units::Auto,
            min_right: morph::Units::Auto,
            max_left: morph::Units::Auto,
            max_width: morph::Units::Auto,
            max_right: morph::Units::Auto,
        }
    }

    fn sync(&mut self, node: ecs::Entity) {
        self.left = self.world.store.left.get(node).copied().unwrap_or_default();
        self.right = self.world.store.right.get(node).copied().unwrap_or_default();
        self.top = self.world.store.top.get(node).copied().unwrap_or_default();
        self.bottom = self.world.store.bottom.get(node).copied().unwrap_or_default();
        self.width = self.world.store.width.get(node).copied().unwrap_or_default();
        self.height = self.world.store.height.get(node).copied().unwrap_or_default();

        self.child_left = self.world.store.child_left.get(node).copied().unwrap_or_default();
        self.child_right = self.world.store.child_right.get(node).copied().unwrap_or_default();
        self.child_top = self.world.store.child_top.get(node).copied().unwrap_or_default();
        self.child_bottom = self.world.store.child_bottom.get(node).copied().unwrap_or_default();
        self.col_between = self.world.store.col_between.get(node).copied().unwrap_or_default();
        self.row_between = self.world.store.row_between.get(node).copied().unwrap_or_default();

        self.layout_type = self.world.store.layout_type.get(node).copied().unwrap_or_default();
        self.position_type = self.world.store.position_type.get(node).copied().unwrap_or_default();

        self.min_left = self.world.store.min_left.get(node).copied().unwrap_or_default();
        self.min_width = self.world.store.min_width.get(node).copied().unwrap_or_default();
        self.min_right = self.world.store.min_right.get(node).copied().unwrap_or_default();
        self.max_left = self.world.store.max_left.get(node).copied().unwrap_or_default();
        self.max_width = self.world.store.max_width.get(node).copied().unwrap_or_default();
        self.max_right = self.world.store.max_right.get(node).copied().unwrap_or_default();
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Relayout => {
                self.root_node.layout(&mut self.world.cache, &self.world.tree, &self.world.store);
                cx.needs_redraw();
            }

            AppEvent::SetCanvasSize(width, height) => {
                self.canvas_width = *width;
                self.canvas_height = *height;
                self.world.set_width(self.root_node, morph::Units::Pixels(*width));
                self.world.set_height(self.root_node, morph::Units::Pixels(*height));
                cx.emit(AppEvent::Relayout);
            }

            AppEvent::AddChildNode => {
                let node = if let Some(selected) = self.selected_nodes.as_ref().and_then(|nodes| nodes.last()) {
                    self.world.add(Some(*selected))
                } else {
                    self.world.add(Some(self.root_node))
                };

                self.world.set_width(node, morph::Units::Pixels(100.0));
                self.world.set_height(node, morph::Units::Pixels(100.0));
                self.world.set_left(node, morph::Units::Auto);
                self.world.set_right(node, morph::Units::Auto);
                self.world.set_top(node, morph::Units::Auto);
                self.world.set_bottom(node, morph::Units::Auto);
                self.world.set_child_left(node, morph::Units::Stretch(1.0));
                self.world.set_child_right(node, morph::Units::Stretch(1.0));
                self.world.set_child_top(node, morph::Units::Stretch(1.0));
                self.world.set_child_bottom(node, morph::Units::Stretch(1.0));
                self.world.set_col_between(node, morph::Units::Stretch(1.0));
                self.world.set_row_between(node, morph::Units::Stretch(1.0));

                cx.emit(AppEvent::SelectNode(Some(node)));
                cx.emit(AppEvent::Relayout);
            }

            AppEvent::AddSiblingNode => {
                let node = if let Some(parent) = self
                    .selected_nodes
                    .as_ref()
                    .and_then(|nodes| nodes.last())
                    .and_then(|selected| self.world.tree.get_parent(selected))
                {
                    self.world.add(Some(*parent))
                } else {
                    self.world.add(Some(self.root_node))
                };

                self.world.set_width(node, morph::Units::Pixels(100.0));
                self.world.set_height(node, morph::Units::Pixels(100.0));
                self.world.set_left(node, morph::Units::Auto);
                self.world.set_right(node, morph::Units::Auto);
                self.world.set_top(node, morph::Units::Auto);
                self.world.set_bottom(node, morph::Units::Auto);
                self.world.set_child_left(node, morph::Units::Stretch(1.0));
                self.world.set_child_right(node, morph::Units::Stretch(1.0));
                self.world.set_child_top(node, morph::Units::Stretch(1.0));
                self.world.set_child_bottom(node, morph::Units::Stretch(1.0));
                self.world.set_col_between(node, morph::Units::Stretch(1.0));
                self.world.set_row_between(node, morph::Units::Stretch(1.0));

                cx.emit(AppEvent::SelectNode(Some(node)));
                cx.emit(AppEvent::Relayout);
            }

            AppEvent::DeleteNodes => {
                if let Some(selected_nodes) = &self.selected_nodes {
                    for node in selected_nodes {
                        if *node != self.root_node {
                            self.world.remove(*node);
                        }
                    }

                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SelectNode(selected) => {
                if let Some(node) = selected {
                    self.selected_nodes = Some(vec![*node]);

                    self.sync(*node);
                } else {
                    self.selected_nodes = None;
                }

                cx.needs_redraw();
            }

            AppEvent::MultiSelectNode(node) => {
                if let Some(nodes) = &mut self.selected_nodes {
                    if let Some((index, _)) = nodes.iter().enumerate().find(|(_, &n)| n == *node) {
                        nodes.remove(index);
                    } else {
                        nodes.push(*node);
                    }
                } else {
                    self.selected_nodes = Some(vec![*node]);
                }

                self.sync(*node);

                cx.needs_redraw();
            }

            AppEvent::SetLeft(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_left(*selected, *value);
                        self.left = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetRight(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_right(*selected, *value);
                        self.right = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetTop(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_top(*selected, *value);
                        self.top = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetBottom(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_bottom(*selected, *value);
                        self.bottom = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetWidth(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_width(*selected, *value);
                        self.width = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetHeight(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_height(*selected, *value);
                        self.height = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetLayoutType(layout_type) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        let layout_type = match *layout_type {
                            "Row" => morph::LayoutType::Row,
                            _ => morph::LayoutType::Column,
                        };
                        self.world.set_layout_type(*selected, layout_type);
                        self.layout_type = layout_type;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetPositionType(position_type) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        let position_type = match *position_type {
                            "Parent Directed" => morph::PositionType::ParentDirected,
                            _ => morph::PositionType::SelfDirected,
                        };
                        self.world.set_position_type(*selected, position_type);
                        self.position_type = position_type;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetChildLeft(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_left(*selected, *value);
                        self.child_left = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetChildRight(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_right(*selected, *value);
                        self.child_right = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetChildTop(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_top(*selected, *value);
                        self.child_top = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetChildBottom(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_bottom(*selected, *value);
                        self.child_bottom = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetColBetween(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_col_between(*selected, *value);
                        self.col_between = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetRowBetween(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_row_between(*selected, *value);
                        self.row_between = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignLeft => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_left(*selected, morph::Units::Pixels(0.0));
                        self.world.set_right(*selected, morph::Units::Stretch(1.0));
                        self.left = morph::Units::Pixels(0.0);
                        self.right = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignRight => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_right(*selected, morph::Units::Pixels(0.0));
                        self.world.set_left(*selected, morph::Units::Stretch(1.0));
                        self.right = morph::Units::Pixels(0.0);
                        self.left = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignCenter => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_left(*selected, morph::Units::Stretch(1.0));
                        self.world.set_right(*selected, morph::Units::Stretch(1.0));
                        self.left = morph::Units::Stretch(1.0);
                        self.right = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::FillWidth => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_right(*selected, morph::Units::Pixels(0.0));
                        self.world.set_left(*selected, morph::Units::Pixels(0.0));
                        self.world.set_width(*selected, morph::Units::Stretch(1.0));
                        self.right = morph::Units::Pixels(0.0);
                        self.left = morph::Units::Pixels(0.0);
                        self.width = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignTop => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_top(*selected, morph::Units::Pixels(0.0));
                        self.world.set_bottom(*selected, morph::Units::Stretch(1.0));
                        self.top = morph::Units::Pixels(0.0);
                        self.bottom = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignBottom => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_bottom(*selected, morph::Units::Pixels(0.0));
                        self.world.set_top(*selected, morph::Units::Stretch(1.0));
                        self.bottom = morph::Units::Pixels(0.0);
                        self.top = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignMiddle => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_top(*selected, morph::Units::Stretch(1.0));
                        self.world.set_bottom(*selected, morph::Units::Stretch(1.0));
                        self.top = morph::Units::Stretch(1.0);
                        self.bottom = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::FillHeight => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_bottom(*selected, morph::Units::Auto);
                        self.world.set_top(*selected, morph::Units::Auto);
                        self.world.set_height(*selected, morph::Units::Stretch(1.0));
                        self.bottom = morph::Units::Auto;
                        self.top = morph::Units::Auto;
                        self.height = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildLeft => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_left(*selected, morph::Units::Pixels(0.0));
                        self.world.set_child_right(*selected, morph::Units::Stretch(1.0));
                        self.child_left = morph::Units::Auto;
                        self.child_right = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildRight => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_right(*selected, morph::Units::Pixels(0.0));
                        self.world.set_child_left(*selected, morph::Units::Stretch(1.0));
                        self.child_right = morph::Units::Auto;
                        self.child_left = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildCenter => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_left(*selected, morph::Units::Stretch(1.0));
                        self.world.set_child_right(*selected, morph::Units::Stretch(1.0));
                        self.child_left = morph::Units::Stretch(1.0);
                        self.child_right = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildTop => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_top(*selected, morph::Units::Pixels(0.0));
                        self.world.set_child_bottom(*selected, morph::Units::Stretch(1.0));
                        self.child_top = morph::Units::Auto;
                        self.child_bottom = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildBottom => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_bottom(*selected, morph::Units::Pixels(0.0));
                        self.world.set_child_top(*selected, morph::Units::Stretch(1.0));
                        self.child_bottom = morph::Units::Auto;
                        self.child_top = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::AlignChildMiddle => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_child_top(*selected, morph::Units::Stretch(1.0));
                        self.world.set_child_bottom(*selected, morph::Units::Stretch(1.0));
                        self.child_top = morph::Units::Stretch(1.0);
                        self.child_bottom = morph::Units::Stretch(1.0);
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMinLeft(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_min_left(*selected, *value);
                        self.min_left = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMinWidth(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_min_width(*selected, *value);
                        self.min_width = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMinRight(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_min_right(*selected, *value);
                        self.min_right = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMaxLeft(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_max_left(*selected, *value);
                        self.max_left = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMaxWidth(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_max_width(*selected, *value);
                        self.max_width = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SetMaxRight(value) => {
                if let Some(nodes) = &self.selected_nodes {
                    for selected in nodes {
                        self.world.set_max_right(*selected, *value);
                        self.max_right = *value;
                    }
                    cx.emit(AppEvent::Relayout);
                }
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("playground/src/theme.css").expect("Failed to find stylesheet");
        AppData::new().build(cx);
        HStack::new(cx, |cx| {
            // Treeview
            TreePanel::new(cx).width(Pixels(250.0));
            // Canvas
            CanvasView::new(cx).background_color(Color::rgb(29, 29, 29));
            // Properties
            PropertiesPanel::new(cx).width(Pixels(300.0));
        });
    })
    .title("Morphorm Playground")
    .inner_size((1000, 600))
    .ignore_default_theme()
    .run();
}
