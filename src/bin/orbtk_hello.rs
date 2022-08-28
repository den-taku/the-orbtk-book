use orbtk::prelude::*;

fn main() {
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk-Book - Chapter 1.2")
                .position((100.0, 100.0))
                .size(420.0, 140.0)
                .child(
                    TextBlock::new()
                        .font_size(28)
                        .h_align("center")
                        .text("Hey OrbTk!")
                        .v_align("center")
                        .style("color: #ff0000")
                        .build(ctx),
                )
                .style("background-color: #000000")
                .build(ctx)
        })
        .run();
}
