

use self::uvmapping::UVMapping;

use super::tool::interaction::Interaction;
mod uvmapping;
#[allow(unused)]
pub enum TextureMapping2D {
    UV(UVMapping)
}
#[allow(unused)]
pub trait Texture<T> {
    fn evaluate(&self,inter:&Interaction)->T;    
}
pub enum TextureMapping3D {

}
