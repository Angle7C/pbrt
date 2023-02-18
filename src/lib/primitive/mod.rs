
use std::fmt::Debug;

use glam::{Vec2, Vec3};

use self::aggreate::BVH;

use super::{tool::{bound::Bound3, interaction::Interaction}, ray::Ray, shape::{Shape}};
mod aggreate;
mod geometric;
pub trait Primitive:Debug {
    fn world_bound(&self)->Bound3;
    fn interacect(&self,ray:&Ray)->Option<Interaction>;
    fn get_area(&self)->f32;
    fn sample_point(&self, sample_point: Vec2)->Vec3;
    fn interacect_bound(&self,ray:&Ray)->bool{
        self.world_bound().intesect(ray)
    }
}
pub trait Aggregate:Sync {
    fn world_bound(&self) -> Bound3;
    fn interacect(&self, ray: &Ray) -> Option<Interaction>;
    fn interacect_bound(&self, ray: &Ray) -> bool{
        self.world_bound().intesect(ray)
    }
}
pub enum AggregateType{
    BVH,

}
struct AggregateDefault;
impl Aggregate for AggregateDefault{
    fn interacect(&self, _ray: &Ray) -> Option<Interaction> {
        panic!("该加速结构不存在加速功能")
        
    }
    fn world_bound(&self) -> Bound3 {
        panic!("该加速结构不存在加速功能")
    }
}
pub fn build_aggregate(types:AggregateType,builder:Vec<Shape>)-> Box<dyn Aggregate>{ 
       match types {
            AggregateType::BVH=>{
                Box::new( BVH::build(builder))
            }
        }

}
