// use a_star::Heap;
use super::rendering::gl_render;
extern crate gl;
extern crate sdl2;
use super::map::Map;
use super::math::{matrix::*, vector::Vec2};
// use super::path_finding::path_find;
use crate::lib::entities::tank::Tank;
use crate::lib::entities::bullets::{Bullet, update_render_and_cull_bullets};
use crate::lib::rendering::camera;
use super::rendering::textures::load_textures;
use sdl2::keyboard::Scancode;
extern crate stb_image;
fn tank_handle_inputs(
    keyboard_state: sdl2::keyboard::KeyboardState,
    shoot: bool,
    tank: &mut Tank,
    bullets: &mut Vec<Bullet>,
) {
    let mut velocity = Vec2 { x: 0., y: 0. };
    if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
        velocity += tank.facing * 0.005;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S) {
        velocity -= tank.facing * 0.005;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D) {
        velocity = RotateMat::rotate_mat(-0.06) * velocity;
    }
    if keyboard_state.is_scancode_pressed(Scancode::A) {
        velocity = RotateMat::rotate_mat(0.06) * velocity;
    }
    tank.position += velocity;
    if velocity.norm() > 1e-6 {
        let dot = Vec2::dot(velocity, tank.facing);
        tank.facing = velocity * (1. / velocity.norm()) * dot.signum();
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
        bullets.push(Bullet::new(bullet_pos, bullet_facing));
    }
}

#[derive(Debug, Default)] // Derive is cool, I have no idea how it works!
struct Events {
    quit: bool,
    window_change: bool,
    space_pressed: bool,
    wheel: i32,
}

fn poll_event(event_pump: &mut sdl2::EventPump) -> Events {
    let mut events = Events::default();
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => {
                events.quit = true;
            }
            sdl2::event::Event::Window { .. } => {
                events.window_change = true;
            }
            sdl2::event::Event::KeyDown {
                scancode, repeat, ..
            } => {
                if let Some(scancode) = scancode {
                    if scancode == Scancode::Space {
                        events.space_pressed = !repeat;
                    }
                }
            }
            sdl2::event::Event::MouseWheel {
                y, ..
            } => {
                events.wheel += y;
            }

            _ => {}
        }
    }
    events
}

pub fn game(window: &sdl2::video::Window, event_pump: &mut sdl2::EventPump) {
    let grid_renderer = gl_render::GridRenderer::new().unwrap();
    let image_renderer = gl_render::ImageRenderer::new().unwrap();
    let mut texture = gl_render::Texture::new().set_min_filter(gl::NEAREST).set_mag_filter(gl::NEAREST);

    let textures = load_textures();
    let mut tank = Tank::new(
        Vec2 { x: 0., y: 0. },
        Vec2 {
            x: 0.0_f32,
            y: 1.0_f32,
        },
        textures.tank_texture.clone(),
        textures.turret_texture.clone(),
    );
    let enemy_tank = Tank::new(
        Vec2 { x: 0.5, y: 0. },
        Vec2 {
            x: 0.0_f32,
            y: 1.0_f32,
        },
        textures.enemy_tank_texture.clone(),
        textures.enemy_turret_texture.clone(),
    );
    let mut time = std::time::Instant::now();
    let mut bullets: Vec<Bullet> = vec![];
    let screen_resolution = window.drawable_size();
    let aspect_ratio = screen_resolution.1 as f32 / screen_resolution.0 as f32;
    let mut camera = camera::Camera {
        center:Vec2{x:-0., y:-0.}, 
        dimensions:Vec2{x: 4., y:4. * aspect_ratio},
        aspect_ratio:aspect_ratio,
    };
    let map_height = 16;
    let map_width = 16;
   
    let values = vec![0u8; map_height * map_width];
    let map = Map {
        height:map_height as i32,
        width:map_width as i32,
        stride:map_width as i32, 
        values
    };
    texture._load_array(&map);
    'main: loop {
        let new_time = std::time::Instant::now();
        println!("frame {}", new_time.duration_since(time).as_millis());
        time = new_time;

        let events = poll_event(event_pump);
        if events.quit {
            return;
        }
        if events.window_change {
            camera.dimensions.y /= camera.aspect_ratio;
            gl_render::update_window(window.drawable_size());
            let screen_resolution = window.drawable_size();
            let aspect_ratio = screen_resolution.1 as f32 / screen_resolution.0 as f32;          
            camera.dimensions.y *= aspect_ratio;
            camera.aspect_ratio = aspect_ratio;
        }
        camera.dimensions *= (-0.1 * events.wheel as f32).exp();
        let screen_resolution = window.drawable_size();
        grid_renderer.render(screen_resolution, &texture);
        tank_handle_inputs(
            event_pump.keyboard_state(),
            events.space_pressed,
            &mut tank,
            &mut bullets,
        );
        tank.update(events.space_pressed);

        tank.render(&image_renderer, &camera);
        enemy_tank.render(&image_renderer, &camera);
        update_render_and_cull_bullets(&mut bullets, &camera, &image_renderer, &textures.bullet_texture);

        window.gl_swap_window();
    }
}
