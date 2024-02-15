#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate nalgebra_glm as glm;

use eframe::egui::SidePanel;
use eframe::glow::HasContext;
use eframe::App;

const VERTEX_SHADER: &'static str = include_str!("../assets/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../assets/fragment.glsl");

struct BiomeViewer {
    graphics: Graphics,
}

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
            gl.use_program(self.graphics.program);
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

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        unsafe { gl.unwrap().delete_program(self.graphics.program.unwrap()) };
    }
}

struct Graphics {
    offset: glm::Vec2,
    program: Option<eframe::glow::NativeProgram>,
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
        Box::new(|cx| {
            // initialize opengl shader program and compile shaders
            // as well as link them to the program

            let gl = cx.gl.as_ref().unwrap();

            let program: eframe::glow::NativeProgram = unsafe {
                use eframe::glow::HasContext as _;
                gl.create_program().unwrap()
            };

            let shader_sources = [
                (eframe::glow::VERTEX_SHADER, VERTEX_SHADER),
                (eframe::glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, source)| unsafe {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("failed to create shader");

                    gl.shader_source(shader, source);
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );

                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            unsafe {
                gl.link_program(program);
                assert!(
                    gl.get_program_link_status(program),
                    "{}",
                    gl.get_program_info_log(program)
                );

                for shader in shaders {
                    gl.detach_shader(program, shader);
                    gl.delete_shader(shader);
                }
            }

            // initialize the actual application state
            let graphics = Graphics {
                offset: glm::Vec2::zeros(),
                program: Some(program),
            };

            Box::new(BiomeViewer { graphics })
        }),
    )
    .map_err(|err| eprintln!("{}", err));
}
