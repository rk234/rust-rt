use raylib::math::Vector3;
use std::ops::Add;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        return Self {
            origin,
            direction,
        };
    }

    pub fn at(&self, t: f32) -> Vector3 {
        return self.origin.add(self.direction.scale_by(t));
    }
}
