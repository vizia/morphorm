use vizia::{fonts::icons_names::DOWN, prelude::*};

use morphorm as morph;

use crate::{AppData, AppEvent};

pub struct PropertiesPanel {}

impl PropertiesPanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Horizontal Axis").class("panel-title");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "left", AppData::left, |val| AppEvent::SetLeft(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "width", AppData::width, |val| AppEvent::SetWidth(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "right", AppData::right, |val| AppEvent::SetRight(val));
                    });
                })
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx| {
                Label::new(cx, "Vertical Axis").class("panel-title");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "top", AppData::top, |val| AppEvent::SetTop(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "height", AppData::height, |val| AppEvent::SetHeight(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "bottom", AppData::bottom, |val| AppEvent::SetBottom(val));
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx|{
                Label::new(cx, "Alignment").class("panel-title");
                HStack::new(cx, |cx|{
                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignTop);
                        cx.emit(AppEvent::AlignLeft);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignTop);
                        cx.emit(AppEvent::AlignCenter);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignTop);
                        cx.emit(AppEvent::AlignRight);
                    }, |cx| Label::new(cx, ""));
                }).left(Stretch(1.0)).right(Stretch(1.0));

                HStack::new(cx, |cx|{
                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignMiddle);
                        cx.emit(AppEvent::AlignLeft);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignMiddle);
                        cx.emit(AppEvent::AlignCenter);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignMiddle);
                        cx.emit(AppEvent::AlignRight);
                    }, |cx| Label::new(cx, ""));
                }).left(Stretch(1.0)).right(Stretch(1.0));

                HStack::new(cx, |cx|{
                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignBottom);
                        cx.emit(AppEvent::AlignLeft);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignBottom);
                        cx.emit(AppEvent::AlignCenter);
                    }, |cx| Label::new(cx, ""));

                    Button::new(cx, |cx| {
                        cx.emit(AppEvent::AlignBottom);
                        cx.emit(AppEvent::AlignRight);
                    }, |cx| Label::new(cx, ""));
                }).left(Stretch(1.0)).right(Stretch(1.0));
            }).class("panel").class("align");

            VStack::new(cx, |cx| {
                Label::new(cx, "Child Layout").class("panel-title");
                HStack::new(cx, |cx| {
                    Label::new(cx, "Layout Type").width(Auto);
                    Dropdown::new(
                        cx,
                        move |cx|
                        // A Label and an Icon
                        HStack::new(cx, move |cx|{
                            Label::new(cx, AppData::layout_type.map(|layout_type| match layout_type {
                                morph::LayoutType::Row => "Row",
                                morph::LayoutType::Column => "Column",
                            })).width(Auto);
                            Label::new(cx, DOWN).class("icon").width(Auto);
                        })
                        .child_left(Pixels(5.0))
                        .child_right(Pixels(5.0))
                        .col_between(Stretch(1.0)),
                        move |cx| {
                            List::new(cx, AppData::layout_type_list, |cx, _, item| {
                                Label::new(cx, item)
                                    .width(Stretch(1.0))
                                    .child_top(Stretch(1.0))
                                    .child_bottom(Stretch(1.0))
                                    .child_left(Pixels(5.0))
                                    .bind(
                                        AppData::layout_type.map(|layout_type| match layout_type {
                                            morph::LayoutType::Row => "Row",
                                            morph::LayoutType::Column => "Column",
                                        }),
                                        move |handle, selected| {
                                            if item.get(handle.cx) == selected.get(handle.cx) {
                                                handle.background_color(Color::from("#4871ae"));
                                            } else {
                                                handle.background_color(Color::transparent());
                                            }
                                        },
                                    )
                                    .on_press(move |cx| {
                                        cx.emit(AppEvent::SetLayoutType(item.get(cx)));
                                        cx.emit(PopupEvent::Close);
                                    });
                            });
                        },
                    )
                    .width(Stretch(1.0));
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-left", AppData::child_left, |val| AppEvent::SetChildLeft(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "col", AppData::col_between, |val| AppEvent::SetColBetween(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-right", AppData::child_right, |val| AppEvent::SetChildRight(val));
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-top", AppData::child_top, |val| AppEvent::SetChildTop(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "row", AppData::row_between, |val| AppEvent::SetRowBetween(val));
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-bottom", AppData::child_bottom, |val| AppEvent::SetChildBottom(val));
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");
            })
            .class("panel");

            Element::new(cx).height(Stretch(1.0)).class("panel");
        })
    }
}

impl View for PropertiesPanel {
    fn element(&self) -> Option<&'static str> {
        Some("properties")
    }
}

fn unit_box(cx: &mut Context, label: &str, lens: impl Lens<Target = morph::Units>, event: impl 'static + Fn(morph::Units) -> AppEvent + Send + Sync) {
    Label::new(cx, label).text_wrap(false);
    Textbox::new(cx, lens.map(|left| print_units(*left))).on_submit(move |cx, txt, _| {
        if let Some(val) = text_to_units(txt.as_ref()) {
            cx.emit(event(val));
        }
    });
}

pub fn text_to_units(text: &str) -> Option<morph::Units> {
    match text {
        "auto" => Some(morph::Units::Auto),
        t => {
            if let Some(tt) = t.strip_suffix("px") {
                tt.parse::<f32>().ok().map(|v| morph::Units::Pixels(v))
            } else if let Some(tt) = t.strip_suffix("%") {
                tt.parse::<f32>().ok().map(|v| morph::Units::Percentage(v))
            } else if let Some(tt) = t.strip_suffix("s") {
                tt.parse::<f32>().ok().map(|v| morph::Units::Stretch(v))
            } else {
                t.parse::<f32>().ok().map(|v| morph::Units::Pixels(v))
            }
        }
    }
}

pub fn print_units(units: morph::Units) -> String {
    match units {
        morph::Units::Pixels(val) => format!("{}px", val),
        morph::Units::Percentage(val) => format!("{}%", val),
        morph::Units::Stretch(val) => format!("{}s", val),
        morph::Units::Auto => format!("auto"),
    }
}
