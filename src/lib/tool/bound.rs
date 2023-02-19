

use glam::{Vec2, Vec3};

use crate::lib::ray::Ray;

use super::{VEC2_MAX, VEC2_MIN};
#[derive(Clone,Copy,Debug)]
pub struct Bound3 {
    pub min: Vec3,
    pub max: Vec3,
}
impl Default for Bound3 {
    fn default() -> Self {
        Self {
            min: Vec3::NAN,
            max: Vec3::NAN,
        }
    }
}
#[allow(unused)]
impl Bound3 {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        let a = min.min(max);
        let b = min.max(max);
        Self { min: a, max: b }
    }
    pub fn merage(&self, other: &Self) -> Self {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self {  min,  max }
    }
    pub fn grow(&self,point:Vec3)->Self{
        let min=self.min.min(point);
        let max=self.max.max(point);
        Self { min, max }
    }
    pub fn center(&self)->Vec3{
        (self.min+self.max)*0.5
    }
    pub fn area(&self)->f32{
        let det = (self.max-self.min);
        (det.x*det.y+det.x*det.z+det.y*det.z)*2.0
    }
    pub fn is_empty(&self)->bool{
        self.min.x>self.max.x||
        self.min.y>self.max.y||
        self.min.z>self.max.z

    }
    pub fn intesect(&self,ray:&Ray)->bool{
        let inv=ray.dir.recip();
        let t1=(self.min-ray.origin)*inv;
        let t2=(self.max-ray.origin)*inv;
        let t_min=t1.min(t2);
        let t_max=t1.max(t2);
        let t1=t_min.max_element();
        let t2=t_max.min_element();
        let t= t1<t2 &&t2>=0.0;
        t2<ray.t_max&&t
    }
}

pub struct Bound2 {
    pub min: Vec2,
    pub max: Vec2,
}
impl Default for Bound2 {
    fn default() -> Self {
        Self {
            min: VEC2_MAX,
            max: VEC2_MIN,
        }
    }
}
#[allow(unused)]
impl Bound2 {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        let a = min.min(max);
        let b = min.max(max);
        Self { min: a, max: b }
    }
    pub fn merage(&self, other: Self) -> Self {
        let min = self.min.min(other.min).min(other.max);
        let max = self.max.max(other.min).max(other.max);
        Self { min, max }
    }
}
