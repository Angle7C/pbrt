use std::f32::consts::PI;

use glam::{ Vec3};
use serde_json::{Map, Value};

use crate::lib::{
    ray::Ray,
    tool::{
        interaction::Interaction, parse_vec3, scene::Sence
    },
};

use super::dist;
#[derive(Debug)]

pub struct PointLight {
    p_light: Vec3,
    i: Vec3,
}
impl Default for PointLight {
    fn default() -> Self {
        Self {
            p_light: Vec3::Z * 100.0,
            i: Vec3::ONE,
        }
    }
}
#[allow(unused)]
impl PointLight {
    pub fn new(pos: Vec3, i: Vec3) -> Self {
        Self {
            p_light: pos,
            i,
        }
    }
    pub fn power(&self) -> Vec3 {
        4.0 * PI * self.i
    }
    pub fn vis_li(&self, inter: &Interaction, sence: &Sence) -> bool {
        let dir = (inter.p-self.p_light);
        
        let ray = Ray::from_with_t(self.p_light, dir,0.00,dir.length()-0.001);
        matches!(sence.intersect(&ray),None)
    }
    pub fn sample_li(&self, sample_point: &Vec3, normal: &Vec3) -> Vec3 {
        let dist = dist(self.p_light, *sample_point);
        let noraml = normal.normalize();
        let wi = (*sample_point - self.p_light).normalize();
        // let cos_a=wi.dot(noraml).clamp(0.0,1.0);
        self.i * dist
    }
    pub fn build_json(json: &Map<String, Value>) -> Self {
        let pos = parse_vec3(json, "pos");
        let i = parse_vec3(json, "i");
        Self::new(pos, i)
    }
}
