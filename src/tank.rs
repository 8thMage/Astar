use crate::math::{vector::Vec2, matrix::*};
use crate::gl_render::*;

pub struct Tank {
    pub position:Vec2<f32>,
    pub facing:Vec2<f32>,
    pub turret_facing_relative_to_tank:Vec2<f32>,
    pub turret_texture:Texture,
    pub base_texture:Texture,
    pivot:Vec2<f32>,
    mover:Vec2<f32>,
    pivot_velocity:Vec2<f32>
}

impl Tank {
    pub fn new(position:Vec2<f32>, facing:Vec2<f32>, base_texture:Texture, turret_texture:Texture)->Self {
        Tank {
            position,
            facing,
            turret_facing_relative_to_tank: Vec2{x: 1.0_f32, y:0.0_f32},
            base_texture,
            turret_texture,
            pivot:Vec2{x:0., y:0.3},
            mover:Vec2{x:0., y:0.},
            pivot_velocity:Vec2{x:0., y:0.0},
        }
    }
    pub fn update(&mut self, shoot:bool) {
        if shoot {
            self.pivot_velocity -= Vec2{x:0., y:0.2};
        }
        self.pivot_velocity -= self.mover * 0.2 + self.pivot_velocity*0.5;
        self.mover += self.pivot_velocity;
    }

    fn base_matrix(&self)->Mat3x2 {
        let trans = TranslationMat::translate_mat(self.position).scale(0.1).rotate(-std::f32::consts::FRAC_PI_2).rotate(self.facing);
        trans
    }

    fn turret_matrix_relative_to_base_matrix(&self)->Mat3x2 {
        let trans = TranslationMat::translate_mat(-self.pivot).rotate(self.turret_facing_relative_to_tank).translate(self.pivot + self.mover);
        trans
    }

    pub fn render(&self, image_renderer:&ImageRenderer, screen_resolution:(u32, u32)) {
        // let trans = transform((0.2 ,0.2 * self.base_texture.height as f32 / self.base_texture.width as f32),
        // (self.position.x, self.position.y), screen_resolution);
        // 
        let base_trans = self.base_matrix();
        image_renderer.render(&self.base_texture, screen_resolution, &base_trans);
        let turret_matrix_relative_to_base_matrix = self.turret_matrix_relative_to_base_matrix();
        let turret_trans = base_trans * turret_matrix_relative_to_base_matrix;
        // let trans2 = transform((0.2 * 0.4, 0.2 * 0.4 * self.turret_texture.height as f32 / self.turret_texture.width as f32),
        // (self.position.x,self.position.y + 0.06), screen_resolution);
        image_renderer.render(&self.turret_texture, screen_resolution, &turret_trans );
    }
    pub fn turret_facing(&self)->Vec2<f32> {
        RotateMat::rotate_mat(self.facing) * self.turret_facing_relative_to_tank
    }
    pub fn turret_position(&self)->Vec2<f32> {
        self.base_matrix() * self.turret_matrix_relative_to_base_matrix() * Vec2{x:0_f32, y:0.5_f32}
    }
}