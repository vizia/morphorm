use morphorm as morph;
use morphorm::*;
use morphorm_ecs as ecs;
use vizia::prelude::*;

mod canvas;
use canvas::*;

mod icons;

pub enum DemoPage {
    LayoutTypePage,
    PositionTypePage,
}

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
    demo_page: DemoPage,

    canvas_width: f32,
    canvas_height: f32,

    tree_changed: usize,

    world: ecs::World,
    root_node: ecs::Entity,
    selected_nodes: Option<Vec<ecs::Entity>>,
    hovered_node: Option<ecs::Entity>,

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
        world.set_layout_type(root_node, morph::LayoutType::Row);

        for _ in 0..4 {
            let node = world.add(Some(root_node));
            world.set_width(node, morph::Units::Pixels(100.0));
            world.set_height(node, morph::Units::Pixels(100.0));
            if let Some(red) = world.store.red.get_mut(node) {
                *red = 28;
            }
            if let Some(green) = world.store.green.get_mut(node) {
                *green = 143;
            }
            if let Some(blue) = world.store.blue.get_mut(node) {
                *blue = 188;
            }
        }

        root_node.layout(&mut world.cache, &world.tree, &world.store);

        Self {
            demo_page: DemoPage::LayoutTypePage,

            canvas_width: 600.0,
            canvas_height: 600.0,

            tree_changed: 0,

            world,
            root_node,
            selected_nodes: Some(vec![root_node]),
            hovered_node: None,

            left: morph::Units::Auto,
            width: morph::Units::Pixels(600.0),
            right: morph::Units::Auto,

            top: morph::Units::Auto,
            height: morph::Units::Pixels(600.0),
            bottom: morph::Units::Auto,

            layout_type: morph::LayoutType::Row,
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

    fn switch_demo_page(&mut self) {
        self.world.clear();
        match self.demo_page {
            DemoPage::LayoutTypePage => {
                println!("or this");
                let root_node = self.world.add(None);
                self.world.set_width(root_node, morph::Units::Pixels(600.0));
                self.world.set_height(root_node, morph::Units::Pixels(600.0));
                self.world.set_child_left(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_right(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_top(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_bottom(root_node, morph::Units::Stretch(1.0));
                self.world.set_col_between(root_node, morph::Units::Stretch(1.0));
                self.world.set_row_between(root_node, morph::Units::Stretch(1.0));
                self.world.set_layout_type(root_node, morph::LayoutType::Row);

                for _ in 0..4 {
                    let node = self.world.add(Some(root_node));
                    self.world.set_width(node, morph::Units::Pixels(100.0));
                    self.world.set_height(node, morph::Units::Pixels(100.0));
                    if let Some(red) = self.world.store.red.get_mut(node) {
                        *red = 28;
                    }
                    if let Some(green) = self.world.store.green.get_mut(node) {
                        *green = 143;
                    }
                    if let Some(blue) = self.world.store.blue.get_mut(node) {
                        *blue = 188;
                    }
                }

                self.root_node = root_node;
            }

            DemoPage::PositionTypePage => {
                let root_node = self.world.add(None);
                self.world.set_width(root_node, morph::Units::Pixels(600.0));
                self.world.set_height(root_node, morph::Units::Pixels(600.0));
                self.world.set_child_left(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_right(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_top(root_node, morph::Units::Stretch(1.0));
                self.world.set_child_bottom(root_node, morph::Units::Stretch(1.0));
                self.world.set_col_between(root_node, morph::Units::Stretch(1.0));
                self.world.set_row_between(root_node, morph::Units::Stretch(1.0));
                self.world.set_layout_type(root_node, morph::LayoutType::Row);

                for n in 0..4 {
                    let node = self.world.add(Some(root_node));
                    self.world.set_width(node, morph::Units::Pixels(100.0));
                    self.world.set_height(node, morph::Units::Pixels(100.0));
                    if let Some(red) = self.world.store.red.get_mut(node) {
                        *red = 28;
                    }
                    if let Some(green) = self.world.store.green.get_mut(node) {
                        *green = 143;
                    }
                    if let Some(blue) = self.world.store.blue.get_mut(node) {
                        *blue = 188;
                    }
                    if n == 2 {
                        self.selected_nodes = Some(vec![node]);
                        if let Some(red) = self.world.store.red.get_mut(node) {
                            *red = 188;
                        }
                        if let Some(green) = self.world.store.green.get_mut(node) {
                            *green = 119;
                        }
                        if let Some(blue) = self.world.store.blue.get_mut(node) {
                            *blue = 28;
                        }
                    }
                }

                self.root_node = root_node;
            }
        }
        self.sync(self.root_node);
        self.root_node.layout(&mut self.world.cache, &self.world.tree, &self.world.store);
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

                self.tree_changed += 1;

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

                self.tree_changed += 1;

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

                    self.tree_changed += 1;

                    cx.emit(AppEvent::Relayout);
                }
            }

            AppEvent::SelectNode(_selected) => {
                // if let Some(node) = selected {
                //     self.selected_nodes = Some(vec![*node]);

                //     self.sync(*node);
                // } else {
                //     self.selected_nodes = None;
                // }

                cx.needs_redraw();
            }

            AppEvent::MultiSelectNode(_node) => {
                // if let Some(nodes) = &mut self.selected_nodes {
                //     if let Some((index, _)) = nodes.iter().enumerate().find(|(_, &n)| n == *node) {
                //         nodes.remove(index);
                //     } else {
                //         nodes.push(*node);
                //     }
                // } else {
                //     self.selected_nodes = Some(vec![*node]);
                // }

                // self.sync(*node);

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
        cx.add_stylesheet("demo/src/theme.css").expect("Failed to find stylesheet");
        cx.add_fonts_mem(&[include_bytes!("tabler-icons.ttf")]);
        let mut app_data = AppData::new();
        app_data.demo_page = DemoPage::PositionTypePage;
        app_data.switch_demo_page();
        app_data.build(cx);
        // ZStack::new(cx, |cx| {
        //     // Canvas
        //     CanvasView::new(cx).background_color(Color::rgb(29, 29, 29));
        //     // Properties
        //     // PropertiesPanel::new(cx).width(Pixels(300.0)).left(Stretch(1.0));
        //     Label::new(cx, "The layout type determines the direction which child elements will be stacked in, either horizontally in a row, or vertically in a column.")
        //         .font_size(16.0)
        //         .width(Pixels(400.0))
        //         .left(Pixels(50.0))
        //         .top(Pixels(50.0));
        //     HStack::new(cx, |cx|{
        //         Button::new(cx, |ex| ex.emit(AppEvent::SetLayoutType("Row")), |cx|{
        //             Label::new(cx, "Row")
        //         })
        //         .class("toggle_button").class("toggle_left")
        //         .checked(AppData::layout_type.map(|layout_type| *layout_type == morph::LayoutType::Row));
        //         Button::new(cx, |ex| ex.emit(AppEvent::SetLayoutType("Column")), |cx|{
        //             Label::new(cx, "Column")
        //         })
        //         .class("toggle_button").class("toggle_right")
        //         .checked(AppData::layout_type.map(|layout_type| *layout_type == morph::LayoutType::Column));
        //     }).left(Stretch(1.0)).top(Pixels(50.0)).right(Pixels(50.0)).width(Pixels(100.0)).size(Auto);
        // });

        // ZStack::new(cx, |cx| {
        //     // Canvas
        //     CanvasView::new(cx).background_color(Color::rgb(29, 29, 29));
        //     // Properties
        //     // PropertiesPanel::new(cx).width(Pixels(300.0)).left(Stretch(1.0));
        //     Label::new(cx, "The position type property determines whether a node should be positioned in-line with its siblings in a stack, or out-of-line and independently of its siblings.")
        //         .font_size(16.0)
        //         .width(Pixels(400.0))
        //         .left(Pixels(50.0))
        //         .top(Pixels(50.0));
        //     HStack::new(cx, |cx|{
        //         Button::new(cx, |ex| ex.emit(AppEvent::SetPositionType("Parent Directed")), |cx|{
        //             Label::new(cx, "Parent Directed")
        //         })
        //         .width(Pixels(100.0))
        //         .class("toggle_button").class("toggle_left")
        //         .checked(AppData::position_type.map(|position_type| *position_type == morph::PositionType::ParentDirected));
        //         Button::new(cx, |ex| ex.emit(AppEvent::SetPositionType("Self Directed")), |cx|{
        //             Label::new(cx, "Self Directed")
        //         })
        //         .width(Pixels(100.0))
        //         .class("toggle_button").class("toggle_right")
        //         .checked(AppData::position_type.map(|position_type| *position_type == morph::PositionType::SelfDirected));
        //     }).left(Stretch(1.0)).top(Pixels(50.0)).right(Pixels(50.0)).width(Pixels(100.0)).size(Auto);
        // });

        // ZStack::new(cx, |cx| {
        //     // Canvas
        //     CanvasView::new(cx).background_color(Color::rgb(29, 29, 29));
        //     // Properties
        //     // PropertiesPanel::new(cx).width(Pixels(300.0)).left(Stretch(1.0));
        //     Label::new(cx, "The size of a node is determined by its `width` and `height` properties, which are specified with `Units` which have four variants.")
        //         .font_size(16.0)
        //         .width(Pixels(400.0))
        //         .left(Pixels(50.0))
        //         .top(Pixels(50.0));

        //     VStack::new(cx, |cx|{
        //         for i in 0..4 {
        //             let current_option = index_to_units(i);
        //             HStack::new(cx, move |cx| {
        //                 RadioButton::new(
        //                     cx,
        //                     AppData::width.map(move |option| units_same(option, &current_option)),
        //                 )
        //                 .on_select(move |cx| {
        //                     cx.emit(AppEvent::SetWidth(current_option));
        //                     cx.emit(AppEvent::SetHeight(current_option));
        //                 })
        //                 .id(format!("button_{i}"));
        //                 Label::new(cx, units_to_label(current_option))
        //                     .width(Pixels(80.0))
        //                     .describing(format!("button_{i}"));
        //                 Textbox::new(cx, AppData::width.map(|width| print_units(*width))).on_submit(move |cx, txt, _| {
        //                     if let Some(val) = text_to_units(txt.as_ref()) {
        //                         cx.emit(AppEvent::SetWidth(val));
        //                         cx.emit(AppEvent::SetHeight(val));
        //                     }
        //                 })
        //                 .visibility(AppData::width.map(move |option| units_same(option, &current_option) && *option != morph::Units::Auto))
        //                 .class("unit_box").width(Pixels(100.0));
        //             })
        //             .size(Auto)
        //             .child_top(Stretch(1.0))
        //             .child_bottom(Stretch(1.0))
        //             .col_between(Pixels(5.0));
        //         }
        //     }).row_between(Pixels(8.0)).left(Stretch(1.0)).top(Pixels(50.0)).right(Pixels(50.0)).width(Pixels(100.0)).size(Auto);
        // });


        ZStack::new(cx, |cx| {
            // Canvas
            CanvasView::new(cx).background_color(Color::rgb(29, 29, 29));
            // Properties
            // PropertiesPanel::new(cx).width(Pixels(300.0)).left(Stretch(1.0));
            Label::new(cx, "The size of a node is determined by its `width` and `height` properties, which are specified with `Units` which have four variants.")
                .font_size(16.0)
                .width(Pixels(400.0))
                .left(Pixels(50.0))
                .top(Pixels(50.0));

            VStack::new(cx, |cx|{
                for i in 0..4 {
                    let current_option = index_to_units(i);
                    HStack::new(cx, move |cx| {
                        RadioButton::new(
                            cx,
                            AppData::width.map(move |option| units_same(option, &current_option)),
                        )
                        .on_select(move |cx| {
                            cx.emit(AppEvent::SetWidth(current_option));
                            cx.emit(AppEvent::SetHeight(current_option));
                        })
                        .id(format!("button_{i}"));
                        Label::new(cx, units_to_label(current_option))
                            .width(Pixels(80.0))
                            .describing(format!("button_{i}"));
                        Textbox::new(cx, AppData::width.map(|width| print_units(*width))).on_submit(move |cx, txt, _| {
                            if let Some(val) = text_to_units(txt.as_ref()) {
                                cx.emit(AppEvent::SetWidth(val));
                                cx.emit(AppEvent::SetHeight(val));
                            }
                        })
                        .visibility(AppData::width.map(move |option| units_same(option, &current_option) && *option != morph::Units::Auto))
                        .class("unit_box").width(Pixels(100.0));
                    })
                    .size(Auto)
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0))
                    .col_between(Pixels(5.0));
                }
            }).row_between(Pixels(8.0)).left(Stretch(1.0)).top(Pixels(50.0)).right(Pixels(50.0)).width(Pixels(100.0)).size(Auto);
        });



    })
    .title("Morphorm Demo")
    .inner_size((1000, 600))
    .ignore_default_theme()
    .run();
}

// fn unit_box(
//     cx: &mut Context,
//     label: &str,
//     lens: impl Lens<Target = morph::Units>,
//     event: impl 'static + Fn(morph::Units) -> AppEvent + Send + Sync,
// ) {
//     // Label::new(cx, label).text_wrap(false);
//     VStack::new(cx, |cx| {
//         Label::new(cx, label).text_wrap(false).class("icons");
//         Textbox::new(cx, lens.map(|left| print_units(*left))).on_submit(move |cx, txt, _| {
//             if let Some(val) = text_to_units(txt.as_ref()) {
//                 cx.emit(event(val));
//             }
//         });
//     })
//     .class("unit_box");
// }

fn index_to_units(index: usize) -> morph::Units {
    match index {
        0 => morph::Units::Pixels(100.0),
        1 => morph::Units::Percentage(25.0),
        2 => morph::Units::Stretch(1.0),
        3 => morph::Units::Auto,
        _ => unreachable!(),
    }
}

fn units_to_label(units: morph::Units) -> &'static str {
    match units {
        morph::Units::Pixels(_) => "Pixels",
        morph::Units::Percentage(_) => "Percentage",
        morph::Units::Stretch(_) => "Stretch",
        morph::Units::Auto => "Auto",
    }
}

