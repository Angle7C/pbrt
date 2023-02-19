use std::{
    fs::File,
    io::{BufReader, Read},
};

use glam::{Mat4, Vec2, Vec3};
use image::RgbImage;
use log::error;
use obj::{IndexTuple, ObjData};
use rand::Rng;
use serde_json::Value;

use crate::lib::{
    camera::{orthographic::Orthographic, parse_camera, Camera},
    light::Light,
    primitive::{build_aggregate, Aggregate, AggregateType, Primitive},
    ray::Ray,
    shape::{shpere::Shpere, triangle::Triangle, Shape},
};

use super::{
    bound::{Bound2, Bound3},
    interaction::Interaction,
};
pub struct Sence {
    pub camera: Camera,
    pub lights: Vec<Light>,
    bound: Bound3,
    aggregate: Option<Box<dyn Aggregate>>,
}
impl Default for Sence {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            lights: vec![],
            bound: Bound3::new(Vec3::ZERO, Vec3::ZERO),
            aggregate: None,
        }
    }
}
#[allow(unused)]
impl Sence {
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    //dur读取Json创建
    pub fn read_json(path: &str) -> Self {
        let mut buf = String::new();
        let sence = if let Ok(mut file) = File::open(path) {
            file.read_to_string(&mut buf);
            let json: Value = serde_json::from_str(&buf).unwrap();
            let camera = parse_camera(&json);
            let mut shapes = parse_shapes(&json);
            let lights = parse_light(&json);
            let mut bound = Bound3::default();
            for item in &shapes {
                bound = bound.merage(&item.world_bound());
            }
            let aggregate = build_aggregate(AggregateType::Bvh, shapes);
            let mut t = Self {
                camera,
                lights,
                bound,
                aggregate: Some(aggregate),
            };
            return t;
        } else {
            error!("读取文件失败 {}", path);
            panic!("读取文件失败 {}", path);
        };
        sence
    }
    pub fn rand_read(light_num: u32, shape_num: u32) -> Self {
        let camera = Camera::Orthographic(Orthographic::new(
            Mat4::IDENTITY,
            0.001,
            1000.0,
            Bound2::new(Vec2::NEG_ONE, Vec2::ONE),
            0.0,
            0.0,
            (512, 512),
        ));
        let mut rand = rand::thread_rng();
        let mut shapes = vec![];
        for i in 0..shape_num {
            let radis = rand.gen_range(0.3..1.0);
            let x = rand.gen_range(-10.0..10.0);
            let y = rand.gen_range(-10.0..10.0);
            let z = rand.gen_range(10.0..20.0);
            let shape = Shpere::new(
                Mat4::from_translation(Vec3::new(x, y, z)),
                radis,
                -radis,
                radis,
                360.0,
                None,
            );
            shapes.push(Shape::Shpere(shape));
        }
        let mut bound = Bound3::default();

        for item in &shapes {
            bound = bound.merage(&item.world_bound());
        }
        Self {
            aggregate: Some(build_aggregate(AggregateType::Bvh, shapes)),
            camera,
            lights: vec![],
            bound,
        }
    }
    //创建图片
    pub fn build_image(&self) -> RgbImage {
        let (x, y) = self.camera.get_film_size();
        RgbImage::new(x, y)
    }
    //计算光线和当前场景的交点
    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        if self.bound.intesect(ray) {
            if let Some(aggregate) = &self.aggregate {
                aggregate.interacect(ray)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn direct_lighting(&self, inter: &Interaction, sence: &Sence, sample_point: Vec2) -> Vec3 {
        let mut color = Vec3::ZERO;
        for light in &self.lights {
            if light.vis_li(inter, sence, sample_point) {
                color += light.sample_li(&inter.p, &inter.normal);
            }
        }
        color / self.lights.len() as f32
    }
}

fn parse_shapes(json: &Value) -> Vec<Shape> {
    let shapes = json["shapes"].as_array().unwrap();
    let mut vec = vec![];
    for iter in shapes {
        let item = iter.as_object().unwrap();
        if item.is_empty() {
            continue;
        }
        vec.push(Shape::build_json(item))
    }
    vec
}
fn parse_light(json: &Value) -> Vec<Light> {
    let mut vec = vec![];
    let light = json["lights"].as_array().unwrap();
    for iter in light {
        let item = iter.as_object().unwrap();
        if item.is_empty() {
            continue;
        }
        vec.push(Light::build_json(item));
    }
    vec
}
#[allow(unused)]
impl Sence {
    pub fn read_obj(path: &'static str) {
        let input = File::open(path).expect("这个文件是错误的");
        let read_buf = BufReader::new(input);
        let model = ObjData::load_buf(read_buf).expect("解析模型文件失败");
        //遍历模型
        for obj in &model.objects {
            //遍历每一个面集合
            let mut vec = Vec::new();
            for item in &obj.groups {
                for ploy in &item.polys {
                    let mut pos = [Vec3::ZERO; 3];
                    let mut uv = [Vec2::ZERO; 3];
                    let mut n = [Vec3::ZERO; 3];
                    for (i, IndexTuple(index, texture, normal)) in ploy.0.iter().enumerate() {
                        if i > 2 {
                            panic!("这个模型存在非三角面")
                        }
                        pos[i] = Vec3::from_array(model.position[*index]);
                        uv[i] = if let Some(texture) = texture {
                            Vec2::from_array(model.texture[*texture])
                        } else {
                            Vec2::ZERO
                        };
                        n[i] = if let Some(normal) = normal {
                            Vec3::from_array(model.normal[*normal])
                        } else {
                            Vec3::Z
                        };
                    }
                    vec.push(Triangle::new(pos, n, uv))
                }
            }
        }
    }
}
