use glam::{Mat4, Vec2, Vec3};
use serde_json::Value;

use self::{orthographic::Orthographic, perspective::Perspective};

use super::{
    ray::Ray,
    tool::{
        bound::Bound2,
        parse_1d_param_str, parse_vec3,
        tranform::{Tranform},
    },
};

pub mod orthographic;
pub mod perspective;
#[allow(unused)]
pub enum Camera {
    Orthographic(orthographic::Orthographic),
    Perspective(Perspective),
}
#[allow(unused)]
impl Default for Camera {
    fn default() -> Self {
        // Camera::Orthographic(orthographic::Orthographic::default())
        Camera::Perspective(Perspective::default())
    }
}
#[allow(unused)]
pub struct CameraSample {
    p_film: Vec2,
    p_lens: Vec2,
}
impl Default for CameraSample {
    fn default() -> Self {
        Self {
            p_film: Vec2::ZERO,
            p_lens: Vec2::ZERO,
        }
    }
}
#[allow(unused)]
impl CameraSample {
    pub fn new(p_film: Vec2, p_lens: Vec2) -> Self {
        Self {
            p_film,
            p_lens,
        }
    }
}
#[allow(unused)]
impl Camera {
    pub fn generate_ray(&self, sample: CameraSample) -> (Ray, f32) {
        match self {
            Self::Orthographic(o) => o.generate_ray(sample),
            Self::Perspective(p) => p.generate_ray(sample),
        }
    }
    pub fn get_film_size(&self) -> (u32, u32) {
        match self {
            Self::Orthographic(o) => o.get_film_size(),
            Self::Perspective(p) => p.get_film_size(),
        }
    }
}
#[allow(unused)]
fn compute_screen_to_raster(screen_window: Bound2, film_size: (u32, u32)) -> Tranform {
    let m = Mat4::from_translation(Vec3::new(
        (-screen_window.min.x + screen_window.max.x) / 2.0,
        (screen_window.max.y - screen_window.min.y) / 2.0,
        0.0,
    ));
    let scale = Mat4::from_scale(Vec3::new(
        1.0 / (screen_window.max.x - screen_window.min.x),
        1.0 / (screen_window.max.y - screen_window.min.y),
        1.0,
    ));
    let raster = Mat4::from_scale(Vec3::new(film_size.0 as f32, film_size.1 as f32, 1.0));
    let m = raster * scale * m;
    Tranform::from_mat4(m)
}

pub fn parse_camera(json: &Value) -> Camera {
    let camera = json["camera"].as_object().unwrap();

    let mode = parse_1d_param_str(camera, "mode");

    if mode.contains("Orthographic") {
        Camera::Orthographic(Orthographic::build_json(camera))
    } else if mode.contains("Perspective") {
        Camera::Perspective(Perspective::build_json(camera))
    } else {
        Camera::default()
    }
}
fn parse_mat_camera(json: &serde_json::Map<String, Value>) -> Mat4 {
    let pos = parse_vec3(json, "pos");
    let look = parse_vec3(json, "lookat");
    let up = parse_vec3(json, "up");
    Mat4::look_at_lh(pos, look, up)
}
pub fn parse_camera_2d_param(json: &serde_json::Map<String, Value>) -> (Bound2, (u32, u32)) {
    let screen_window = if let Some(screen_window) = json["screen_window"].as_object() {
        let min = if let Some(min) = screen_window["min"].as_object() {
            let x = min["x"].as_f64().unwrap_or(-1.0);
            let y = min["y"].as_f64().unwrap_or(-1.0);
            Vec2::new(x as f32, y as f32)
        } else {
            Vec2::NEG_ONE
        };
        let max = if let Some(max) = screen_window["max"].as_object() {
            let x = max["x"].as_f64().unwrap_or(1.0);
            let y = max["y"].as_f64().unwrap_or(1.0);
            Vec2::new(x as f32, y as f32)
        } else {
            Vec2::ONE
        };
        Bound2::new(min, max)
    } else {
        Bound2::new(Vec2::NEG_ONE, Vec2::ONE)
    };
    let size = if let Some(size) = json["size"].as_object() {
        let x = size["x"].as_u64().unwrap_or(512) as u32;
        let y = size["y"].as_u64().unwrap_or(512) as u32;
        (x, y)
    } else {
        (512, 512)
    };
    (screen_window, size)
}
