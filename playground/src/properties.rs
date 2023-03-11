use vizia::{fonts::icons_names::DOWN, prelude::*};

use morphorm as morph;

use crate::{AppData, AppEvent};

pub struct PropertiesPanel {}

impl PropertiesPanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Space and Size").class("panel-title");

                HStack::new(cx, |cx| {
                    Label::new(cx, "Position Type").width(Auto);
                    Dropdown::new(
                        cx,
                        move |cx|
                        // A Label and an Icon
                        HStack::new(cx, move |cx|{
                            Label::new(cx, AppData::position_type.map(|position_type| match position_type {
                                morph::PositionType::ParentDirected => "Parent Directed",
                                morph::PositionType::SelfDirected => "Self Directed",
                            })).width(Auto);
                            Label::new(cx, DOWN).class("icon").width(Auto);
                        })
                        .child_left(Pixels(5.0))
                        .child_right(Pixels(5.0))
                        .col_between(Stretch(1.0)),
                        move |cx| {
                            List::new(cx, AppData::position_type_list, |cx, _, item| {
                                Label::new(cx, item)
                                    .width(Stretch(1.0))
                                    .child_top(Stretch(1.0))
                                    .child_bottom(Stretch(1.0))
                                    .child_left(Pixels(5.0))
                                    .bind(
                                        AppData::position_type.map(|position_type| match position_type {
                                            morph::PositionType::ParentDirected => "Parent Directed",
                                            morph::PositionType::SelfDirected => "Self Directed",
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
                                        cx.emit(AppEvent::SetPositionType(item.get(cx)));
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
                        unit_box(cx, "left", AppData::left, AppEvent::SetLeft);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "width", AppData::width, AppEvent::SetWidth);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "right", AppData::right, AppEvent::SetRight);
                    });
                })
                .class("row");

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "top", AppData::top, AppEvent::SetTop);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "height", AppData::height, AppEvent::SetHeight);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "bottom", AppData::bottom, AppEvent::SetBottom);
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx| {
                Label::new(cx, "Alignment").class("panel-title");
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignTop);
                            cx.emit(AppEvent::AlignLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignTop);
                            cx.emit(AppEvent::AlignCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignTop);
                            cx.emit(AppEvent::AlignRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignMiddle);
                            cx.emit(AppEvent::AlignLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignMiddle);
                            cx.emit(AppEvent::AlignCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignMiddle);
                            cx.emit(AppEvent::AlignRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignBottom);
                            cx.emit(AppEvent::AlignLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignBottom);
                            cx.emit(AppEvent::AlignCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignBottom);
                            cx.emit(AppEvent::AlignRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));
            })
            .class("panel")
            .class("align");

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
                        unit_box(cx, "child-left", AppData::child_left, AppEvent::SetChildLeft);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "col", AppData::col_between, AppEvent::SetColBetween);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-right", AppData::child_right, AppEvent::SetChildRight);
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-top", AppData::child_top, AppEvent::SetChildTop);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "row", AppData::row_between, AppEvent::SetRowBetween);
                    });

                    VStack::new(cx, |cx| {
                        unit_box(cx, "child-bottom", AppData::child_bottom, AppEvent::SetChildBottom);
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx| {
                Label::new(cx, "Child Alignment").class("panel-title");
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildTop);
                            cx.emit(AppEvent::AlignChildLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildTop);
                            cx.emit(AppEvent::AlignChildCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildTop);
                            cx.emit(AppEvent::AlignChildRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildMiddle);
                            cx.emit(AppEvent::AlignChildLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildMiddle);
                            cx.emit(AppEvent::AlignChildCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildMiddle);
                            cx.emit(AppEvent::AlignChildRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));

                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildBottom);
                            cx.emit(AppEvent::AlignChildLeft);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildBottom);
                            cx.emit(AppEvent::AlignChildCenter);
                        },
                        |cx| Label::new(cx, ""),
                    );

                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::AlignChildBottom);
                            cx.emit(AppEvent::AlignChildRight);
                        },
                        |cx| Label::new(cx, ""),
                    );
                })
                .left(Stretch(1.0))
                .right(Stretch(1.0));
            })
            .class("panel")
            .class("align");

            Element::new(cx).height(Stretch(1.0)).class("panel");
        })
    }
}

impl View for PropertiesPanel {
    fn element(&self) -> Option<&'static str> {
        Some("properties")
    }
}

fn unit_box(
    cx: &mut Context,
    label: &str,
    lens: impl Lens<Target = morph::Units>,
    event: impl 'static + Fn(morph::Units) -> AppEvent + Send + Sync,
) {
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
