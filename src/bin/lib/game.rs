// use a_star::Heap;
use super::rendering::gl_render;
extern crate gl;
extern crate sdl2;
use super::map::Map;
use super::math::{matrix::*, vector, vector::Vec2};
// use super::path_finding::path_find;
use super::rendering::textures::load_textures;
use crate::lib::entities::bullets::{update_render_and_cull_bullets, Bullet};
use crate::lib::entities::tank::Tank;
use crate::lib::rendering::camera;
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
    mouse_button_clicked: bool,
    mouse_button_position: Vec2<i32>,
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
            sdl2::event::Event::MouseWheel { y, .. } => {
                events.wheel += y;
            }
            sdl2::event::Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == sdl2::mouse::MouseButton::Right {
                    events.mouse_button_clicked = true;
                    events.mouse_button_position = Vec2 { x, y };
                }
            }

            _ => {}
        }
    }
    events
}

fn position_to_map(position: Vec2<f32>, map: &Map) -> Vec2<f32> {
    vector::hadamard_int(position + 1., map._dimensions()) * 0.5
}

fn map_to_position(map_position: Vec2<f32>, map: &Map) -> Vec2<f32> {
    Vec2::hadamard(
        map_position,
        Vec2 {
            x: 1.0f32 / map.width as f32,
            y: 1.0f32 / map.height as f32,
        },
    ) * 2.
        + (-1.)
}

pub fn game(window: &sdl2::video::Window, event_pump: &mut sdl2::EventPump) {
    let grid_renderer = gl_render::GridRenderer::new().unwrap();
    let image_renderer = gl_render::ImageRenderer::new().unwrap();
    let mut texture = gl_render::Texture::new()
        .set_min_filter(gl::NEAREST)
        .set_mag_filter(gl::NEAREST);

    let textures = load_textures();
    let enemy_tank = Tank::new(
        Vec2 { x: -0.5, y: 0. },
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
        center: Vec2 { x: -0., y: -0. },
        dimensions: Vec2 {
            x: 2.,
            y: 2. * aspect_ratio,
        },
        aspect_ratio: aspect_ratio,
    };
    let map_height = 16;
    let map_width = 16;
    let mut arr: Vec<u8> = Vec::new();
    for y in 0..map_height {
        for x in 0..map_width {
            arr.push(if x % 2 == 1 && y % 2 == 1 { 0 } else { 3 });
        }
    }
    let map = Map {
        height: map_height as i32,
        width: map_width as i32,
        stride: map_width as i32,
        values: arr,
    };
    let mut tank = Tank::new(
        map_to_position(Vec2 { x: 0.5, y: 4.5 }, &map),
        Vec2 {
            x: 1.0_f32,
            y: 0.0_f32,
        },
        textures.tank_texture.clone(),
        textures.turret_texture.clone(),
    );

    let mut p = super::path_finding::path_find(&map, Vec2 { x: 0, y: 4 }, Vec2 { x: 6, y: 10 });
    texture._load_array(&map);
    let mut dest = map_to_position(Vec2 { x: 6.2f32, y: 10.1f32 }, &map);
    let mut cooldown = 0.;
    'main: loop {
        let new_time = std::time::Instant::now();
        let dt = new_time.duration_since(time).as_millis() as f32;
        println!("frame {}", dt);
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
        grid_renderer.render(&camera, &texture);
        let map_pos = position_to_map(tank.position, &map);
        if events.mouse_button_clicked {
            let screen_pos = events.mouse_button_position;
            let screen_size = Vec2 {
                x: window.drawable_size().0 as f32,
                y: window.drawable_size().1 as f32,
            };
            let relative_screen_pos = Vec2 {
                x: 2. * screen_pos.x as f32 / screen_size.x - 1.,
                y: 1. - 2. * screen_pos.y as f32 / screen_size.y,
            };
            let transform = camera.inverse_transform();
            let camera_pos = transform * relative_screen_pos;
            let camera_pos_int = position_to_map(camera_pos, &map).floor(); 
            let tank_pos_int = position_to_map(tank.position, &map).floor();
            p = super::path_finding::path_find(&map, tank_pos_int, camera_pos_int);
            dest = camera_pos;
        }
        let mut shoot = events.space_pressed; 
        if p.len() != 1 && p.len() !=0 {
            if p[0].x != map_pos.x as i32 || p[0].y != map_pos.y as i32 {
                p.remove(0);
            }
        }
        let delta;
        if p.len() != 1 && p.len() !=0 {
            let next_pos = p[1].to_f32() + 0.5;
            let next_pos_in_world = map_to_position(next_pos, &map);
            delta = (next_pos_in_world - tank.position).normalize();
        } else {
            delta = dest - tank.position;
        }
        if delta.norm2() > 0.001 && (dest - tank.position).norm2() > 0.01 { 
            let delta_facing = Vec2 {
                x: Vec2::dot(delta, tank.facing),
                y: Vec2::dot(delta.perp(), tank.facing),
            };
            let angle = delta_facing.y.atan2(delta_facing.x);
            tank.facing_derivative = RotateMat::rotate_mat(angle * 0.05) * tank.facing_derivative ;
            tank.position += tank.facing * 0.003;
        }
        if p.len() < 3  {
            let delta = (dest - tank.position).conj();
            let delta_turret = RotateMat::rotate_mat(delta) * tank.turret_facing(); 
            let angle = - delta_turret.y.atan2(delta_turret.x);
            tank.turret_facing_relative_to_tank = RotateMat::rotate_mat(angle*0.3) * tank.turret_facing_relative_to_tank;
            if delta_turret.y < 0.001 && delta_turret.x > 0. {
                if cooldown < 0. {
                    shoot = true;
                    cooldown = 300.;
                }
            }
        }
        cooldown -= dt;
    
        tank_handle_inputs(
            event_pump.keyboard_state(),
            shoot,
            &mut tank,
            &mut bullets,
        );

        tank.update(shoot);

        tank.render(&image_renderer, &camera);
        enemy_tank.render(&image_renderer, &camera);
        update_render_and_cull_bullets(
            &mut bullets,
            &camera,
            &image_renderer,
            &textures.bullet_texture,
        );

        window.gl_swap_window();
    }
}
