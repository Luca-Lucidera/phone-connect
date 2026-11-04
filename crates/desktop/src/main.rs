// In release niente finestra della console: la UI si apre dall'icona nel tray.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct MainView;

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xe0e0e0))
            .child(div().text_xl().child("Phone Connect"))
            .child(format!("Protocollo v{}", phc_core::PROTOCOL_VERSION))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // La UI è solo una vista sul daemon: chiusa la finestra, il processo
        // termina e non occupa memoria finché non la si riapre dal tray.
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(480.), px(360.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MainView),
        )
        .expect("apertura della finestra principale");
        cx.activate(true);
    });
}
