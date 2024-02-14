#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::SidePanel;
use eframe::App;

struct BiomeViewer {}

impl App for BiomeViewer {
    fn update(&mut self, cx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let size = cx.input(|i| i.viewport().outer_rect).unwrap();
        let panel_width: f32 = size.width() / 4.0;

        SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(panel_width)
            .show(cx, |_ui| {});

        // canvas
        use eframe::glow::HasContext as _;
        let gl = frame.gl().unwrap();

        unsafe {
            gl.disable(eframe::glow::SCISSOR_TEST);
            gl.viewport(
                panel_width as i32,
                0,
                size.width() as i32,
                size.height() as i32,
            );
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(eframe::glow::COLOR_BUFFER_BIT);
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280., 900.])
            .with_min_inner_size([1280., 900.]),
        ..Default::default()
    };

    _ = eframe::run_native(
        "biome viewer",
        options,
        Box::new(|_cx| Box::<BiomeViewer>::new(BiomeViewer {})),
    )
    .map_err(|err| eprintln!("{}", err));
}
