use vizia::{fonts::icons_names::DOWN, prelude::*};

use morphorm as morph;
use morphorm_ecs as ecs;

use crate::{AppData, AppEvent};

pub struct PropertiesPanel {}

impl PropertiesPanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Horizontal Axis").class("panel-title");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "left");
                        Textbox::new(cx, AppData::left.map(|left| print_units(*left))).on_submit(|cx, txt, flag| {
                            if let Some(val) = text_to_units(txt.as_ref()) {
                                cx.emit(AppEvent::SetLeft(val));
                            }
                        });
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "width");
                        Textbox::new(cx, AppData::width.map(|width| print_units(*width))).on_submit(|cx, txt, flag| {
                            if let Some(val) = text_to_units(txt.as_ref()) {
                                cx.emit(AppEvent::SetWidth(val));
                            }
                        });
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "right");
                        Textbox::new(cx, AppData::right.map(|right| print_units(*right))).on_submit(|cx, txt, flag| {
                            if let Some(val) = text_to_units(txt.as_ref()) {
                                cx.emit(AppEvent::SetRight(val));
                            }
                        });
                    });
                })
                .class("row");

                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignLeft), |cx| Label::new(cx, "Align Left"));
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignCenter), |cx| Label::new(cx, "Align Center"));
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignRight), |cx| Label::new(cx, "Align Right"));
                    Button::new(cx, |cx| cx.emit(AppEvent::FillWidth), |cx| Label::new(cx, "Fill Width"));
                })
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx| {
                Label::new(cx, "Vertical Axis").class("panel-title");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "top");
                        Textbox::new(cx, AppData::top.map(|left| print_units(*left))).on_submit(|cx, txt, flag| {
                            if let Some(val) = text_to_units(txt.as_ref()) {
                                cx.emit(AppEvent::SetTop(val));
                            }
                        });
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "height");
                        Textbox::new(cx, AppData::height.map(|width| print_units(*width))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetHeight(val));
                                }
                            },
                        );
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "bottom");
                        Textbox::new(cx, AppData::bottom.map(|right| print_units(*right))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetBottom(val));
                                }
                            },
                        );
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");

                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignTop), |cx| Label::new(cx, "Align Top"));
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignMiddle), |cx| Label::new(cx, "Align Middle"));
                    Button::new(cx, |cx| cx.emit(AppEvent::AlignBottom), |cx| Label::new(cx, "Align Bottom"));
                    Button::new(cx, |cx| cx.emit(AppEvent::FillHeight), |cx| Label::new(cx, "Fill Height"));
                })
                .class("row");
            })
            .class("panel");

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
                        Label::new(cx, "child-left").text_wrap(false);
                        Textbox::new(cx, AppData::child_left.map(|left| print_units(*left))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetChildLeft(val));
                                }
                            },
                        );
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "col").text_wrap(false);
                        Textbox::new(cx, AppData::col_between.map(|width| print_units(*width))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetColBetween(val));
                                }
                            },
                        );
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "child-right").text_wrap(false);
                        Textbox::new(cx, AppData::child_right.map(|right| print_units(*right))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetChildRight(val));
                                }
                            },
                        );
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "child-top").text_wrap(false);
                        Textbox::new(cx, AppData::child_top.map(|left| print_units(*left))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetChildTop(val));
                                }
                            },
                        );
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "row").text_wrap(false);
                        Textbox::new(cx, AppData::row_between.map(|width| print_units(*width))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetRowBetween(val));
                                }
                            },
                        );
                    });

                    VStack::new(cx, |cx| {
                        Label::new(cx, "child-bottom").text_wrap(false);
                        Textbox::new(cx, AppData::child_bottom.map(|right| print_units(*right))).on_submit(
                            |cx, txt, flag| {
                                if let Some(val) = text_to_units(txt.as_ref()) {
                                    cx.emit(AppEvent::SetChildBottom(val));
                                }
                            },
                        );
                    });
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .class("row");
            })
            .class("panel");

            VStack::new(cx, |cx| {}).height(Stretch(1.0)).class("panel");
        })
    }
}

impl View for PropertiesPanel {
    fn element(&self) -> Option<&'static str> {
        Some("properties")
    }
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