fn units_same(first: &morph::Units, second: &morph::Units) -> bool {
    matches!(
        (first, second),
        (morph::Units::Pixels(_), morph::Units::Pixels(_))
            | (morph::Units::Percentage(_), morph::Units::Percentage(_))
            | (morph::Units::Stretch(_), morph::Units::Stretch(_))
            | (morph::Units::Auto, morph::Units::Auto)
    )
}

pub fn text_to_units(text: &str) -> Option<morph::Units> {
    match text {
        "auto" => Some(morph::Units::Auto),
        t => {
            if let Some(tt) = t.strip_suffix("px") {
                tt.parse::<f32>().ok().map(morph::Units::Pixels)
            } else if let Some(tt) = t.strip_suffix('%') {
                tt.parse::<f32>().ok().map(morph::Units::Percentage)
            } else if let Some(tt) = t.strip_suffix('s') {
                tt.parse::<f32>().ok().map(morph::Units::Stretch)
            } else {
                t.parse::<f32>().ok().map(morph::Units::Pixels)
            }
        }
    }
}

pub fn print_units(units: morph::Units) -> String {
    match units {
        morph::Units::Pixels(val) => format!("{val}px"),
        morph::Units::Percentage(val) => format!("{val}%"),
        morph::Units::Stretch(val) => format!("{val}s"),
        morph::Units::Auto => String::from("auto"),
    }
}
