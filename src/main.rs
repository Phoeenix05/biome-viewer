#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate nalgebra_glm as glm;

use eframe::egui::{RichText, SidePanel};
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
        let panel_width: f32 = size.width() / 4.0; // this is required to determine
                                                   // where to put the gl vieport

        SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(panel_width)
            .show(cx, |ui| {
                ui.label(RichText::new("biome-viewer").size(20.0));
            });

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

            // setup
            gl.use_program(Some(self.graphics.program));

            // TODO: drawing

            // cleanup
            gl.use_program(None);
        }
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        unsafe { gl.unwrap().delete_program(self.graphics.program) };
    }
}

struct Graphics {
    offset: glm::Vec2,
    program: eframe::glow::NativeProgram,
    vao: eframe::glow::NativeVertexArray,
    vbo: eframe::glow::NativeBuffer,
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

            let shader_map = |(shader_type, source): &(u32, &str)| unsafe {
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
            };

            let shaders: Vec<_> = shader_sources.iter().map(shader_map).collect();

            unsafe {
                // these two lines as `layout(location = ...)` doesn't seem to work
                gl.bind_attrib_location(program, 0, "position");
                gl.bind_attrib_location(program, 1, "color");

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

            let (vao, vbo) = unsafe {
                let vao = gl.create_vertex_array().unwrap();
                gl.bind_vertex_array(Some(vao));

                let vbo = gl.create_buffer().unwrap();
                gl.bind_buffer(eframe::glow::ARRAY_BUFFER, Some(vbo));
                gl.buffer_data_u8_slice(
                    eframe::glow::ARRAY_BUFFER,
                    bytemuck::cast_slice(&[0.0, 1.0, -1.0, -1.0, 1.0, -1.0]),
                    eframe::glow::STATIC_DRAW,
                );

                // Position attribute (location = 0)
                gl.enable_vertex_attrib_array(0);
                gl.vertex_attrib_pointer_f32(0, 2, eframe::glow::FLOAT, false, 0, 0);
                gl.vertex_attrib_divisor(0, 1);

                // Color attribute (location = 1)
                gl.enable_vertex_attrib_array(1);
                gl.vertex_attrib_pointer_f32(1, 4, eframe::glow::FLOAT, false, 0, 0);
                gl.vertex_attrib_divisor(1, 1);

                (vao, vbo)
            };

            // initialize the actual application state;
            Box::new(BiomeViewer {
                graphics: Graphics {
                    offset: glm::Vec2::zeros(),
                    program,
                    vao,
                    vbo,
                },
            })
        }),
    )
    .map_err(|err| eprintln!("{}", err));
}
