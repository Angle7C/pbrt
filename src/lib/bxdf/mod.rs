use glam::{Vec2, Vec3};

use self::reflect::{LambertianReflection, SpecularReflection};

use super::{
    sample::Sample,
    tool::{interaction::Interaction, uniform_disk},
};

pub mod reflect;
#[allow(unused)]
#[derive(Clone, Debug)]

pub enum TransportMode {
    Radiance,
    Importance
}
impl PartialEq for TransportMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TransportMode::Radiance, TransportMode::Radiance) => true,
            (TransportMode::Importance, TransportMode::Importance) => true,
            _=>false
            
        }
    }
}
/// BSDF返回值
#[derive(Clone, Copy)]
pub struct SampleBSDF {
    //概率密度
    pub pdf: f32,
    //射出方向
    pub wi: Vec3,
    //bsdf值【0，1】
    pub bsdf: Vec3,
}
impl Default for SampleBSDF {
    fn default() -> Self {
        Self {
            pdf: 1.0,
            wi: Vec3::default(),
            bsdf: Vec3::default(),
        }
    }
}
impl SampleBSDF {
    pub fn new(pdf: f32, wi: Vec3, bsdf: Vec3) -> Self {
        Self { pdf, wi, bsdf }
    }
}
#[allow(unused)]
#[derive(Clone,Copy, Debug)]

pub enum BxdfType {
    REFLECTION = 0,
    TRANSMISSION = 1,
    DIFFUSE = 2,
    GLOSSY = 4,
    SPECULAR = 8,
    ALL = 15,
}
#[derive(Clone, Debug)]
#[allow(unused)]

pub enum Bxdf {
    SpecularReflection(SpecularReflection),
    LambertianReflection(LambertianReflection),
}
#[allow(unused)]

impl Bxdf {
    fn match_type(&self, bxdf: BxdfType) -> bool {
        match self {
            Self::SpecularReflection(specular) => (specular.get_type() | bxdf as u8) >0,
            Self::LambertianReflection(lambertian) => (lambertian.get_type() | bxdf as u8) >0
        }
    }
    fn f(&self, wo: Vec3, wi: Vec3) -> SampleBSDF {
        match self {
            Self::SpecularReflection(specular) => specular.f(wo, wi),
            Self::LambertianReflection(lambertian) => lambertian.f(wo, wi),
        }
    }
    fn sample_f(&self, wo: Vec3, wi: Vec3, sample: Sample<10>) -> SampleBSDF {
        todo!()
    }
    //求反射率
    fn rho(&self) {}
    fn pdf(&self) {}
}
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct BSDF {
    pub eta: f32,
    ns: Vec3,
    ng: Vec3,
    ss: Vec3,
    ts: Vec3,
    bxdfs: Vec<Bxdf>,
}
#[allow(unused)]
impl BSDF {
    pub fn new(inter: &Interaction, eta: f32) -> Self {
        Self {
            eta: eta,
            ns: inter.duv.shading.n,
            ng: inter.normal,
            ss: inter.duv.shading.dpdu.normalize(),
            ts: inter
                .duv
                .shading
                .n
                .cross(inter.duv.shading.dpdu.normalize()),
            bxdfs: vec![],
        }
    }
    pub fn add_bxdf(&mut self,bxdf:Bxdf){
        self.bxdfs.push(bxdf);
    }
    //通过bxdf返回 新方向，bsdf值，pdf
    pub fn sample_f(
        &self,
        wi: Vec3,
        sample_lobe: f32,
        sample_point: Vec2,
        bxdf: BxdfType,
    ) -> SampleBSDF {
        todo!()
    }
    pub fn f(&self, wo: Vec3, bxdf: BxdfType) -> SampleBSDF {
        let wi = uniform_disk();
        let mut vec = vec![];
        for item in &self.bxdfs {
            if item.match_type(bxdf) {
                vec.push(item.f(wo, wi).bsdf)
            }
        }
        let color:Vec3=vec.iter().sum();
        let color=color/vec.len() as f32;
        SampleBSDF::new(1.0,wi,color)
    }

    pub fn shading_to_world(&self){

    }
}
