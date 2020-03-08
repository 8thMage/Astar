use std;
use std::ffi::{CStr, CString};
use stb_image::image::LoadResult;
use super::map::Map;
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

pub struct Texture {
    index: gl::types::GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        let mut index: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut index as *mut u32);
            gl::BindTexture(gl::TEXTURE_2D, index);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            let color: [i32; 4] = [0, 0, 0, 0];
            gl::TexParameteriv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, color.as_ptr());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            let error = gl::GetError();
            if error != 0 {
                println!("define texture {}", error);
                panic!(error);
            };
        }
        Texture { index }
    }

    pub fn load_array(&self, map: &Map) {
        assert!(map.stride % 4 == 0);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.index);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::R8I as i32,
                map.stride,
                map.height,
                0,
                gl::RED_INTEGER,
                gl::UNSIGNED_BYTE,
                map.values.as_ptr() as *const std::ffi::c_void,
            );
            let error = gl::GetError();
            if error != 0 {
                println!("load array {}", error);
                panic!(error);
            };
        }
    }

    pub fn load_stb_image(&self, image: &LoadResult) {
        if let LoadResult::ImageU8(image_u8) = image {
            assert!(image_u8.depth * image_u8.width % 4 == 0);
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.index);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    image_u8.width as i32,
                    image_u8.height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    image_u8.data.as_ptr() as *const std::ffi::c_void,
                );
                let error = gl::GetError();
                if error != 0 {
                    println!("load array {}", error);
                    panic!(error);
                };
            }    
        }
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.index);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, (&self.index) as *const u32);
        }
    }
}

pub struct GridRenderer {
    program: Program,
    uniforms: Vec<gl::types::GLint>,
    vao: gl::types::GLuint,
}

impl GridRenderer {
    pub fn new(program: Program) -> Option<GridRenderer> {
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
            gl::EnableVertexAttribArray(0);
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
            gl::GetUniformLocation(program.id, CString::new("screen_resolution").unwrap().as_ptr())
        };
        assert!(screen_resolution_uniform_position != -1);
        let zoom_uniform_position = unsafe {
            gl::GetUniformLocation(program.id, CString::new("zoom").unwrap().as_ptr())
        };
        // assert!(zoom_uniform_position != -1);

        Some(GridRenderer {
            program,
            uniforms: [screen_resolution_uniform_position, zoom_uniform_position].to_vec(),
            vao,
        })
    }

    pub fn render(&self, screen_resolution: (u32, u32), texture: &Texture) {
        unsafe {
            self.program.set_used();
            texture.bind_texture();
            gl::BindVertexArray(self.vao);
            gl::Uniform2uiv(
                self.uniforms[0],
                1,
                (&screen_resolution) as *const (u32, u32) as *const u32,
            );
            gl::Uniform1f(self.uniforms[1], 10.);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
            let error = gl::GetError();
            if error != 0 {
                println!("render error {}", error);
                panic!(error);
            };
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

pub struct ImageRenderer {
    program: Program,
    uniforms: Vec<gl::types::GLint>,
    vao: gl::types::GLuint,
}

impl ImageRenderer {
    pub fn new(program: Program) -> Option<ImageRenderer> {
        program.set_used();
        let vertices: Vec<f32> = vec![
            -1., -1., 0.0, 1., 1., 0.0, -1.0, 1., 0.0,
            -1., -1., 0.0, 1., 1., 0.0, 1.0, -1.0, 0.0];
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
        let error = unsafe{gl::GetError()};

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
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
        let error = unsafe{gl::GetError()};

        let screen_resolution_uniform_position = unsafe {
            gl::GetUniformLocation(program.id, CString::new("screen_resolution").unwrap().as_ptr())
        };
        // assert!(screen_resolution_uniform_position != -1);
        let zoom_uniform_position = unsafe {
            gl::GetUniformLocation(program.id, CString::new("zoom").unwrap().as_ptr())
        };
        // assert!(zoom_uniform_position != -1);
        let error = unsafe{gl::GetError()};

        if error != 0 {
            println!("imageRenderer {}", error);
            panic!(error);
        };

        Some(ImageRenderer {
            program,
            uniforms: [screen_resolution_uniform_position, zoom_uniform_position].to_vec(),
            vao,
        })
    }

    pub fn render(&self, screen_resolution: (u32, u32), texture: &Texture) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
            self.program.set_used();
            texture.bind_texture();
            gl::BindVertexArray(self.vao);
            gl::Uniform2uiv(
                self.uniforms[0],
                1,
                (&screen_resolution) as *const (u32, u32) as *const u32,
            );
            gl::Uniform1f(self.uniforms[1], 10.);
            let error = gl::GetError();
            if error != 0 {
                println!("imageRenderer {}", error);
                panic!(error);
            }
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
            let error = gl::GetError();
            if error != 0 {
                println!("render error {}", error);
                panic!(error);
            };
        }
    }
}

impl Drop for ImageRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, (&self.vao) as *const u32);
        }
    }
}