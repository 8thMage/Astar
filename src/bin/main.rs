// use a_star::Heap;
use a_star::gl_render;
use std::ffi::CString;
extern crate gl;
extern crate sdl2;
use a_star::map::Map;
use a_star::vector::Vec2;
use a_star::Heap;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(3, 3);
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
    let grid_renderer = gl_render::GridRenderer::new(program).unwrap();
    let texture = gl_render::Texture::new();

    let mut arr: Vec<u8> = Vec::new();
    for i in 0..240 {
        arr.insert(i, 3);
    }
    arr[4] = 0;
    let mut map = Map {
        width: 12,
        stride: 12,
        height: 20,
        values: arr,
    };
    texture.load_array(&map);
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("x y ({},{}) XrelYrel ({},{})", x, y, xrel, yrel);
                }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    let rounded_x =
                        ((x * map.width) as f32 / window.drawable_size().0 as f32).floor() as i32;
                    let rounded_y = map.height
                        - ((y * map.height) as f32 / window.drawable_size().1 as f32).ceil() as i32;
                    let texel = map.value_mut((rounded_x, rounded_y));
                    *texel = 3u8 - *texel;
                    texture.load_array(&map);
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
        grid_renderer.render(screen_resolution, &texture);

        window.gl_swap_window();
    }
    println!("Hello, world!");
}

struct Node {
    father: Option<Box<Node>>,
    position: Vec2<i32>,
    real_distance: i32,
}

fn heuristic(start_point: Vec2<i32>, end_point: Vec2<i32>) -> f32 {
    let res = (start_point - end_point).norm();
    res
}

fn path_find(map: &Map, start_point: Vec2<i32>, end_point: Vec2<i32>) {
    let mut heap: Heap<f32, Node> = Heap::new();
    let mut hash = std::collections::HashSet::new();
    let start: Node = Node {
        position: start_point,
        father: None,
        real_distance: 0,
    };
    let value = heuristic(start_point, end_point);
    heap.push(0., start);
    let mut min_distance = 0;
    let neighborsDelta = vec!(Vec2{x:0,y:1},Vec2{x:1,y:0},Vec2{x:-1,y:0},Vec2{x:0,y:-1});
    while let Some(popped) = heap.pop() {
        let distance = popped.0;
        let node = popped.1;
        let position = node.position;
        for &delta in &neighborsDelta {
            let new_position = position + delta;
            let new_real_distance = node.real_distance + 1;
            let h = heuristic(new_position, end_point);
            let value = new_real_distance as f32 + h;
            let new_node = Node{
                position:new_position,
                real_distance: new_real_distance,
                father: Some(Box::new(node)),
            };
        }
        hash.insert(position);
    }
}
