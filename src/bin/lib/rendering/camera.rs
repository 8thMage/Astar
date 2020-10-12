use crate::lib::math::{vector::*, matrix::*};

pub struct Camera {
    pub center:Vec2<f32>,
    pub dimensions:Vec2<f32>,
    pub aspect_ratio:f32,
}

impl Camera {
    pub fn transform(&self) -> Mat3x2{
        let camera_scale = (2. / self.dimensions.x, 2. / self.dimensions.y);
        let camera_transform = ScaleMat::scale_mat(camera_scale).translate(-self.center);
        camera_transform
    }
    
    pub fn inverse_transform(&self) -> Mat3x2{
        let camera_scale = (self.dimensions.x / 2. , self.dimensions.y /  2.);
        let camera_transform = ScaleMat::scale_mat(camera_scale).translate(self.center);
        camera_transform
    }
}