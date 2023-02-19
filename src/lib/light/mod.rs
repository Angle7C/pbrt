use glam::{Vec3, Vec2};
use serde_json::Value;

use self::{point::PointLight, area::AreaLight};

use super::tool::{interaction::Interaction, scene::Sence};

pub mod point;
pub mod area;
#[derive(Debug)]
#[allow(unused)]

pub enum Light {
    Point(PointLight),
    AreaLight(AreaLight),
}
impl Light {
    /// 光的到交点的 光的颜色随距离衰减，pdf。
    pub fn vis_li(&self, inter: &Interaction,sence:&Sence,sample_point: Vec2) -> bool {
        match self {
            Light::Point(point) => point.vis_li(inter,sence),
            Self::AreaLight(area) => area.vis_li(inter, sence, sample_point)
        }
    }
    pub fn sample_li(&self, sample_point: &Vec3, normal: &Vec3) -> Vec3 {
        match self {
            Self::Point(point) => point.sample_li(sample_point, normal),
            Self::AreaLight(area) => area.sample_li(sample_point, normal),
        }
    }
    pub fn build_json(json: &serde_json::Map<String, Value>)->Self{
        json["mode"].as_str().map(|mode|{
            if mode.contains("Point"){
                Light::Point(PointLight::build_json(json))
            }else{
                // Light::Point(PointLight::build_json(json))
                unimplemented!()
            }
        }).unwrap()
    }
}
fn dist(p1: Vec3, p2: Vec3) -> f32 {
    let len = (p1-p2).length();
    1.0/(len*len)
}
