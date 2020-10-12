use super::vector::Vec2;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub struct Mat3x2 {
    // collumn order.
    pub arr: [[f32; 2]; 3],
}

impl Add for Mat3x2 {
    type Output = Mat3x2;
    fn add(self, b: Mat3x2) -> Mat3x2 {
        let mut new_arr = [[0_f32; 2]; 3];
        for x in 0..new_arr.len() {
            for y in 0..new_arr[x].len() {
                new_arr[x][y] = self.arr[x][y] + b.arr[x][y];
            }
        }
        Mat3x2 { arr: new_arr }
    }
}

impl AddAssign for Mat3x2 {
    fn add_assign(&mut self, b: Mat3x2) {
        for x in 0..self.arr.len() {
            for y in 0..self.arr[x].len() {
                self.arr[y][x] = self.arr[y][x] + b.arr[y][x];
            }
        }
    }
}

impl Sub for Mat3x2 {
    type Output = Mat3x2;
    fn sub(self, b: Mat3x2) -> Mat3x2 {
        let mut new_arr = [[0_f32; 2]; 3];
        for x in 0..new_arr.len() {
            for y in 0..new_arr[x].len() {
                new_arr[x][y] = self.arr[x][y] - b.arr[x][y];
            }
        }
        Mat3x2 { arr: new_arr }
    }
}

impl SubAssign for Mat3x2 {
    fn sub_assign(&mut self, b: Mat3x2) {
        for x in 0..self.arr.len() {
            for y in 0..self.arr[x].len() {
                self.arr[x][y] = self.arr[x][y] - b.arr[x][y];
            }
        }
    }
}

impl Mul for Mat3x2 {
    type Output = Mat3x2;
    fn mul(self, b: Mat3x2) -> Mat3x2 {
        let mut new_arr = [[0_f32; 2]; 3];
        for x in 0..new_arr.len() {
            for y in 0..new_arr[x].len() {
                let mut acc = 0_f32;
                for i in 0..new_arr[0].len() {
                    acc += self.arr[i][y] * b.arr[x][i];
                }
                if x == new_arr.len() - 1 {
                    acc += self.arr[x][y];
                }
                new_arr[x][y] = acc;
            }
        }
        Mat3x2 { arr: new_arr }
    }
}

impl Mul for &Mat3x2 {
    type Output = Mat3x2;
    fn mul(self, b: &Mat3x2) -> Mat3x2 {
        let mut new_arr = [[0_f32; 2]; 3];
        for x in 0..new_arr.len() {
            for y in 0..new_arr[x].len() {
                let mut acc = 0_f32;
                for i in 0..new_arr[0].len() {
                    acc += self.arr[i][y] * b.arr[x][i];
                }
                if x == new_arr.len() - 1 {
                    acc += self.arr[x][y];
                }
                new_arr[x][y] = acc;
            }
        }
        Mat3x2 { arr: new_arr }
    }
}

impl MulAssign<&Mat3x2> for Mat3x2 {
    fn mul_assign(&mut self, b: &Mat3x2) {
        let mut new_arr = [[0_f32; 2]; 3];
        for x in 0..new_arr.len() {
            for y in 0..new_arr[x].len() {
                let mut acc = 0_f32;
                for i in 0..new_arr[0].len() {
                    acc += self.arr[i][y] * b.arr[x][i];
                }
                if x == new_arr.len() - 1 {
                    acc += self.arr[x][y];
                }
                new_arr[x][y] = acc;
            }
        }
        self.arr = new_arr;
    }
}

impl Mul<Vec2<f32>> for Mat3x2 {
    type Output = Vec2<f32>;
    fn mul(self, b: Vec2<f32>) -> Vec2<f32> {
        let mut new_arr = [0_f32; 2];
        for y in 0..self.arr[0].len() {
            let mut acc = 0_f32;
            acc += self.arr[0][y] * b.x;
            acc += self.arr[1][y] * b.y;
            acc += self.arr[2][y];
            new_arr[y] = acc;
        }
        Vec2 {
            x: new_arr[0],
            y: new_arr[1],
        }
    }
}

pub trait ScaleMat {
    fn scale_mat(var: Self)->Mat3x2;
}

impl ScaleMat for f32 {
    fn scale_mat(var:f32) ->Mat3x2  {
        let scale_mat = [[var, 0.], [0., var], [0., 0.]];
        Mat3x2 { arr: scale_mat }
    }
}

impl ScaleMat for (f32,f32) {
    fn scale_mat(var:(f32,f32))->Mat3x2 {
        let scale_mat = [[var.0, 0.], [0., var.1], [0., 0.]];
        Mat3x2 { arr: scale_mat }
    }
}

pub trait TranslationMat {
    fn translate_mat(var: Self)->Mat3x2;
}

impl TranslationMat for (f32,f32) {
    fn translate_mat(var: (f32, f32)) -> Mat3x2 {
        let translate_mat = [[1., 0.], [0., 1.], [var.0, var.1]];
        Mat3x2 { arr: translate_mat }
    }
}

