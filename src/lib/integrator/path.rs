use crate::lib::{bxdf::BxdfType, ray::Ray};
#[allow(unused)]
use crate::lib::{
    camera::{Camera, CameraSample},
    sample::{stratified::StratifiedSampler, PixelSample, Sample},
    shape::Shape,
    tool::{film::Film, interaction::Interaction, log_init, scene::Sence, THREAD_NUM},
};
use glam::{Vec2, Vec3};
#[allow(unused)]
use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
    time::SystemTime,
};

use super::to_color;
//俄罗斯轮盘的路径追踪
#[allow(unused)]

pub struct PathIntegrator<const N: usize> {
    //被终止的概率
    q: f32,
    //在递归深度大于max_path时，开始俄罗斯轮盘
    max_path: usize,
    sampler: Sample<N>,
}
#[allow(unused)]
impl<const N: usize> Default for PathIntegrator<N> {
    fn default() -> Self {
        Self {
            q: 0.5,
            max_path: 10,
            sampler: Sample::StratifiedSampler(StratifiedSampler::new(false)),
        }
    }
}
#[allow(unused)]
impl<const N: usize> PathIntegrator<N> {
    //路径追踪积分器
    pub fn new(q: f32, max_path: usize, jitter: bool) -> Self {
        Self {
            q,
            max_path,
            sampler: Sample::StratifiedSampler(StratifiedSampler::<N>::new(jitter)),
        }
    }
    #[inline]
    //终止求解
    fn is_next(&self, p: f32, dept: &mut usize) -> Option<f32> {
        *dept += 1;
        if *dept > self.max_path {
            // if p > self.q {
            //     None
            // } else {
            //     Some(1.0 - self.q)
            // }
            None
        } else {
            Some(1.0)
        }
    }
    //求一条路径的值
    fn fi(&self, sence: &Sence, ray: &Ray, sampler: &mut Sample<N>) -> Vec3 {
        let mut p = 0.0;
        let mut ray = Ray::new(ray.origin, ray.dir);

        let mut depth = 0;
        let mut stack: Vec<Ray> = vec![];
        let mut color = if let Some(inter) = sence.intersect(&ray) {
            //直接光照
            
            sence.direct_lighting(
                &inter,
                sence,
                Vec2::new(sampler.sample_rand_1d(), sampler.sample_rand_1d()),
            )
        } else {
           return Vec3::ZERO;
        };
        if color==Vec3::ZERO{
            return Vec3::ZERO
        }
        while let Some(pp) = self.is_next(p, &mut depth) {
            color = if let Some(inter) = sence.intersect(&ray) {
                //间接光照
                color
                    * if let Some(bsdf) = inter.bsdf {
                        let t = bsdf.f(ray.dir, BxdfType::All);
                        ray = Ray::new(inter.p, t.wi);
                        t.bsdf
                    } else {
                        return color;
                    }
            } else {
                color
            };
            p = sampler.sample_rand_1d()
        }
        color
    }
    ///单线程渲染
    pub fn render(self, sence: &Sence, name: &str) {
        let mut image = sence.build_image();
        let film = Film::new(&image);
        let camera = &sence.camera;
        let mut sampler = self.sampler.clone();
        let mut ray_sapler = self.sampler.clone();
        let mut bar = pbr::ProgressBar::new(film.size());
        while let Some(item) = film.iter() {
            bar.add(1);
            for (u, v) in item {
                let mut pixel_sample = PixelSample::new(u as f32, v as f32);
                let mut color = Vec3::ZERO;
                for _ in 0..N {
                    let camera_sample =
                        CameraSample::new(pixel_sample.sample_2d(&mut sampler), Vec2::ZERO);
                    let (ray, weight) = camera.generate_ray(camera_sample);
                    color += self.fi(sence, &ray, &mut ray_sapler.clone())
                }
                *image.get_pixel_mut(u, v) = to_color(color, N as f32);
            }
        }
        bar.finish();
        let (x,y)=image.dimensions();
        image.save(format!("F:/Dept/ray_track_weekend_rs/pbrt/image/{x}_{y}_{name}_{N}.png"));
    }
    ///多线程渲染
    pub fn render_process(self, sence: &Sence, name: &str) {

    }
}
#[test]
pub fn test_path() {
    log_init();
    let sence = Sence::read_json("D:\\Depot\\pbrt\\sence\\template.json");
    let path = PathIntegrator::<64>::default();
    path.render(&sence, "tset_path_reflect");
}
