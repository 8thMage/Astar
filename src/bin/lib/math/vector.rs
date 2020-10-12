use std::ops::{Add, AddAssign, SubAssign, Sub, Mul, MulAssign, Neg};

pub trait Arithmetic where Self: Copy + Add<Output=Self> + AddAssign + SubAssign + Sub<Output=Self> + Mul<Output=Self> + MulAssign + Neg<Output =Self> + Into<f64>  + std::cmp::PartialEq{}

impl<T> Arithmetic for T where T:Copy+ Add<Output=Self> + AddAssign + SubAssign + Sub<Output=Self> + Mul<Output=Self> + MulAssign + Neg<Output =Self> + Into<f64> + std::cmp::PartialEq{}

#[derive(Copy, Clone, Hash, Eq, Debug, Default)]
pub struct Vec2<T:Arithmetic> {
    pub x : T, 
    pub y : T,
}

impl<T:Arithmetic> Vec2<T> {
    pub fn dot(a: Vec2<T>, b:Vec2<T>) -> T {
        let res = a.x * b.x + a.y * b.y;
        res
    }
    
    pub fn hadamard(a: Vec2<T>, b:Vec2<T>) -> Vec2<T> {
        let res = Vec2{x: a.x * b.x, y: a.y * b.y};
        res
    }

    pub fn perp(self)->Vec2<T> {
        let res = Vec2{x:self.y, y:-self.x};
        res
    }

    pub fn norm2(self) ->T {
        let res = self.x * self.x + self.y * self.y;
        res
    }
    
    pub fn norm(self) ->f32 {
        let norm2:f64 = self.norm2().into();
        let res : f64 = norm2.sqrt();
        res as f32
    }

    pub fn normalize(self) -> Vec2<f32> {
        let norm2:f64 = self.norm2().into();
        let rnorm : f64 = norm2.sqrt().recip();
        Vec2 {
            x:(self.x.into() * rnorm) as f32, 
            y:(self.y.into() * rnorm) as f32
        }
    }
    
}

impl Vec2<i32> {
    pub fn to_f32(&self) -> Vec2<f32> {
        Vec2{
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Vec2<f32> {
    pub fn floor(&self) -> Vec2<i32> {
        Vec2{
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
        }
    }
}

impl<T> Add for Vec2<T> 
where T: Arithmetic 
    {
    type Output = Vec2<T>;
    fn add(self, b: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl<T:Arithmetic> AddAssign for Vec2<T> {
    fn add_assign(&mut self, b:Vec2<T>) {
        self.x += b.x;
        self.y += b.y;
    }
}

impl<T> Add<T> for Vec2<T> 
where T: Arithmetic 
    {
    type Output = Vec2<T>;
    fn add(self, b: T) -> Vec2<T> {
        Vec2 {
            x: self.x + b,
            y: self.y + b,
        }
    }
}

impl<T:Arithmetic> Sub for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, b: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }
}

impl<T:Arithmetic> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, b:Vec2<T>) {
        self.x -= b.x;
        self.y -= b.y;
    }
}

impl<T:Arithmetic> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, b:T) -> Vec2<T> {
        Vec2 {
            x: self.x * b,
            y: self.y * b,
        }
    }
}

impl<T:Arithmetic> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, b:T) {
        self.x *= b;
        self.y *= b;
    }
}

impl<T:Arithmetic> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) ->Vec2<T> {
        Vec2 {x:-self.x, y: -self.y}
    }
}

impl<T:Arithmetic> std::cmp::PartialEq for Vec2<T> {
    fn eq(&self, b: &Vec2<T>) ->bool {
        let res = self.x==b.x &&self.y == b.y;
        res
    }
}


// impl<T:Arithmetic> Div<f32> for Vec2<T> {
//     type Output = Vec2<T>;
//     fn div(self, b:f32) ->Vec2<T> {
//         Vec2 {
//             x: self.x / b,
//             y: self.y / b,
//         }
//     }
// }

// impl<T:Arithmetic> Div<f32> for Vec2<T> {
//     type Output = Vec2<T>;
//     fn div(self, b:f32) ->Vec2<T> {
//         Vec2 {
//             x: self.x / b,
//             y: self.y / b,
//         }
//     }
// }

pub fn hadamard_int(a: Vec2<f32>, b:Vec2<i32>) -> Vec2<f32> {
    let res = Vec2{x: a.x * b.x as f32, y: a.y * b.y as f32 };
    res
}
