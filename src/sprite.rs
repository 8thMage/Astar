use crate::gl_render as gl_render;
use gl_render::Texture as Texture;
pub fn mipmapped_texture_from_path(path:&str)->Texture {
    let mut texture = gl_render::Texture::new().set_min_filter(gl::LINEAR_MIPMAP_LINEAR);
    let mut tank_image = stb_image::image::load(path);
    texture.load_stb_image(&mut tank_image, false);
    texture
}