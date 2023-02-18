use glam::Vec2;
use rand::{
    rngs::{StdRng, ThreadRng},
    SeedableRng, Rng,
};

use super::{ONE_MINUS_EPSILON};

#[allow(unused)]
#[derive(Clone)]
pub struct StratifiedSampler<const N: usize> {
    //X轴样本数
    x_samples: usize,
    //Y轴样本数
    y_samples: usize,
    //是否随机影响
    jitter: bool,
    //对角线
    random: StdRng,
    sample_2d: Box<[(f32, f32); N]>,
    sample_1d: Box<[f32; N]>,
    sample_rand_1d:Box<[f32;N]>,
    now_1d:usize,
    now_2d:usize,
    now_rand_1d:usize
}
#[allow(unused)]
impl<const N: usize> StratifiedSampler<N> {
    pub fn new( jitter: bool) -> Self {
        let rand = ThreadRng::default();
        let mut a = Self {
            x_samples: N,
            y_samples: N,
            sample_1d: Box::new([0.0; N]),
            sample_2d: Box::new([(0.0, 0.0); N]),
            sample_rand_1d:Box::new([0.0;N]),
            random: SeedableRng::from_entropy(),
            jitter,
            now_1d:0,
            now_2d:0,
            now_rand_1d:0
        };
        a.init();
        a
    }
    fn init(&mut self) {
        self.init_1d();
        self.init_2d();
        let inv_x = 1.0 / self.x_samples as f32;
        let inv_y = 1.0 / self.y_samples as f32;
    }
    pub fn sample_2d(&mut self) -> Vec2 {
        match self.sample_2d.get(self.now_2d) {
            None => {
                self.init_2d();
                Vec2::new(0.5, 0.5)
            }
            Some((x, y)) => {
                Vec2::new(*x, *y)
            }
        }
    }
    pub fn sample_rand_1d(&mut self)->f32{
        match self.sample_rand_1d.get(self.now_rand_1d) {
            None => {
                self.init_2d();
                self.random.gen_range(0.0..1.0)
            }
            Some(value) => {
                *value
            }
        }
    }
    pub fn sample_1d(&mut self) -> f32 {
        match self.sample_1d.get(self.now_1d) {
            None => {
                self.init_1d();
                0.5
            }
            Some(x) => {
                *x
            }
        }
    }
    pub fn init_2d(&mut self) {
        let mut i = 0.0;
        let mut j = 0.0;
        let inv_x = 1.0 / self.x_samples as f32;
        let inv_y = 1.0 / self.y_samples as f32;
        let iter = self.sample_2d.iter_mut();
        for item in iter {
            let x = if self.jitter { rand::random() } else { 0.5 };
            let y = if self.jitter { rand::random() } else { 0.5 };
            let dx = (i + x) * inv_x.min(ONE_MINUS_EPSILON);
            let dy = (j + y) * inv_y.min(ONE_MINUS_EPSILON);
            j += 1.0;
            if j >= self.y_samples as f32 {
                i += 1.0;
                j = 0.0;
            }
            *item = (dx, dy);
        }
        self.now_2d=0;
    }
    //生成[0,1)的随机有序数列
    pub fn init_1d(&mut self) {
        let inv = 1.0 / (self.x_samples * self.y_samples) as f32;
        let iter = self.sample_1d.iter_mut();
        for (index, item) in iter.enumerate() {
            let det = if self.jitter { rand::random() } else { 0.5 };
            *item = ((index as f32 + det) * inv).min(ONE_MINUS_EPSILON);
        }
        self.now_1d=0;
    }
    pub fn init_rand_1d(&mut self) {
        let inv = 1.0 / (self.x_samples * self.y_samples) as f32;
        let iter = self.sample_1d.iter_mut();
        for (index, item) in iter.enumerate() {
            let det = if self.jitter { rand::random() } else { 0.5 };
            *item = ((index as f32 + det) * inv).min(ONE_MINUS_EPSILON);
        }
        self.now_rand_1d=0;
    }
}