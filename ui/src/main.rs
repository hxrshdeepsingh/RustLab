
use windui::prelude::*;

fn main() {
    let ui = Element::col()
        .fill()
        .padding(20)
        .spacing(12)
        .bg(Color::hex(0xF5F6FA))
        .child(
            Element::label("Rust Controller")
                .font_size(22.0)
                .height(32)
        )
        .child(
            Element::label("UDP Server: Stopped")
                .height(28)
        )
        .child(
            Element::button("Start UDP Server")
                .on_click(|_| {
                    println!("Start clicked!");
                })
        );

    App::new("Rust Controller", 360, 200)
        .content(ui)
        .run();
}
