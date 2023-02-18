use glam::{Vec2, Vec3};

use crate::lib::tool::INV_PI;

use super::{BxdfType, SampleBSDF, TransportMode};

//菲涅尔定律
#[allow(unused)]
pub enum Fresnel {
    NoOP(NoOPFresnel),
    Conductor(ConductorFresnel),
    Dielectric(DielectricFresnel),
    Disney(DisneyFrenel),
}
#[allow(unused)]

impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        match self {
            Fresnel::NoOP(noop) => noop.evaluate(cos_theta_i),
            Fresnel::Dielectric(die) => die.evaluate(cos_theta_i),
            Fresnel::Conductor(con) => con.evaluate(cos_theta_i),
            Fresnel::Disney(dis) => dis.evaluate(cos_theta_i),
        }
    }
}
#[allow(unused)]

pub struct DisneyFrenel {
    r0: Vec3,
    metallic: f32,
    eta: f32,
}
#[allow(unused)]

impl DisneyFrenel {
    pub fn new(r0: Vec3, metallic: f32, eta: f32) -> Self {
        Self { r0, metallic, eta }
    }
    pub fn evaluate(&self, cos_i: f32) -> Vec3 {
        let r = fr_dielectric(cos_i, 1.0, self.eta);
        let a = fr_schlick_spectrum(self.r0, cos_i);
        Vec3::lerp(Vec3::splat(r), a, self.metallic)
    }
}
#[allow(unused)]

pub struct ConductorFresnel {
    eta_i: Vec3,
    eta_t: Vec3,
    k: Vec3,
}
#[allow(unused)]

impl ConductorFresnel {
    pub fn new(eta_i: Vec3, eta_t: Vec3, k: Vec3) -> Self {
        Self { eta_i, eta_t, k }
    }
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        fr_conductor(cos_theta_i, self.eta_i, self.eta_t, self.k)
    }
}
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct DielectricFresnel {
    eta_i: f32,
    eta_t: f32,
}
impl DielectricFresnel {
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        Vec3::splat(fr_dielectric(cos_theta_i, self.eta_i, self.eta_t))
    }
}
#[allow(unused)]

pub struct NoOPFresnel;
#[allow(unused)]

impl NoOPFresnel {
    pub fn evaluate(&self, _cos_theta_i: f32) -> Vec3 {
        Vec3::ONE
    }
}

/// 镜面反射
#[allow(unused)]
#[derive(Clone, Debug)]

pub struct SpecularReflection {
    t: Vec3,
    eta_a: f32,
    eta_b: f32,
    frensnel: DielectricFresnel,
    mode: TransportMode,
    sc_opt: Option<Vec3>,
}
#[allow(unused)]
impl SpecularReflection {
    pub fn new(t: Vec3, eta_a: f32, eta_b: f32, mode: TransportMode, sc_opt: Option<Vec3>) -> Self {
        Self {
            t,
            eta_a,
            eta_b,
            frensnel: DielectricFresnel {
                eta_i: eta_a,
                eta_t: eta_b,
            },
            mode: mode,
            sc_opt: sc_opt,
        }
    }
    pub fn f(&self, wo: Vec3, wi: Vec3) -> SampleBSDF {
        SampleBSDF::default()
    }
    // 返回方向，pdf
    pub fn sample_f(&self, wo: Vec3, sample: &Vec2) -> (Vec3, f32) {
        let entering = wo.z > 0.0;
        let eta_i = if entering { self.eta_a } else { self.eta_b };
        let eta_t = if entering { self.eta_b } else { self.eta_a };
        let noraml = if Vec3::Z.dot(wo) < 0.0 {
            Vec3::NEG_ONE
        } else {
            Vec3::Z
        };
        let mut wi = Vec3::default();
        if refract(&wo, &noraml, eta_i / eta_t, &mut wi) {
            (Vec3::ZERO, 0.0)
        } else {
            let pdf = 1.0;
            let mut ft = self.t * (Vec3::ONE - self.frensnel.evaluate(wi.z));
            if self.mode == TransportMode::Radiance {
                ft *= Vec3::splat((eta_i * eta_i) / (eta_t * eta_t));
            }
            ft = if let Some(sc) = self.sc_opt {
                sc * ft / wi.z.abs()
            } else {
                ft / wi.z.abs()
            };
            (ft, pdf)
        }
    }
    pub fn pdf(&self, wo: Vec3, wi: Vec3) -> f32 {
        if wo.z * wi.z > 0.0 {
            wi.z * INV_PI
        } else {
            0.0
        }
    }
    pub fn get_type(&self) -> u8 {
        BxdfType::TRANSMISSION as u8 | BxdfType::SPECULAR as u8
    }
}
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct LambertianReflection {
    r: Vec3,
}
#[allow(unused)]

pub struct FrensnelSpecular {
    r: Vec3,
    t: Vec3,
    eta_a: f32,
    eta_b: f32,
    mode: TransportMode,
    sc_opt: Option<Vec3>,
}
#[allow(unused)]

