use glow::HasContext;

/// A compiled shader program
pub struct ShaderProgram {
    pub id: glow::Program,
}

impl ShaderProgram {
    /// Create a new shader program from vertex and fragment shader sources
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, vertex_src)?;
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src)?;

            let program = gl.create_program().map_err(|e| e.to_string())?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                gl.delete_program(program);
                gl.delete_shader(vertex_shader);
                gl.delete_shader(fragment_shader);
                return Err(format!("Shader program linking failed: {}", log));
            }

            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { id: program })
        }
    }

    unsafe fn compile_shader(
        gl: &glow::Context,
        shader_type: u32,
        source: &str,
    ) -> Result<glow::Shader, String> {
        let shader = gl.create_shader(shader_type).map_err(|e| e.to_string())?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            let type_name = match shader_type {
                glow::VERTEX_SHADER => "vertex",
                glow::FRAGMENT_SHADER => "fragment",
                _ => "unknown",
            };
            return Err(format!("{} shader compilation failed: {}", type_name, log));
        }

        Ok(shader)
    }

    pub fn delete(self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.id);
        }
    }
}
