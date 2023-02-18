use std::sync::atomic::{ AtomicU32};


use image::RgbImage;

static  ATOM_COUNT:AtomicU32=AtomicU32::new(0);
pub struct Film {
    x_index:u32,
    y_index:u32,
    size:(u32,u32)
}
// unsafe impl Sync for Film{}
unsafe impl  Sync for Film {}
unsafe impl  Send for Film {}
#[allow(unused)]
impl  Film  {
    const BLOCK_SIZE:(u32,u32)=(16,16);
    pub fn new(image:& RgbImage) -> Self {
        let (x_size,y_size)=image.dimensions();
        let  x_index=x_size/Self::BLOCK_SIZE.0+ if x_size%Self::BLOCK_SIZE.0!=0{1}else{0};
        let  y_index=y_size/Self::BLOCK_SIZE.1+ if y_size%Self::BLOCK_SIZE.1!=0{1}else{0};
        Film {
            x_index,
            y_index,
            size:(x_size,y_size)
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        self.size
    }
    pub fn iter(& self) -> Option<FilmIter> {
        let index=ATOM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Release);
        if index>=self.x_index*self.y_index {return None}
        let x=index/self.x_index;
        let y=index%self.y_index;

        let left_up=(Self::BLOCK_SIZE.0*x,Self::BLOCK_SIZE.1*y);
        let right_down=(Self::BLOCK_SIZE.0*(x+1),Self::BLOCK_SIZE.1*(y+1));
        Some(FilmIter::new(left_up, right_down, Self::BLOCK_SIZE))
    }
    pub fn reset(){
        ATOM_COUNT.store(0, std::sync::atomic::Ordering::Release)
    }
    pub fn size(&self)->u64{
        (self.x_index*self.y_index) as  u64
    }
}
pub struct FilmIter {
    pub block_size:(u32,u32),
    pub left_up: (u32, u32),
    pub right_down:(u32,u32),
    pub now:(u32,u32),

}
#[allow(unused)]

impl FilmIter {
    pub fn new(left_up:(u32,u32),right_down:(u32,u32),block_size:(u32,u32)) -> Self {
        Self {
            left_up:left_up,
            right_down:right_down,
            block_size:block_size,
            now:left_up
        }
    }
    pub fn size(&self)->u64{
        let (a,b) = self.block_size;
        (a*b) as u64
    }
}
impl Iterator for FilmIter {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let (mut x,mut y)=self.now;
        
        if y>=self.right_down.1{
            y=0;
            x+=1;
        };
        if x>=self.right_down.0{
            None
        }else{
            self.now=(x,y+1);
            Some((x,y))
        }
    }
    
}
