// use a_star::Heap;
use a_star::gl_render;
use a_star::sprite::mipmapped_texture_from_path;
use std::ffi::CString;
extern crate gl;
extern crate sdl2;
use a_star::map::Map;
use a_star::math::{matrix::*, vector::Vec2};
use a_star::path_finding::path_find;
use a_star::tank::Tank;
use sdl2::keyboard::Scancode;
extern crate stb_image;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(3, 3);
    let window = video_subsystem
        .window("astar", 1000, 500)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _swapped_interval = video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
    if let Err(string) = _swapped_interval {
        println!("{}", string);
    }
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
    let frag_shader =
        gl_render::Shader::from_frag_source(&CString::new(include_str!("../map.frag")).unwrap())
            .unwrap();
    let vert_shader =
        gl_render::Shader::from_vert_source(&CString::new(include_str!("../map.vert")).unwrap())
            .unwrap();
    let program = gl_render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    let image_program;
    {
        let frag_shader = gl_render::Shader::from_frag_source(
            &CString::new(include_str!("../image.frag")).unwrap(),
        )
        .unwrap();
        let vert_shader = gl_render::Shader::from_vert_source(
            &CString::new(include_str!("../image.vert")).unwrap(),
        )
        .unwrap();
        image_program = gl_render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    }

    let grid_renderer = gl_render::GridRenderer::new(program).unwrap();
    let image_renderer = gl_render::ImageRenderer::new(image_program).unwrap();
    let mut texture = gl_render::Texture::new().set_min_filter(gl::NEAREST);

    let mut arr: Vec<u8> = Vec::new();
    let map_height = 300;
    let map_width = 300;
    for y in 0..map_height {
        for x in 0..map_width {
            arr.push(if x % 2 == 1 && y % 2 == 1 { 0 } else { 3 });
        }
    }
    // arr[4] = 0;
    let mut map = Map {
        width: map_width,
        stride: map_width,
        height: map_height,
        values: arr,
    };
    let prev_time = std::time::Instant::now();
    let p = path_find(
        &map,
        Vec2 { x: 0, y: 0 },
        Vec2 {
            x: map_width - 2,
            y: map_height - 2,
        },
    );
    let new_time = std::time::Instant::now();
    let dt = new_time.duration_since(prev_time).as_micros();
    println!("frame0 {}", dt);
    println!("frame0 {}", dt);
    println!("plen {}", p.len());
    *map.value_mut((0, 1)) = 0u8;
    texture.load_array(&map);
    let tank_texture = mipmapped_texture_from_path("assets/Hull_A_01.png");
    let turret_texture = mipmapped_texture_from_path("assets/Gun_A_01.png");
    let mut tank = Tank::new(
        Vec2 { x: 0., y: 0. },
        Vec2 {
            x: 0.0_f32,
            y: 1.0_f32,
        },
        tank_texture,
        turret_texture,
    );
    let enemy_tank_texture = mipmapped_texture_from_path("assets/Hull_B_01.png");
    let enemy_turret_texture = mipmapped_texture_from_path("assets/Gun_B_01.png");
    let enemy_tank = Tank::new(
        Vec2 { x: 0.5, y: 0. },
        Vec2 {
            x: 0.0_f32,
            y: 1.0_f32,
        },
        enemy_tank_texture,
        enemy_turret_texture,
    );
    let mut time = std::time::Instant::now();
    let _path = path_find(
        &map,
        Vec2 { x: 10, y: 10 },
        Vec2 {
            x: map.width - 10,
            y: map.height - 10,
        },
    );
    let bullet_texture = mipmapped_texture_from_path("assets/Light_Shell.png");
    let mut bullets: Vec<(Vec2<f32>, Vec2<f32>)> = vec![];
    let mut already_shot = false;
    'main: loop {
        let new_time = std::time::Instant::now();
        println!("frame {}", new_time.duration_since(time).as_millis());
        time = new_time;
        let mut shoot = false;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                // sdl2::event::Event::MouseMotion {
                //     x, y, xrel, yrel, ..
                // } => {
                // }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    let rounded_x =
                        ((x * map.width) as f32 / window.drawable_size().0 as f32).floor() as i32;
                    let rounded_y = map.height
                        - ((y * map.height) as f32 / window.drawable_size().1 as f32).ceil() as i32;
                    let texel = map.value_mut((rounded_x, rounded_y));
                    *texel = 3u8 - *texel;
                }
                // ...
                sdl2::event::Event::KeyDown { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        if scancode == Scancode::Space {
                            if !already_shot {
                                shoot = true;
                            }
                        }
                    }
                }

                sdl2::event::Event::KeyUp { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        if scancode == Scancode::Space {
                            already_shot = false;
                        }
                    }
                }

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
        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            tank.position += tank.facing * 0.005;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            tank.position -= tank.facing * 0.005;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            tank.facing = RotateMat::rotate_mat(-0.06) * tank.facing;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            tank.facing = RotateMat::rotate_mat(0.06) * tank.facing;
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            tank.turret_facing_relative_to_tank =
                RotateMat::rotate_mat(-0.05) * tank.turret_facing_relative_to_tank;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            tank.turret_facing_relative_to_tank =
                RotateMat::rotate_mat(0.05) * tank.turret_facing_relative_to_tank;
        }
        if shoot {
            let bullet_pos = tank.turret_position();
            let bullet_facing = tank.turret_facing();
            bullets.push((bullet_pos, bullet_facing));
            already_shot = true;
        }
        let aspect_ratio = screen_resolution.1 as f32 / screen_resolution.0 as f32;
        if tank.position.y > aspect_ratio {
            tank.position.y -= 2. * aspect_ratio;
        }
        if tank.position.y < -aspect_ratio {
            tank.position.y += 2. * aspect_ratio;
        }
        if tank.position.x > 1. {
            tank.position.x -= 2.;
        }
        if tank.position.x < -1. {
            tank.position.x += 2.;
        }
        tank.update(shoot);
        for bullet in &mut bullets {
            bullet.0 = bullet.0 + bullet.1 * 0.01;
        }
        bullets.retain(|bullet| {
            let res = bullet.0.x < 1.
                && bullet.0.x > -1.
                && bullet.0.y < aspect_ratio
                && bullet.0.y > -aspect_ratio;
            res
        });
        for bullet in &bullets {
            image_renderer.render(
                &bullet_texture,
                screen_resolution,
                &TranslationMat::translate_mat(bullet.0)
                    .rotate(bullet.1)
                    .rotate(-std::f32::consts::FRAC_PI_2)
                    .scale(0.15),
            );
        }
        tank.render(&image_renderer, screen_resolution);
        enemy_tank.render(&image_renderer, screen_resolution);
        window.gl_swap_window();
        texture.load_array(&map);
    }
    println!("Hello, world!");
}
