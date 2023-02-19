use glam::{Vec2, Vec3};
use serde_json::Value;

use self::{disk::Disk, shpere::Shpere};
use super::{
    ray::Ray,
    tool::{bound::Bound3, interaction::Interaction},
};
use crate::lib::primitive::Primitive;

pub mod disk;
pub mod shpere;
pub mod triangle;
#[derive(Debug, Clone, Copy)]
#[allow(unused)]

pub enum Shape {
    Shpere(shpere::Shpere),
    Disk(Disk),
}

impl Shape {
    pub fn build_json(json: &serde_json::Map<String, Value>) -> Self {
        let t = json["mode"].as_str().map(|mode| {
            if mode.contains("shpere") {
                Shape::Shpere(Shpere::build(json))
            } else {
                unimplemented!()
            }
        });
        t.unwrap()
    }
}
impl Primitive for Shape {
    fn world_bound(&self) -> Bound3 {
        match self {
            Shape::Shpere(shpere) => shpere.world_bound(),
            Self::Disk(disk) => disk.world_bound(),
        }
    }

    fn interacect(&self, ray: &Ray) -> Option<Interaction> {
        match self {
            Shape::Shpere(shpere) => shpere.interacect(ray),
            Self::Disk(disk)=>disk.interacect(ray),
        }
    }
    fn interacect_bound(&self, ray: &Ray) -> bool {
        self.world_bound().intesect(ray)
    }
    fn get_area(&self)->f32{
        match self {
            Self::Disk(disk)=>disk.get_area(),
            Self::Shpere(sphere)=>sphere.get_area(),
        }
    }
    fn sample_point(&self, sample_point: Vec2)->Vec3{
        match self {
            Self::Disk(disk)=>disk.sample_point(sample_point),
            Self::Shpere(shpere)=>shpere.sample_point(sample_point)
        }
       
    }
}
impl rtbvh::Primitive for Shape {
    fn aabb(&self) -> rtbvh::Aabb {
        let bound = self.world_bound();
        rtbvh::Aabb {
            min: bound.min,
            extra1: Default::default(),
            max: bound.max,
            extra2: Default::default(),
        }
    }
    fn center(&self) -> glam::Vec3 {
        self.world_bound().center()
    }
}
