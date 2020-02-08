use std;
use std::ffi::{CStr, CString};
fn empty_cstring_from_length(len: i32) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
    return error;
}
fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
    let id = unsafe { gl::CreateShader(kind) };
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        len = 1024;
        let error = empty_cstring_from_length(len);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(Shader { id })
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        shader_from_source(source, kind)
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        shader_from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        shader_from_source(source, gl::FRAGMENT_SHADER)
    }
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Program {
    pub id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(id, shader.id);
            }
            gl::LinkProgram(id);
            for shader in shaders {
                gl::DetachShader(id, shader.id);
            }
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = empty_cstring_from_length(len);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Program { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

pub struct GridRenderer {
    program: Program,
    uniforms: Vec<gl::types::GLint>,
    vao: gl::types::GLuint,
}
impl GridRenderer {
    pub fn new(program: Program) -> GridRenderer {
        program.set_used();
        let vertices: Vec<f32> = vec![-1., -3., 0.0, 3., 1., 0.0, -1.0, 1., 0.0];
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexArrayAttrib(vao, 0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::DeleteBuffers(1, (&vbo) as *const u32);
        };
        let screen_resolution_uniform_position = unsafe {
            gl::GetUniformLocation(program.id, b"screen_resolution".as_ptr() as *const i8)
        };
        assert!(screen_resolution_uniform_position != -1);

        GridRenderer {
            program,
            uniforms: [screen_resolution_uniform_position].to_vec(),
            vao,
        }
    }
    pub fn render(&self, screen_resolution: (u32, u32)) {
        unsafe {
            self.program.set_used();
            gl::BindVertexArray(self.vao);
            gl::Uniform2uiv(
                self.uniforms[0],
                1,
                (&screen_resolution) as *const (u32, u32) as *const u32,
            );
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
    }
}

impl Drop for GridRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, (&self.vao) as *const u32);
        }
    }
}
