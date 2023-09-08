use vizia::icons::{
    ICON_ARROW_AUTOFIT_HEIGHT, ICON_ARROW_AUTOFIT_WIDTH, ICON_ARROW_BAR_DOWN, ICON_ARROW_BAR_LEFT,
    ICON_ARROW_BAR_RIGHT, ICON_ARROW_BAR_UP, ICON_BOX_ALIGN_BOTTOM, ICON_BOX_ALIGN_BOTTOM_LEFT,
    ICON_BOX_ALIGN_BOTTOM_RIGHT, ICON_BOX_ALIGN_LEFT, ICON_BOX_ALIGN_RIGHT, ICON_BOX_ALIGN_TOP,
    ICON_BOX_ALIGN_TOP_LEFT, ICON_BOX_ALIGN_TOP_RIGHT, ICON_BOX_MARGIN, ICON_LAYOUT_ALIGN_BOTTOM,
    ICON_LAYOUT_ALIGN_LEFT, ICON_LAYOUT_ALIGN_RIGHT, ICON_LAYOUT_ALIGN_TOP, ICON_LAYOUT_DISTRIBUTE_HORIZONTAL,
    ICON_LAYOUT_DISTRIBUTE_VERTICAL,
};
use vizia::prelude::*;

use morphorm as morph;

use crate::{AppData, AppEvent};

pub struct PropertiesPanel {}