impl TranslationMat for Vec2<f32> {
    fn translate_mat(var: Vec2<f32>) -> Mat3x2 {
        let translate_mat = [[1., 0.], [0., 1.], [var.x, var.y]];
        Mat3x2 { arr: translate_mat }
    }
}
impl TranslationMat for Vec2<i32> {
    fn translate_mat(var:Vec2<i32>) -> Mat3x2 {
        let translate_mat = [[1., 0.], [0., 1.], [var.x as f32, var.y as f32]];
        Mat3x2 { arr: translate_mat }
    }
}

pub trait RotateMat {
    fn rotate_mat(var: Self)->Mat3x2 ;
}
impl RotateMat for f32 {
    fn rotate_mat(angle: f32) -> Mat3x2 {
        let rot = [
            [angle.cos(), angle.sin()],
            [- angle.sin(), angle.cos()],
            [0., 0.],
        ];
        Mat3x2 { arr: rot }
    }
}

impl RotateMat for Vec2<f32> {
    fn rotate_mat(facing: Vec2<f32>) -> Mat3x2 {
        let normalized_facing = facing * (1./facing.norm());
        let rot = [
            [normalized_facing.x, normalized_facing.y],
            [- normalized_facing.y, normalized_facing.x],
            [0., 0.],
        ];
        Mat3x2 { arr: rot }
    }
}

impl Mat3x2 {
    pub fn identity() -> Mat3x2 {
        let iden = [[1., 0.], [0., 1.], [0., 0.]];
        Mat3x2 { arr: iden }
    }
    pub fn rotate<T:RotateMat>(&self, var:T) ->Mat3x2 {
        self * &RotateMat::rotate_mat(var)
    }
    
    pub fn translate<T:TranslationMat>(&self, var:T) ->Mat3x2 {
        self * &TranslationMat::translate_mat(var)
    }

    pub fn scale<T:ScaleMat>(&self, var:T) ->Mat3x2 {
        self * &ScaleMat::scale_mat(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scale_vec_by_2() {
        let vec = Vec2{x:1., y:2.};
        let scale = Mat3x2::identity().scale(2.);
        let new_vec = scale * vec;
        assert_eq!(Vec2{x:2., y:4.}, new_vec);
    }
    #[test]
    fn rotate_vec_by_90degrees() {
        let vec = Vec2{x:1., y:2.};
        let rotate = Mat3x2::identity().rotate(std::f32::consts::PI * 0.5);
        let new_vec = rotate * vec;
        assert!((Vec2{x:-2., y:1.} - new_vec).norm() < 1e-5);
    }
    #[test]    
    fn mat_mul() {
        let rotate = Mat3x2::identity().rotate(std::f32::consts::PI * 0.5);
        let rotate2 = Mat3x2::identity().rotate(std::f32::consts::PI * 0.5);
        let rotate3 = Mat3x2::identity().rotate(std::f32::consts::PI);
        let new_mat = rotate*rotate2 - rotate3;
        let mut norm = 0_f32;
        for x in 0..new_mat.arr.len() {
            for y in 0..new_mat.arr[x].len() {
                norm += new_mat.arr[x][y].abs();
            }
        }
        assert!(norm < 1e-5);
    }

    #[test]    
    fn mat_mul2() {
        let rotate = Mat3x2::identity().rotate(std::f32::consts::PI * 0.2);
        let rotate2 = Mat3x2::identity().rotate(std::f32::consts::PI * 0.8);
        let rotate3 = Mat3x2::identity().rotate(std::f32::consts::PI);
        let new_mat = rotate*rotate2 - rotate3;
        let mut norm = 0_f32;
        for x in 0..new_mat.arr.len() {
            for y in 0..new_mat.arr[x].len() {
                norm += new_mat.arr[x][y].abs();
            }
        }
        assert!(norm < 1e-5);
    }

    #[test]    
    fn mat_mul3() {
        let rotate = Mat3x2::identity().rotate(std::f32::consts::PI * 0.5);
        let translate = Mat3x2::identity().translate((0., 1.));
        let expected = [
            [0., 1.],
            [-1., 0.],
            [-1., 0.],
        ];
        let result = Mat3x2 {
            arr:expected,
        };
        let new_mat = rotate*translate - result;
        let mut norm = 0_f32;
        for x in 0..new_mat.arr.len() {
            for y in 0..new_mat.arr[x].len() {
                norm += new_mat.arr[x][y].abs();
            }
        }
        assert!(norm < 1e-5);
    }

    #[test]    
    fn mat_mul4() {
        let rotate = Mat3x2::identity().rotate(std::f32::consts::PI * 0.5);
        let translate = Mat3x2::identity().translate((0., 1.));
        let expected = [
            [0., 1.],
            [-1., 0.],
            [0., 1.],
        ];
        let result = Mat3x2 {
            arr:expected,
        };
        let new_mat = translate*rotate - result;
        let mut norm = 0_f32;
        for x in 0..new_mat.arr.len() {
            for y in 0..new_mat.arr[x].len() {
                norm += new_mat.arr[x][y].abs();
            }
        }
        assert!(norm < 1e-5);
    }
}