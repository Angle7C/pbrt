use glam::{Vec2, Vec3};

pub struct Triangle {
    point: [Vec3; 3],
    normal: [Vec3; 3],
    uv: [Vec2; 3],
}
impl Triangle {
    pub fn new(point: [Vec3; 3], normal: [Vec3; 3], uv: [Vec2; 3]) -> Self {
        Self { point, normal, uv }
    }
}
