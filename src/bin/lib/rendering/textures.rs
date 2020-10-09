use super::gl_render::Texture as Texture;
use super::sprite::mipmapped_texture_from_path;
use std::rc::Rc;
pub struct Textures {
    pub tank_texture:Rc<Texture>,
    pub turret_texture:Rc<Texture>,
    pub enemy_tank_texture:Rc<Texture>,
    pub enemy_turret_texture:Rc<Texture>,
    pub bullet_texture:Rc<Texture>,
}

pub fn load_textures() -> Textures {
    let tank_texture = mipmapped_texture_from_path("assets/Hull_A_01.png");
    let enemy_tank_texture = mipmapped_texture_from_path("assets/Hull_B_01.png");
    let turret_texture = mipmapped_texture_from_path("assets/Gun_A_01.png");
    let enemy_turret_texture = mipmapped_texture_from_path("assets/Gun_B_01.png");
    let bullet_texture = mipmapped_texture_from_path("assets/Light_Shell.png");
    Textures {
        tank_texture:Rc::new(tank_texture),
        turret_texture:Rc::new(turret_texture),
        enemy_tank_texture:Rc::new(enemy_tank_texture),
        enemy_turret_texture:Rc::new(enemy_turret_texture),
        bullet_texture:Rc::new(bullet_texture),
    }
}