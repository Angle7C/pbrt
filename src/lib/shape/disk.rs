use glam::{Vec3, Mat4, Vec2};

use crate::lib::{
    primitive::Primitive,
    ray::Ray,
    tool::{bound::Bound3, interaction::{Interaction, Duv}, tranform::Tranform},
};

#[derive(Clone, Copy, Debug)]
pub struct Disk {
    object_to_world:Tranform,
    h: f32,
    radius: f32,
    inner_radius: f32,
    phi_max: f32,

}
#[allow(unused)]

impl Disk {
    pub fn new(m:Mat4,h: f32, radius: f32, inner_radius: f32, phi_max: f32) -> Self {
        Self {
            object_to_world:Tranform::from_mat4(m),
            h,
            radius:radius.max(0.0),
            inner_radius:inner_radius.max(0.0),
            phi_max: phi_max.clamp(0.0,360.0).to_radians(),
        }
    }
}
impl Primitive for Disk {
    //获取球体的包围盒
    fn world_bound(&self) -> Bound3 {
        let min=Vec3::new(-self.radius,-self.radius,self.h);
        let max=Vec3::new(self.radius,self.radius,self.h);
        let bound=Bound3::new(min, max);
        self.object_to_world.tranform_bound3(&bound)
    }

    fn interacect(&self, ray: &Ray) -> Option<Interaction> {
        let ray=self.object_to_world.inverse().tranform_ray(ray);
        let t=(self.h-ray.origin.z)/ray.dir.z;
        if t>ray.t_max{
            None
        }else{
            let slove_point=ray.at(t);
            let r_2=slove_point.x*slove_point.x+slove_point.y*slove_point.y;
            if  r_2>self.inner_radius*self.inner_radius&&r_2<self.radius*self.radius{
                let nomral=if Vec3::Z.dot(ray.dir)>0.0{Vec3::Z}else{Vec3::NEG_Z};
                let slolve_point=self.object_to_world.tranform_point3(slove_point);
                let normal=self.object_to_world.tranform_normal(nomral);
                Some(Interaction::new(-ray.dir, slolve_point, normal, t, Duv::default(), None, Some(self)))
            }else{
                None
            }
        }
    }
    fn sample_point(&self, sample_point: Vec2)->Vec3 {
        let a=self.phi_max*sample_point.x;
        let (sin,cos)=a.sin_cos();
        let x=((1.0-sample_point.y)*self.radius+sample_point.y*self.inner_radius)*cos;
        let y=((1.0-sample_point.y)*self.radius+sample_point.y*self.inner_radius)*sin;
        
       let vec= Vec3::new(x, y, self.h);
       self.object_to_world.tranform_point3(vec)

    }
    fn get_area(&self)->f32 {
        self.phi_max*(self.radius*self.radius-self.inner_radius*self.inner_radius)*0.5
    }
}
