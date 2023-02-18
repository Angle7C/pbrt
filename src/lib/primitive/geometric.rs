use crate::lib::{light::Light, material::Material, shape::Shape};

use super::Primitive;
#[allow(unused)]
#[derive(Debug)]
pub struct GeometricePrimitive<'a> {
    shape: &'a Shape,
    material: Option<&'a Material>,
    area_light: Option<&'a Light>,
}
#[allow(unused)]
impl<'a> GeometricePrimitive<'a> {
    pub fn new(
        shape: &'a Shape,
        material:Option<&'a Material>,
        area_light:Option<&'a Light>,
    ) -> Self {
        Self {
            shape,
            material,
            area_light,
        }
    }
}
impl<'a> Primitive for GeometricePrimitive<'a> {
    fn world_bound(&self) -> crate::lib::tool::bound::Bound3 {
        self.shape.world_bound()
    }

    fn interacect(
        &self,
        ray: &crate::lib::ray::Ray,
    ) -> Option<crate::lib::tool::interaction::Interaction> {
        self.shape.interacect(ray)
    }

    fn interacect_bound(&self, ray: &crate::lib::ray::Ray) -> bool {
        self.world_bound().intesect(ray)
    }
    fn get_area(&self)->f32 {
        self.shape.get_area()
    }
    fn sample_point(&self, sample_point: glam::Vec2)->glam::Vec3 {
        self.shape.sample_point(sample_point)
    }
 
}
