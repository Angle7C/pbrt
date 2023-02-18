use std::f32::consts::PI;

use glam::{Vec2, Vec3, Mat4};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use serde_json::Value;
pub mod bound;
pub mod film;
pub mod interaction;
pub mod tranform;
pub mod scene;
pub type Point3 = glam::Vec3;
#[allow(unused)]

pub const INV_PI: f32 = 1.0 / PI;
pub const THREAD_NUM: u32 = 8;
pub const VEC2_MAX: Vec2 = Vec2::splat(f32::MAX);
pub const VEC2_MIN: Vec2 = Vec2::splat(f32::MIN);

pub fn quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        None
    } else {
        let det = det.sqrt();
        Some(((-b + det) / (2.0 * a), (-b - det) / (2.0 * a)))
    }
}
#[allow(unused)]
pub fn log_init() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}


pub fn parse_vec3(json: &serde_json::Map<String, Value>, name: &str) -> Vec3 {
    let name = json[name].as_object().unwrap();
    let (x, y, z) = (
        name["x"].as_f64().unwrap_or_default(),
        name["y"].as_f64().unwrap_or_default(),
        name["z"].as_f64().unwrap_or_default(),
    );
    Vec3::new(x as f32, y as f32, z as f32)
}
pub fn parse_mat(json: &serde_json::Map<String, Value>) -> Mat4 {
    let pos = parse_vec3(json, "pos");
    let scale = parse_vec3(json, "scale");
    // let mut rotation = parse_vec3(json, "rotation");
    // rotation.x=rotation.x.to_radians();
    // rotation.y=rotation.y.to_radians();
    // rotation.z=rotation.z.to_radians();
    // let quat = Quat::from_rotation_arc(Vec3::ZERO, rotation);
    Mat4::from_translation(pos)*Mat4::from_scale(scale)
}

pub fn parse_1d_param_f32(json: &serde_json::Map<String, Value>,name:&str)->f32{
    json[name].as_f64().unwrap_or_default() as f32
}
pub fn parse_1d_param_str(json: &serde_json::Map<String, Value>,name:&str)->String{

    json[name].as_str().unwrap().to_string()
}
pub fn uniform_disk()->Vec3{
    let r=rand::random::<f32>();
    let xie=rand::random::<f32>()*2.0*PI;
    let (sin,cos)=xie.sin_cos();
    let (x,y)=(r*cos,r*sin);
    let dir=Vec3::new(x,y,(1.0-x*x-y*y).sqrt());
    dir

}