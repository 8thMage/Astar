// use a_star::Heap;
use a_star::gl_render;
use std::ffi::CString;
extern crate gl;
extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(4, 5);
    let window = video_subsystem
        .window("astar", 100, 100)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(
            0,
            0,
            window.drawable_size().0 as i32,
            window.drawable_size().1 as i32,
        ); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    let frag_shader = gl_render::Shader::from_frag_source(
        &CString::new(include_str!("../triangle.frag")).unwrap(),
    )
    .unwrap();
    let vert_shader = gl_render::Shader::from_vert_source(
        &CString::new(include_str!("../triangle.vert")).unwrap(),
    )
    .unwrap();
    let program = gl_render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
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
    }
    
    let screen_resolution_uniform_position = unsafe {gl::GetUniformLocation(program.id, b"screen_resolution".as_ptr() as *const i8)};
    assert!(screen_resolution_uniform_position != -1);
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {
                    x,
                    y,
                    xrel,
                    yrel,..
                } => {
                    println!("x y ({},{}) XrelYrel ({},{})", x, y, xrel, yrel);
                }
                sdl2::event::Event::MouseButtonDown {
                    mouse_btn,
                    clicks,
                    x,
                    y,..
                } => {
                    println!(
                        "x y ({},{}) mouseBtn {:?} clicks {}",
                        x, y, mouse_btn, clicks
                    );
                }
                // ...
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(
                0,
                0,
                window.drawable_size().0 as i32,
                window.drawable_size().1 as i32,
            ); // set viewport
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        unsafe {
            gl::BindVertexArray(vao);
            let screen_resolution = window.drawable_size();
            gl::Uniform2uiv(screen_resolution_uniform_position, 1, (&screen_resolution) as *const(u32,u32) as *const u32);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
    println!("Hello, world!");
}
