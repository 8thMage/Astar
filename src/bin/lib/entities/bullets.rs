use crate::lib::math::vector::Vec2;
use crate::lib::rendering::gl_render::{ImageRenderer, Texture};
use crate::lib::rendering::camera::*;
use crate::lib::math::matrix::*;
pub struct Bullet {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}
impl Bullet {
    pub fn new(position: Vec2<f32>, facing: Vec2<f32>) -> Bullet{
        Bullet {
            position,
            velocity: facing.normalize()*0.01,
        }
    }
}

pub fn update_render_and_cull_bullets(bullets: &mut Vec<Bullet>, camera:&Camera, 
    image_renderer:&ImageRenderer, texture:&Texture) {
    *bullets = bullets
        .iter_mut()
        .map(|bullet| Bullet{position:bullet.position + bullet.velocity, velocity:bullet.velocity})
        .filter(|bullet| {
            let rel = bullet.position - camera.center;

            let res = rel.x.abs() < camera.dimensions.x * 0.5 && 
                    rel.y.abs() < camera.dimensions.y * 0.5;
            res
        })
        .map(|bullet| {
            image_renderer.render(
                &texture,
                camera,
                &TranslationMat::translate_mat(bullet.position)
                    .rotate(bullet.velocity.normalize())
                    .rotate(-std::f32::consts::FRAC_PI_2)
                    .scale(0.15),
            );
            bullet
        })
        .collect();
}
