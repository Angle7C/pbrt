use std::ops::Mul;

use glam::{Mat4, Vec3};

use crate::lib::ray::Ray;

use super::{bound::Bound3, Point3};
#[derive(Clone, Copy,Debug)]
pub struct Tranform {
    pub m: Mat4,
    pub inv_m: Mat4,
}
impl Default for Tranform {
    fn default() -> Self {
        Self {
            m: Mat4::IDENTITY,
            inv_m: Mat4::IDENTITY,
        }
    }
}
/// Tranform变换
#[allow(unused)]
impl Tranform {
    pub fn tranform_point3(&self, p: Point3) -> Point3 {
        self.m.transform_point3(p)
    }
    pub fn tranform_vector3(&self, vector: Vec3) -> Vec3 {
        self.m.transform_vector3(vector)
    }
    pub fn tranform_normal(&self, normal: Vec3) -> Vec3 {
        self.inv_m.transpose().transform_vector3(normal)
    }
    pub fn tranform_ray(&self, ray: &Ray) -> Ray {
        let origin = self.m.transform_point3(ray.origin);
        let dir = self.m.transform_vector3(ray.dir);
        Ray::new(origin, dir)
    }
    pub fn tranform_bound3(&self, bound3: &Bound3) -> Bound3 {
        let min = self.m.transform_point3(bound3.min);
        let max = self.m.transform_point3(bound3.max);
        Bound3::new(min, max)
    }
    pub fn inverse(&self) -> Self {
        Self {
            m: self.inv_m,
            inv_m: self.m,
        }
    }
    pub fn transpose(&self) -> Self {
        Self {
            m: self.m.transpose(),
            inv_m: self.inv_m.transpose(),
        }
    }
}
impl Mul<Tranform> for Tranform {
    type Output = Tranform;
    fn mul(self, rhs: Tranform) -> Self::Output {
        let m = self.m * rhs.m;
        let inv_m = rhs.inv_m * self.inv_m;
        Tranform::new(m, inv_m)
    }
}
/// 创建Tranfrom
#[allow(unused)]
impl Tranform {
    pub fn new(m: Mat4, inv_m: Mat4) -> Self {
        Self {  m, inv_m }
    }
    pub fn from_mat4(m: Mat4) -> Self {
        Self {
        m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_translation(det: Vec3) -> Self {
        let m = Mat4::from_translation(det);
        Self {
            m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        let m = Mat4::from_scale(scale);
        Self {
             m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let m = Mat4::from_rotation_x(angle.to_radians());
        Self {
             m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let m = Mat4::from_rotation_y(angle.to_radians());
        Self {
            m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let m = Mat4::from_rotation_z(angle.to_radians());
        Self {
             m,
            inv_m: m.inverse(),
        }
    }

    pub fn from_rotation_axis(axis: &Vec3, angle: f32) -> Self {
        let m = Mat4::from_axis_angle(*axis, angle);
        Self {
             m,
            inv_m: m.inverse(),
        }
    }

    pub fn look(pos: Vec3, look: Vec3, up: Vec3) -> Self {
        let dir = (look - pos).normalize();
        let right = (up.normalize().cross(dir)).normalize();
        let up = dir.cross(right);
        let m = Mat4::from_cols(
            right.extend(0.0),
            up.extend(0.0),
            dir.extend(0.0),
            pos.extend(1.0),
        );
        Self {
             m,
            inv_m: m.inverse(),
        }
    }
}
/// Tranfrom 常量
#[allow(unused)]
impl Tranform {
    pub const IDENTITY: Tranform = Self {
        m: Mat4::IDENTITY,
        inv_m: Mat4::IDENTITY,
    };
}
