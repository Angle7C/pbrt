use std::f32::consts::PI;

use glam::{Mat4,  Vec3, Vec2};
use serde_json::Value;

use crate::lib::{
    primitive::Primitive,
    ray::Ray,
    tool::{
        bound::Bound3, interaction::{Interaction, Duv}, parse_1d_param_f32, parse_mat, quadratic,
        tranform::Tranform,
    }, material::Material, sample::{ Sample}, bxdf::{BSDF, reflect::LambertianReflection},
};
#[allow(unused)]
#[derive(Clone,Copy,Debug)]
pub struct Shpere {
    //局部坐标->世界坐标
    object_to_world: Tranform,
    //圆的半径
    radis: f32,
    //最低高度
    zmin: f32,
    //最大高度
    zmax: f32,
    //theta角 球坐标系
    theta_min: f32,
    theta_max: f32,
    //phi角 球坐标系
    phi_max: f32,
    pub material:Option<Material>
}
impl Default for Shpere {
    fn default() -> Self {
        Self {
            object_to_world: Tranform::IDENTITY,
            radis: 1.0,
            zmin: -1.0,
            zmax: 1.0,
            theta_min: 0.0,
            theta_max: PI,
            phi_max: 2.0 * PI,
            material:None,
        }
    }
}
#[allow(unused)]
impl Shpere {
    pub fn new(object_to_world: Mat4, radis: f32, zmin: f32, zmax: f32, phi_max: f32,material:Option<Material>) -> Self {
        Self {
            object_to_world: Tranform::from_mat4(object_to_world),
             radis,
            zmin: zmin.clamp(zmin, zmax).min(-radis),
            zmax: zmax.clamp(zmin, zmax).max(radis),
            theta_min: (zmin / radis).clamp(-1.0, 1.0).acos(),
            theta_max: (zmax / radis).clamp(-1.0, 1.0).acos(),
            phi_max: phi_max.clamp(0.0, 360.0).to_radians(),
            material
        }
    }
    //Json创建
    pub fn build(json: &serde_json::Map<String, Value>) -> Self {
        let mat4 = parse_mat(json);
        let radis = parse_1d_param_f32(json, "radis");
        let zmin = parse_1d_param_f32(json, "zmin");
        let zmax = parse_1d_param_f32(json, "zmax");
        let phi_max = parse_1d_param_f32(json, "phi_max");
        Shpere::new(mat4, radis, zmin, zmax, phi_max,None)
    }
}
#[allow(unused)]
impl Shpere{
    pub fn sample_li<const N:usize> (&self,
        mut sample:Sample<N>,
    ){
        sample.sample_rand_1d();
    }
}
impl Primitive for Shpere {
    //获取球体的包围盒
    fn world_bound(&self) -> Bound3 {
        let min = Vec3::new(-self.radis, -self.radis, self.zmin);
        let max = Vec3::new(self.radis, self.radis, self.zmax);
        let bound = Bound3::new(min,max);
        bound.area();
        self.object_to_world.tranform_bound3(&bound)
    }

    fn interacect(&self, ray: &Ray) -> Option<Interaction> {
        let ray = self.object_to_world.inverse().tranform_ray(ray);
        let a = ray.dir.length_squared();
        let b = 2.0 * (ray.origin.dot(ray.dir));
        let c = ray.origin.length_squared() - self.radis * self.radis;
        match quadratic(a, b, c) {
            None => None,
            Some((x1, x2)) => {
                let mut x = f32::INFINITY;
                for t in [x1, x2] {
                    if t < ray.t_max && t > ray.t_min && t < x {
                        x = t;
                    }
                }
                let slove_point = ray.at(x);
                let normal = slove_point.normalize();
                let slove_point = self.object_to_world.tranform_point3(slove_point);
                let normal = self.object_to_world.tranform_normal(normal);
                let mut inter = Interaction::new(
                    Vec3::ZERO,
                    slove_point,
                    normal,
                    x,
                    Duv::default(),
                    None,
                    None
                );
                let mut bsdf=BSDF::new(&inter, 1.0);
                bsdf.add_bxdf(crate::lib::bxdf::Bxdf::LambertianReflection(LambertianReflection::new(Vec3::splat(0.5)) ));
                inter.bsdf=Some(bsdf);
                Some(inter)
            }
        }
    }

    fn sample_point(&self, sample_point: Vec2)->Vec3 {
        let a=self.theta_min+sample_point.x*(self.theta_max-self.theta_min);
        let b=sample_point.y*self.phi_max;
        let (sin_a,cos_a)=a.sin_cos();
        let (sin_b,cos_b)=b.sin_cos();
        let p=Vec3::new(self.radis*sin_a*cos_b, self.radis*sin_a*sin_b, self.radis*cos_a);
        self.object_to_world.tranform_point3(p)
    }
    fn get_area(&self)->f32 {
        self.phi_max*self.radis*(self.zmax-self.zmin)
    }
}
