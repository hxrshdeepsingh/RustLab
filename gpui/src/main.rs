
use gpui::*;

struct MyApp;

impl Render for MyApp {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .child(
                div()
                    .text_2xl()
                    .text_color(white())
                    .child("Hello, GPUI!")
            )
            .child(
                div()
                    .text_color(rgb(0xaaaaaa))
                    .child("My first Rust desktop application")
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions::default(),
            |_, cx| cx.new(|_| MyApp),
        )
        .unwrap();
    });
}
