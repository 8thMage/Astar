use crate::vector::Vec2;
use crate::gl_render::*;

pub struct _Tank {
    pub position:Vec2<f32>,
    pub angle:f32,
    pub turret_angle:f32,
    pub turret_texture:Texture,
    pub base_texture:Texture
}

impl _Tank {
    pub fn _render(&self, image_renderer:&ImageRenderer, screen_resolution:(u32, u32)) {
        image_renderer.render(screen_resolution, &self.base_texture, (0.2 ,0.2 * self.base_texture.height as f32 / self.base_texture.width as f32), (self.position.x, self.position.y));
        image_renderer.render(screen_resolution, &self.turret_texture, (0.2 * 0.4, 0.2 * 0.4 * self.turret_texture.height as f32 / self.turret_texture.width as f32), (self.position.x,self.position.y + 0.06));
    }
}