impl FrensnelSpecular {
    pub fn new(
        r: Vec3,
        t: Vec3,
        eta_a: f32,
        eta_b: f32,
        mode: TransportMode,
        sc_opt: Option<Vec3>,
    ) -> Self {
        Self {
            r,
            t,
            eta_a,
            eta_b,
            mode,
            sc_opt,
        }
    }
    pub fn f(wo: &Vec3, wi: &Vec3) -> Vec3 {
        Vec3::ZERO
    }
    pub fn sample_f(&self) -> Vec3 {
        todo!()
    }
    pub fn get_type(&self) -> u8 {
        BxdfType::REFLECTION as u8 | BxdfType::TRANSMISSION as u8 | BxdfType::SPECULAR as u8
    }
}
#[allow(unused)]
impl LambertianReflection {
    pub fn new(r: Vec3) -> Self {
        Self { r: r }
    }
    pub fn f(&self, _wo: Vec3, _wi: Vec3) -> SampleBSDF {
        let mut bsdf = SampleBSDF::default();

        let bsdf_value = self.r * Vec3::splat(INV_PI);

        bsdf.bsdf = bsdf_value;
        bsdf
    }
    pub fn sample_f(&self, wo: Vec3, u: &Vec2, _sampled_type: u8, wi: &mut Vec3) -> (Vec3, f32) {
        todo!()
    }
    pub fn pdf(&self, wo: Vec3, wi: Vec3) -> f32 {
        if wo.z * wi.z > 0.0 {
            wi.z * INV_PI
        } else {
            0.0
        }
    }
    pub fn get_type(&self) -> u8 {
        BxdfType::DIFFUSE as u8 | BxdfType::REFLECTION as u8
    }
}
pub fn fr_dielectric(cos_theta_i: f32, eta_i: f32, eta_t: f32) -> f32 {
    let mut cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
    let entering = cos_theta_i > 0.0;
    let mut local_eta_i = eta_i;
    let mut local_eta_t = eta_t;
    if !entering {
        std::mem::swap(&mut local_eta_i, &mut local_eta_t);
        cos_theta_i = cos_theta_i.abs();
    }
    let sin_theta_i = 0.0f32.max(1.0 - cos_theta_i * cos_theta_i).sqrt();
    let sin_theta_t = local_eta_i / local_eta_t * sin_theta_i;
    if sin_theta_t >= 1.0 {
        return 1.0;
    }
    let cos_theta_t = 0.0f32.max(1.0 - sin_theta_t * sin_theta_t).sqrt();
    let r_parl = ((local_eta_t * cos_theta_i) - (local_eta_i * cos_theta_t))
        / ((local_eta_t * cos_theta_i) + (local_eta_i * cos_theta_t));
    let r_perp = ((local_eta_i * cos_theta_i) - (local_eta_t * cos_theta_t))
        / ((local_eta_i * cos_theta_i) + (local_eta_t * cos_theta_t));
    (r_parl * r_parl + r_perp * r_perp) / 2.0
}
#[allow(unused)]

pub fn fr_conductor(cos_theta_i: f32, eta_i: Vec3, eta_t: Vec3, k: Vec3) -> Vec3 {
    let not_clamped = cos_theta_i;
    let cos_theta_i = not_clamped.clamp(-1.0, 1.0);
    let eta = eta_t / eta_i;
    let eta_k = k / eta_i;
    let cos_theta_i2 = cos_theta_i * cos_theta_i;
    let sin_theta_i2 = 1.0 - cos_theta_i2;
    let eta_2 = eta * eta;
    let eta_k2 = eta_k * eta_k;
    let t0 = eta_2 - eta_k2 - Vec3::splat(sin_theta_i2);
    let a2_plus_b2 = (t0 * t0 + eta_2 * eta_k2 * Vec3::splat(4.0)).powf(0.5);
    let t1 = a2_plus_b2 + Vec3::splat(cos_theta_i2);
    let a = ((a2_plus_b2 + t0) * 0.5).powf(0.5);
    let t2 = a * 2.0 * cos_theta_i;
    let rs = (t1 - t2) / (t1 + t2);
    let t3 = a2_plus_b2 * cos_theta_i2 + Vec3::splat(sin_theta_i2 * sin_theta_i2);
    let t4 = t2 * sin_theta_i2;
    let rp = rs * (t3 - t4) / (t3 + t4);
    (rp + rs) * Vec3::splat(0.5)
}
#[inline]
#[allow(unused)]

fn pow5(v: f32) -> f32 {
    (v * v) * (v * v) * v
}
#[allow(unused)]

fn schlick_weight(cos_theta: f32) -> f32 {
    let m = (1.0 - cos_theta).clamp(0.0, 1.0);
    pow5(m)
}
#[allow(unused)]

pub fn fr_schlick(r0: f32, cos_theta: f32) -> f32 {
    schlick_weight(cos_theta)
}
#[allow(unused)]

fn fr_schlick_spectrum(r0: Vec3, cos_theta: f32) -> Vec3 {
    let r = schlick_weight(cos_theta);
    Vec3::lerp(r0, Vec3::ONE, r)
}
fn refract(wi: &Vec3, n: &Vec3, eta: f32, wt: &mut Vec3) -> bool {
    let cos_theta_i = n.dot(*wi);
    let sin2_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
    let sin2_theta_t = eta * eta * sin2_theta_i;
    if sin2_theta_t >= 1.0 {
        return false;
    }
    let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
    *wt = (-(*wi) * eta) + *n * (eta * cos_theta_i - cos_theta_t);
    true
}
