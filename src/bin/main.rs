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
    for i in 0..20*20 {
        arr.insert(i, 3);
    }
    arr[4] = 0;
    let mut map = Map {
        width: 20,
        stride: 20,
        height: 20,
        values: arr,
    };
    *map.value_mut((0,1)) = 0u8;
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
        let path = path_find(&map, Vec2{x:0,y:0}, Vec2{x:2,y:2});
        for pos in &path {
            *map.value_mut((pos.x,pos.y)) = 2;
        }
        texture.load_array(&map);
        for pos in &path {
            *map.value_mut((pos.x,pos.y)) = 3;
        }
        println!("{:?}",path);
    }
    println!("Hello, world!");
}

struct Node {
    father: Option<std::rc::Rc<Node>>,
    position: Vec2<i32>,
    real_distance: i32,
}

fn heuristic(start_point: Vec2<i32>, end_point: Vec2<i32>) -> f32 {
    let res = (start_point - end_point).norm();
    res
}

// std::collections::BTreeSet

fn path_find(map: &Map, start_point: Vec2<i32>, end_point: Vec2<i32>) -> Vec<Vec2<i32>>{
    let mut heap: Heap<f32, Node> = Heap::new();
    let mut hash = std::collections::HashMap::new();
    let mut hashOpen = std::collections::HashMap::new();
    let start: Node = Node {
        position: start_point,
        father: None,
        real_distance: 0,
    };
    let value = heuristic(start_point, end_point);
    heap.push(value, start);
    hashOpen.insert(start_point, value);
    let neighbors_delta = vec!(Vec2{x:0,y:1},Vec2{x:1,y:0},Vec2{x:-1,y:0},Vec2{x:0,y:-1});
    let path = 
    {
        let mut result = None;
        'whileHeapNotEmpty : while let Some(popped) = heap.pop() {
            let node = popped.1;
            let position = node.position;
            let real_distance = node.real_distance;
            if let Some(&hash_pos) = hash.get(&position) {
                if (real_distance < hash_pos) {
                    continue;
                }
            }
            let boxed = std::rc::Rc::new(node);
            for &delta in &neighbors_delta {
                let new_position = position + delta;
                if new_position.x >= map.width || new_position.y >= map.height || new_position.x < 0 || new_position.y < 0 {
                    continue;
                } 
                if *map.value((new_position.x, new_position.y)) != 3 {
                    continue;
                }
                let new_real_distance = real_distance + 1;
                let h = heuristic(new_position, end_point);
                let value = new_real_distance as f32 + h;
                let new_node = Node{
                    position:new_position,
                    real_distance:new_real_distance,
                    father: Some((&boxed).clone()),
                };
                if new_position == end_point {
                    result = Some(new_node);
                    break 'whileHeapNotEmpty;
                }
                if let Some(old_dist) = hash.get(&new_position) {
                    if new_real_distance < *old_dist {
                        heap.push(value, new_node);
                        hashOpen.insert(new_position, value);
                    }
                } else if let Some(old_dist) = hashOpen.get(&new_position) {
                    if value < *old_dist {
                        heap.push(value, new_node);
                        hashOpen.insert(new_position, value);
                    }
                } else {
                    heap.push(value, new_node);
                    hashOpen.insert(new_position, value);
                }
            }
            
            if let Some(hash_pos) = hash.get_mut(&position) {
                *hash_pos = real_distance.max(*hash_pos);
            } else {
                hash.insert(position, real_distance);
            }
        }
        result
    };
    drop(heap);
    
    let mut result_vector = vec!();
    let mut iter_path = path;
    while let Some(p) = iter_path {
        result_vector.insert(0, p.position);
        if p.father.is_none() {
            break;
        } else {
            let option = std::rc::Rc::try_unwrap(p.father.unwrap());
            iter_path = option.ok();
        }
    }
    result_vector
}
