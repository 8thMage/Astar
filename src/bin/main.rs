// use a_star::Heap;
use a_star::gl_render;
use std::ffi::CString;
extern crate gl;
extern crate sdl2;
use a_star::map::Map;
use a_star::vector::Vec2;
use a_star::data_structures::heap_hash::HeapHash;
use a_star::path_finding::path_find;
use a_star::tank::_Tank;
extern crate stb_image;
fn main() {
    let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
    heap.push(2, 0, 0);
    heap.push(-1, -1, -1);
    heap.push(-2, -2, -2);
    heap.push(1, 1, 1);
    heap.push(-3, 0, 1);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(3, 3);
    let window = video_subsystem
        .window("astar", 1000, 1000)
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
        &CString::new(include_str!("../map.frag")).unwrap(),
    ).unwrap();
    let vert_shader = gl_render::Shader::from_vert_source(
        &CString::new(include_str!("../map.vert")).unwrap(),
    ).unwrap();
    let program = gl_render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    let image_program;
    {
        let frag_shader = gl_render::Shader::from_frag_source(
            &CString::new(include_str!("../image.frag")).unwrap(),
        ).unwrap();
        let vert_shader = gl_render::Shader::from_vert_source(
            &CString::new(include_str!("../image.vert")).unwrap(),
        ).unwrap();
        image_program = gl_render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    
    }

    let grid_renderer = gl_render::GridRenderer::new(program).unwrap();
    let image_renderer = gl_render::ImageRenderer::new(image_program).unwrap();
    let mut texture = gl_render::Texture::new();

    let mut arr: Vec<u8> = Vec::new();
    for i in 0..300*300 {        
        arr.insert(i, if (rand::random::<u8>() % 3) == 0 { 0 } else {3});
    }
    arr[4] = 0;
    let mut map = Map {
        width: 300,
        stride: 300,
        height: 300,
        values: arr,
    };
    *map.value_mut((0,1)) = 0u8;
    texture.load_array(&map);
    let mut tank_texture = gl_render::Texture::new();
    let mut turret_texture = gl_render::Texture::new();
    let mut tank_image = stb_image::image::load("assets/Hull_01.png");
    tank_texture.load_stb_image(&mut tank_image, false);
    let mut turret_image = stb_image::image::load("assets/Gun_01.png");
    turret_texture.load_stb_image(&mut turret_image, false);
    let mut tank = _Tank {
        position: Vec2{x:0.,y:0.},
        angle: 0.,
        turret_angle: 0.,
        base_texture:tank_texture,
        turret_texture:turret_texture,
    };
    let mut time = std::time::Instant::now();
    let _path = path_find(&map, Vec2{x:10,y:10}, Vec2{x:map.width - 10, y:map.height - 10});
    
    'main: loop {
        let new_time = std::time::Instant::now();
        println!("frame {}", new_time.duration_since(time).as_millis());
        time = new_time;
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
        tank.position.y +=0.01;
        tank._render(&image_renderer,screen_resolution);
        window.gl_swap_window();
        // for pos in &path {
            // *map.value_mut((pos.x,pos.y)) = 2;
        // }
        texture.load_array(&map);
        // for pos in &path {
            // *map.value_mut((pos.x,pos.y)) = 3;
        // }
        
        for y in 0..map.height { 
            for x in 0..map.width {
                *map.value_mut((x,y)) =  if (rand::random::<u8>()) > 180 { 0 } else {3};
            }
        }
    
        // println!("{:?}",path);
    }
    println!("Hello, world!");
}
