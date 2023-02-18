use glam::Vec2;

use crate::lib::tool::interaction::Interaction;

pub struct UVMapping {
    su: f32,
    sv: f32,
    du: f32,
    dv: f32,
}
impl UVMapping {
    pub fn new(su: f32, sv: f32, du: f32, dv: f32) -> Self {
        Self { su, sv, du, dv }
    }
    pub fn map(&self, inter: Interaction) -> (Vec2, Vec2, Vec2) {
        let dstdx = Vec2::new(inter.dxy.dudx * self.su, inter.dxy.dvdx * self.sv);
        let dstdy = Vec2::new(inter.dxy.dudy * self.su, inter.dxy.dvdy * self.sv);
        let uv=Vec2::new(
            inter.duv.uv.x * self.su + self.du,
            inter.duv.uv.y * self.sv + self.dv,
        );
        (dstdx,dstdy,uv)
    }
}
