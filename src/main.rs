use std::{io::{BufRead, BufReader}, fs::File};

use lib::{tool::{scene::Sence, log_init}, integrator::path::PathIntegrator};
use obj::ObjData;


mod lib;
fn main() {
    // log_init();
    // let sence = Sence::rand_read(0, 1000);
    // // let sence = Sence::read("F:/Dept/ray_track_weekend_rs/pbrt/sence/template.json");
    // let path = PathIntegrator::<16>::default();
    // path.render(&sence,"main.png")
    let input =BufReader::new(File::open("D:\\Dept\\pbrt\\object\\box.obj").unwrap());
    let model= ObjData::load_buf(input).unwrap();
    println!("{:?}",model)

    
}
