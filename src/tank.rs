use crate::math::{vector::Vec2, matrix::*};
use crate::gl_render::*;

pub struct _Tank {
    pub position:Vec2<f32>,
    pub facing:Vec2<f32>,
    pub turret_facing_relative_to_tank:Vec2<f32>,
    pub turret_texture:Texture,
    pub base_texture:Texture
}

fn scale_matrix_from_vector_u32(vec:(u32, u32)) -> Mat3x2 {
    let scale = Mat3x2::identity().scale((1., vec.0 as f32/vec.1 as f32));
    scale
}

fn scale_matrix_from_vector_i32(vec:(i32, i32)) -> Mat3x2 {
    let scale = Mat3x2::identity().scale((1., vec.0 as f32 / vec.1 as f32));
    scale
}

impl _Tank {
    pub fn _render(&self, image_renderer:&ImageRenderer, screen_resolution:(u32, u32)) {
        // let trans = transform((0.2 ,0.2 * self.base_texture.height as f32 / self.base_texture.width as f32),
        // (self.position.x, self.position.y), screen_resolution);
        // 
        let res_scale = scale_matrix_from_vector_u32(screen_resolution);
        let tex_scale = scale_matrix_from_vector_i32((self.base_texture.height, self.base_texture.width));
        let trans = res_scale.translate(self.position).scale(0.1).rotate(-std::f32::consts::FRAC_PI_2).rotate(self.facing);

        image_renderer.render(&self.base_texture, &(&trans * &tex_scale));
        let pivot = Vec2{x:0_f32, y:0.3_f32};
        let trans2 = trans.translate(-pivot).rotate(self.turret_facing_relative_to_tank).translate(pivot);
        let tex_scale = scale_matrix_from_vector_i32((self.turret_texture.height, self.turret_texture.width));
        // let trans2 = transform((0.2 * 0.4, 0.2 * 0.4 * self.turret_texture.height as f32 / self.turret_texture.width as f32),
        // (self.position.x,self.position.y + 0.06), screen_resolution);
        image_renderer.render(&self.turret_texture, &(&trans2 * &tex_scale) );
    }
    pub fn turret_facing(&self)->Vec2<f32> {
        RotateMat::rotate_mat(self.facing) * self.turret_facing_relative_to_tank
    }
}