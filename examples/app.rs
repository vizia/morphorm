

use morphorm::LayoutType;
use morphorm_ecs::*;

fn main() {
    let window_description = WindowBuilder::new()
        .with_title("Main Window");

    let app = Application::new(window_description, |state, window| {
        // let sub_window = Window::new(WindowBuilder::new().with_title("Subwindow"))
        //     .build(state, window, |builder| 
        //         builder
        //             .set_child_space(Units::Stretch(1.0))

        // );
        
        

        window
            .set_layout_type(state, LayoutType::Row)
            .set_col_between(state, Units::Pixels(50.0))
            .set_child_space(state, Units::Pixels(100.0));

        Element::new().build(state, window, |builder| builder);
        Element::new().build(state, window, |builder| 
            builder
                .set_width(Units::Pixels(300.0))
                .set_height(Units::Pixels(100.0))
        );
        Element::new().build(state, window, |builder| builder);
        Element::new().build(state, window, |builder| 
            builder
                .set_width(Units::Pixels(150.0))
                .set_unique_layer(true)
        );


    }).expect("Failed to create app");

    app.run();
}