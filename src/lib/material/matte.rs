use glam::Vec3;

use crate::lib::tool::interaction::Interaction;
#[derive(Debug,Clone, Copy)]
pub struct Matte{
    //反射值，
    kd:Option<Vec3>,
    //粗糙度
    sigma:Option<f32>,
    bump_map:Option<f32>,
    /*
        kd: kd图
        sigma: sigma材质
        bump_map: bump材质
    */
}
impl Matte{
    pub fn new()->Self{
        todo!()
    }
    pub fn scattering(&self,inter:&Interaction){

    }
}