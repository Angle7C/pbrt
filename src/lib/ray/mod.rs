use glam::Vec3;

#[derive(Debug)]
pub struct Ray{
    pub origin:Vec3,
    pub dir:Vec3,
    pub t_min:f32,
    pub t_max:f32
}
impl PartialEq for Ray{
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin)&& self.dir.eq(&other.dir)
    }
}
#[allow(unused)]
impl Ray {
    pub fn new(origin:Vec3,dir:Vec3)->Self{
        Self { origin: origin, dir: dir.normalize(),t_max:f32::INFINITY,t_min:0.0 }
    }
    pub fn at(&self,t:f32)->Vec3{
        self.origin+self.dir*t
    }
    pub fn to_rtbvh(&self)->rtbvh::Ray{
        rtbvh::Ray::new(self.origin, self.dir)
    }
    pub fn from_with_t(origin:Vec3,dir:Vec3,t_min:f32,t_max:f32)->Self{
        Self { origin: origin, dir: dir.normalize(),t_max:t_max,t_min:t_min }

    }
}
