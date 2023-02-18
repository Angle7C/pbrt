use glam::Vec3;
use image::Rgb;

use self::path::PathIntegrator;

pub mod path;
#[allow(unused)]
pub enum Integrator<const N: usize> {
    Path(PathIntegrator<N>),
}
fn to_color(vec: Vec3, ssp: f32) -> Rgb<u8> {
    let vec = (vec / ssp).powf(0.5);
    let color = vec;
    let rgb = color * 255.0;
    Rgb([
        rgb.x.clamp(0.0, 255.0) as u8,
        rgb.y.clamp(0.0, 255.0) as u8,
        rgb.z.clamp(0.0, 255.0) as u8,
    ])
}
