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
    let grid_renderer = gl_render::GridRenderer::new(program);
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("x y ({},{}) XrelYrel ({},{})", x, y, xrel, yrel);
                }
                sdl2::event::Event::MouseButtonDown {
                    mouse_btn,
                    clicks,
                    x,
                    y,
                    ..
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
        let screen_resolution = window.drawable_size();
        grid_renderer.render(screen_resolution);

        window.gl_swap_window();
    }
    println!("Hello, world!");
}
