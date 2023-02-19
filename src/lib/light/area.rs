use std::f32::consts::PI;

use glam::{Vec3, Vec2};
use serde_json::{Value, Map};

use crate::lib::{shape::Shape, tool::{interaction::Interaction, scene::Sence}, ray::Ray, primitive::Primitive};
#[derive(Debug)]

#[allow(unused)]

pub struct AreaLight {
    lemit: Vec3,
    shape: Shape,
    area: f32,
}
#[allow(unused)]
impl AreaLight {
    pub fn new(lemit: Vec3, shape: Shape, area: f32) -> Self {
        Self { lemit, shape, area:shape.get_area() }
    }
    pub fn power(&self) -> Vec3 {
        self.lemit*self.area*PI
    }
    pub fn l(&self,inter:&Interaction,w:Vec3)->Vec3{
        if inter.normal.dot(w)>0.0{self.lemit}else{Vec3::ZERO}
    }
  
    pub fn vis_li(&self, inter: &Interaction,sence:&Sence,sample_point: Vec2) -> bool {
        let dir = self.shape.sample_point(sample_point)-inter.p;
        let ray = Ray::new(inter.p, dir);
        match sence.intersect(&ray)  {
            Some(a) if ray.dir.dot(inter.normal)<0.0=>{
              a.p.distance(inter.p)<0.00001
            } 
            _=>false 
        } 
    }
    pub fn sample_li(&self, sample_point: &Vec3, normal: &Vec3) -> Vec3 {
        todo!()
    }
    pub fn build_json(json: &Map<String, Value>) -> Self {
        todo!()
    }
}