impl PropertiesPanel {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Space and Size").class("panel-title");

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Position Type");
                            PickList::new(cx, AppData::position_type_list, AppData::selected_position_type, true)
                                .on_select(|cx, index| cx.emit(AppEvent::SetPositionType(index)))
                                .width(Stretch(1.0));
                        })
                        .class("row");

                        HStack::new(cx, |cx| {
                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_BAR_RIGHT, AppData::left, AppEvent::SetLeft);
                            // }).height(Auto);

                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_AUTOFIT_WIDTH, AppData::width, AppEvent::SetWidth);
                            // }).height(Auto);

                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_BAR_LEFT, AppData::right, AppEvent::SetRight);
                            // }).height(Auto);
                        })
                        .class("row");

                        HStack::new(cx, |cx| {
                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_BAR_DOWN, AppData::top, AppEvent::SetTop);
                            // }).height(Auto);

                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_AUTOFIT_HEIGHT, AppData::height, AppEvent::SetHeight);
                            // }).height(Auto);

                            // VStack::new(cx, |cx| {
                            unit_box(cx, ICON_ARROW_BAR_UP, AppData::bottom, AppEvent::SetBottom);
                            // }).height(Auto);
                        })
                        .col_between(Pixels(10.0))
                        .height(Auto)
                        .class("row");
                    })
                    .class("panel");

                    Element::new(cx).class("divider");

                    // VStack::new(cx, |cx| {
                    //     Label::new(cx, "Space and Size Constraints").class("panel-title");

                    //     HStack::new(cx, |cx| {
                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "min-left", AppData::min_left, AppEvent::SetMinLeft);
                    //         });

                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "min-width", AppData::min_width, AppEvent::SetMinWidth);
                    //         });

                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "min-right", AppData::min_right, AppEvent::SetMinRight);
                    //         });
                    //     })
                    //     .class("row");

                    //     HStack::new(cx, |cx| {
                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "max-left", AppData::max_left, AppEvent::SetMaxLeft);
                    //         });

                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "max-width", AppData::max_width, AppEvent::SetMaxWidth);
                    //         });

                    //         VStack::new(cx, |cx| {
                    //             unit_box(cx, "max-right", AppData::max_right, AppEvent::SetMaxRight);
                    //         });
                    //     })
                    //     .col_between(Pixels(10.0))
                    //     .height(Auto)
                    //     .class("row");
                    // })
                    // .class("panel");

                    // VStack::new(cx, |cx| {
                    //     Label::new(cx, "Alignment").class("panel-title");
                    //     HStack::new(cx, |cx| {
                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignTop);
                    //                 cx.emit(AppEvent::AlignLeft);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignTop);
                    //                 cx.emit(AppEvent::AlignCenter);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignTop);
                    //                 cx.emit(AppEvent::AlignRight);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );
                    //     })
                    //     .left(Stretch(1.0))
                    //     .right(Stretch(1.0));

                    //     HStack::new(cx, |cx| {
                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignMiddle);
                    //                 cx.emit(AppEvent::AlignLeft);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignMiddle);
                    //                 cx.emit(AppEvent::AlignCenter);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignMiddle);
                    //                 cx.emit(AppEvent::AlignRight);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );
                    //     })
                    //     .left(Stretch(1.0))
                    //     .right(Stretch(1.0));

                    //     HStack::new(cx, |cx| {
                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignBottom);
                    //                 cx.emit(AppEvent::AlignLeft);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignBottom);
                    //                 cx.emit(AppEvent::AlignCenter);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );

                    //         Button::new(
                    //             cx,
                    //             |cx| {
                    //                 cx.emit(AppEvent::AlignBottom);
                    //                 cx.emit(AppEvent::AlignRight);
                    //             },
                    //             |cx| Label::new(cx, ""),
                    //         );
                    //     })
                    //     .left(Stretch(1.0))
                    //     .right(Stretch(1.0));
                    // })
                    // .class("panel")
                    // .class("align");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "Child Layout").class("panel-title");
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Layout Type").width(Auto);
                            PickList::new(cx, AppData::layout_type_list, AppData::selected_layout_type, true)
                                .on_select(|cx, index| cx.emit(AppEvent::SetLayoutType(index)))
                                .width(Stretch(1.0));
                        })
                        .col_between(Pixels(10.0))
                        .height(Auto)
                        .class("row");

                        HStack::new(cx, |cx| {
                            unit_box(cx, ICON_LAYOUT_ALIGN_LEFT, AppData::child_left, AppEvent::SetChildLeft);

                            unit_box(
                                cx,
                                ICON_LAYOUT_DISTRIBUTE_VERTICAL,
                                AppData::col_between,
                                AppEvent::SetColBetween,
                            );

                            unit_box(cx, ICON_LAYOUT_ALIGN_RIGHT, AppData::child_right, AppEvent::SetChildRight);
                        })
                        .col_between(Pixels(10.0))
                        .height(Auto)
                        .class("row");

                        HStack::new(cx, |cx| {
                            unit_box(cx, ICON_LAYOUT_ALIGN_TOP, AppData::child_top, AppEvent::SetChildTop);

                            unit_box(
                                cx,
                                ICON_LAYOUT_DISTRIBUTE_HORIZONTAL,
                                AppData::row_between,
                                AppEvent::SetRowBetween,
                            );

                            unit_box(cx, ICON_LAYOUT_ALIGN_BOTTOM, AppData::child_bottom, AppEvent::SetChildBottom);
                        })
                        .col_between(Pixels(10.0))
                        .height(Auto)
                        .class("row");
                    })
                    .class("panel");

                    Element::new(cx).class("divider");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "Child Alignment").class("panel-title");
                        HStack::new(cx, |cx| {
                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildTop);
                                    cx.emit(AppEvent::AlignChildLeft);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_TOP_LEFT),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildTop);
                                    cx.emit(AppEvent::AlignChildCenter);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_TOP),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildTop);
                                    cx.emit(AppEvent::AlignChildRight);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_TOP_RIGHT),
                            );
                        })
                        .height(Auto)
                        .child_left(Stretch(1.0))
                        .child_right(Stretch(1.0));

                        HStack::new(cx, |cx| {
                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildMiddle);
                                    cx.emit(AppEvent::AlignChildLeft);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_LEFT),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildMiddle);
                                    cx.emit(AppEvent::AlignChildCenter);
                                },
                                |cx| Icon::new(cx, ICON_BOX_MARGIN),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildMiddle);
                                    cx.emit(AppEvent::AlignChildRight);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_RIGHT),
                            );
                        })
                        .height(Auto)
                        .child_left(Stretch(1.0))
                        .child_right(Stretch(1.0));

                        HStack::new(cx, |cx| {
                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildBottom);
                                    cx.emit(AppEvent::AlignChildLeft);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_BOTTOM_LEFT),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildBottom);
                                    cx.emit(AppEvent::AlignChildCenter);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_BOTTOM),
                            );

                            Button::new(
                                cx,
                                |cx| {
                                    cx.emit(AppEvent::AlignChildBottom);
                                    cx.emit(AppEvent::AlignChildRight);
                                },
                                |cx| Icon::new(cx, ICON_BOX_ALIGN_BOTTOM_RIGHT),
                            );
                        })
                        .height(Auto)
                        .child_left(Stretch(1.0))
                        .child_right(Stretch(1.0));
                    })
                    .class("panel")
                    .class("align");

                    Element::new(cx).class("divider");
                })
                .width(Stretch(1.0))
                .height(Auto)
                .row_between(Pixels(1.0))
                .child_left(Pixels(10.0))
                .child_right(Pixels(10.0));
            })
            .size(Stretch(1.0));
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
    // Label::new(cx, label).text_wrap(false);
    HStack::new(cx, |cx| {
        Label::new(cx, label).text_wrap(false).class("icons");
        Textbox::new(cx, lens.map(|left| print_units(*left))).on_submit(move |cx, txt, _| {
            if let Some(val) = text_to_units(txt.as_ref()) {
                cx.emit(event(val));
            }
        });
    })
    .class("unit_box");
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
