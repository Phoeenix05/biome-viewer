use gpui::*;

actions!(biomeviewer, [Hide, HideOthers, ShowAll, Quit]);

struct BiomeViewer {}

impl Render for BiomeViewer {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    App::new().run(|cx| {
        cx.on_action(|_: &Hide, cx| cx.hide());
        cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
        cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
        cx.on_action(|_: &Quit, cx| {
            cx.spawn(|cx| async move {
                cx.update(|cx| cx.quit()).unwrap();
            })
            .detach();
        });

        let window_opts = WindowOptions {
            bounds: WindowBounds::Fixed(Bounds {
                size: size(GlobalPixels::from(1280.0), GlobalPixels::from(900.0)),
                ..Default::default()
            }),
            center: true,
            ..Default::default()
        };

        cx.open_window(window_opts, |cx| cx.new_view(|_cx| BiomeViewer {}));
    });
}
