use egui::Vec2;

use crate::State;

pub struct Fractal{
    program: glow::Program
}

impl Fractal {
    pub fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = 
                (include_str!("shaders/shader.vs.glsl"), include_str!("shaders/shader.fs.glsl"));

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

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

            Self {
                program,
            }
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
        }
    }

    pub fn paint(&self, gl: &glow::Context, state: &State, dimensions: Vec2) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "iterations").as_ref(), 
                state.iterations as i32);
            gl.uniform_2_f32(
                gl.get_uniform_location(self.program, "scale").as_ref(),
                state.scale * dimensions.x,
                state.scale * dimensions.y
            );
            gl.uniform_2_f32(
                gl.get_uniform_location(self.program, "offset").as_ref(),
                state.offset.x,
                state.offset.y,
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "startColor").as_ref(),
                state.start_color[0],
                state.start_color[1],
                state.start_color[2],
            );
            gl.uniform_3_f32(
                gl.get_uniform_location(self.program, "endColor").as_ref(),
                state.end_color[0],
                state.end_color[1],
                state.end_color[2],
            );
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}