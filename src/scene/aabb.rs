use raylib::math::Vector3;

use crate::rendering::Ray;

pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn new() -> AABB {
        return AABB {
            min: Vector3::zero(),
            max: Vector3::zero(),
        };
    }

    pub fn from_bounds(min: Vector3, max: Vector3) -> AABB {
        return AABB { min, max };
    }

    pub fn include(&mut self, p: Vector3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }

    pub fn intersect(&self, ray: Ray) -> bool {
        false
    }
}
