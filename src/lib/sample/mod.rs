use glam::Vec2;

pub mod stratified;

const ONE_MINUS_EPSILON: f32 = 0.99999999;
#[allow(unused)]

pub struct PixelSample {
    index_1d: usize,
    index_2d: usize,
    //像素左下角
    start: Vec2,
}
#[allow(unused)]
impl PixelSample {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            index_1d: 0,
            index_2d: 0,
            start:Vec2::new(x,y),
        }
    }
    pub fn sample_1d<const N:usize>(&mut self,sample:&mut Sample<N>)->f32{
        sample.sample_1d()
    }   
    pub fn sample_2d<const N:usize>(&mut self,sample:&mut Sample<N>)->Vec2{
        sample.sample_2d()+self.start
    }
}
impl Default for PixelSample {
    fn default() -> Self {
        Self {
            index_1d: 0,
            index_2d: 0,
            start: Vec2::ZERO,
        }
    }
}
#[allow(unused)]
pub enum Sample<const N: usize> {
    StratifiedSampler(stratified::StratifiedSampler<N>),
}
impl<const N: usize> Clone for Sample<N>{
     fn clone(&self) -> Self {
        match self {
            Sample::StratifiedSampler(stratified)=>{Sample::StratifiedSampler(stratified.clone())}
        }
    }
}
#[allow(unused)]
impl<const N:usize> Sample<N>{
    pub fn sample_1d(&mut self)->f32{
        match self {
            Sample::StratifiedSampler(s)=>s.sample_1d()
        }
    }
    pub fn sample_2d(&mut self)->Vec2{
        match self {
            Sample::StratifiedSampler(s)=>s.sample_2d()
        }
    }
    pub fn sample_rand_1d(&mut self)->f32{
        match self {
            Sample::StratifiedSampler(s)=>s.sample_rand_1d()
        }
    }
}
