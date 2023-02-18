use super::{CameraSample, parse_camera_2d_param, parse_mat_camera};
#[allow(unused)]
use super::{compute_screen_to_raster, Camera};
use crate::lib::{tool::{parse_1d_param_f32}};
#[allow(unused_imports)]
use crate::lib::tool::log_init;
#[allow(unused)]
use crate::lib::{
    ray::Ray,
    sample::{stratified::StratifiedSampler, PixelSample, Sample},
    tool::{
        bound::Bound2,
        film::{self, Film, FilmIter},
        interaction::Interaction,
        tranform::Tranform,
        THREAD_NUM,
    },
};

use glam::{Mat4, Vec2, Vec3};
#[allow(unused)]
use image::RgbImage;
use serde_json::{Map, Value};
#[allow(unused)]
use std::sync::mpsc::{channel, Sender};
#[allow(unused)]
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
    thread::{self, Thread},
};
#[derive(Clone, Copy)]
#[allow(unused)]
pub struct Perspective {
    //相机坐标->世界坐标
    camera_to_world: Tranform,
    //近平面->图像
    screen_to_raster: Tranform,
    //图像->相机坐标
    raster_to_camera: Tranform,
    lens_radius: f32,
    focal_distance: f32,
    dx_camera: Vec3,
    dy_camera: Vec3,
    size: (u32, u32),
}
impl Default for Perspective {
    fn default() -> Self {
        let screen_to_raster =
            compute_screen_to_raster(Bound2::new(Vec2::NEG_ONE, Vec2::ONE), (512, 512));
        let camera_to_screen =
            Tranform::from_mat4(Mat4::perspective_infinite_lh(90.0, 1.0, 10000.0 ));
        let raster_to_camera = camera_to_screen.inverse() * screen_to_raster.inverse();
        Self {
            dx_camera: raster_to_camera.tranform_vector3(Vec3::X),
            dy_camera: raster_to_camera.tranform_vector3(Vec3::Y),
            camera_to_world: Tranform::from_mat4(Mat4::IDENTITY),
            screen_to_raster: screen_to_raster,
            raster_to_camera: raster_to_camera,
            lens_radius: 1.0,
            focal_distance: 1.0,
            size: (512, 512),
        }
    }
}
#[allow(unused)]
impl Perspective {
    pub fn new(
        camera_to_world: Mat4,
        fov:f32,
        screen_window: Bound2,
        lens_radius: f32,
        focal_distance: f32,
        size: (u32, u32),
    ) -> Self {
        let screen_to_raster = compute_screen_to_raster(screen_window, size);
        let camera_to_screen =
            Tranform::from_mat4(Mat4::perspective_infinite_lh(fov, size.0 as f32/size.1 as f32, 10000.0 ));
        let raster_to_camera = camera_to_screen.inverse() * screen_to_raster.inverse();

        Self {
            dx_camera: raster_to_camera.tranform_vector3(Vec3::X),
            dy_camera: raster_to_camera.tranform_vector3(Vec3::Y),
            camera_to_world: Tranform::from_mat4(camera_to_world),
            screen_to_raster: screen_to_raster,
            raster_to_camera: raster_to_camera,
            lens_radius: lens_radius,
            focal_distance: focal_distance,
            size: size,
        }
    }

    //生成光线，需要一个采样器生成的样本。 返回光线 和 光线权重 [0,1]
    pub fn generate_ray(&self, sample: CameraSample) -> (Ray, f32) {
        // 将图像坐标变成相机坐标
        let p_camera = self
            .raster_to_camera
            .tranform_point3(sample.p_film.extend(0.0));
        // 正交相机，生成垂直于近平面的光线
        let ray = Ray::new(Vec3::ZERO, p_camera.normalize());
        (self.camera_to_world.tranform_ray(&ray), 1.0)
    }
    pub fn get_film_size(&self) -> (u32, u32) {
        self.size
    }
}

impl Perspective{
    pub fn build_json(camera:&Map<String,Value>)->Self{
        let mat4 = parse_mat_camera(camera);
        let focal_distance=parse_1d_param_f32(camera,"focal_distance");
        let lens_radius=parse_1d_param_f32(camera, "lens_radius");
        let (bound,size)=parse_camera_2d_param(camera);
        let fov=parse_1d_param_f32(camera, "fov");
        Self::new(mat4, fov, bound, lens_radius, focal_distance, size)
    }

}