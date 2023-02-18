use glam::{Vec2, Vec3};
use image::Rgb;

use crate::lib::{bxdf::{BSDF}, shape::Shape, primitive::Primitive};
#[derive(Default, Debug)]
pub struct Shading{
   pub n:Vec3,
   pub  dpdu:Vec3,
   pub  dpdv:Vec3,
   pub dndu:Vec3,
   pub dndv:Vec3
}
#[allow(unused)]
#[derive(Default, Debug)]
pub struct Duv<'a> {
    pub uv: Vec2,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Vec3,
    pub dndv: Vec3,
    pub shape: Option<&'a Shape>,
    pub shading:Shading
}
#[derive(Default, Debug)]

pub struct Dxy{
    pub dpdx:Vec3,
    pub dpdy:Vec3,
    pub dudx:f32,
    pub dvdx:f32,
    pub dudy:f32,
    pub dvdy:f32
}
#[derive(Default, Debug)]
pub struct Interaction<'a> {
    //入射方向
    pub w0: Vec3,
    //击中点
    pub p: Vec3,
    //表面法线
    pub normal: Vec3,
    //传播时间
    pub time: f32,
    //uv坐标微分
    pub duv: Duv<'a>,
    pub dxy: Dxy,
    pub bsdf: Option<BSDF>,
   
    pub primitive:Option<&'a dyn Primitive>
}
#[allow(unused)]
impl<'a> Interaction<'a> {
    pub fn new(
        w0: Vec3,
        p: Vec3,
        normal: Vec3,
        time: f32,
        duv: Duv<'a>,
        bsdf: Option<BSDF>,
        primitive:Option<&'a dyn Primitive>
    ) -> Self {
        Self {
            w0,
            p,
            normal,
            time,
            duv,
            dxy:Dxy::default(),
            bsdf,
            primitive
        }
    }
    pub fn to_rgb(&self, ssp: f32) -> Rgb<u8> {
        let normal = (self.normal / ssp + Vec3::ONE * 0.5).powf(0.5);
        let color = normal.clamp(Vec3::ZERO, Vec3::ONE);
        let rgb = color * 255.0;
        Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8])
    }
}
