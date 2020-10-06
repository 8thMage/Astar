use super::math::vector::Vec2;
use super::gl_render::{ImageRenderer, Texture};
use super::math::matrix::*;
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
pub fn update_render_and_cull_bullets(bullets: &mut Vec<Bullet>, aspect_ratio:f32, 
    image_renderer:&ImageRenderer, texture:&Texture) {
    *bullets = bullets
        .iter_mut()
        .map(|bullet| Bullet{position:bullet.position + bullet.velocity, velocity:bullet.velocity})
        .filter(|bullet| {
            let res = bullet.position.x < 1.
                && bullet.position.x > -1.
                && bullet.position.y < aspect_ratio
                && bullet.position.y > -aspect_ratio;
            res
        })
        .map(|bullet| {
            image_renderer.render(
                &texture,
                aspect_ratio,
                &TranslationMat::translate_mat(bullet.position)
                    .rotate(bullet.velocity.normalize())
                    .rotate(-std::f32::consts::FRAC_PI_2)
                    .scale(0.15),
            );
            bullet
        })
        .collect();
}
