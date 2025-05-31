use raylib::math::Vector3;

use crate::rendering::Ray;

use super::Triangle;

pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl Clone for AABB {
    fn clone(&self) -> Self {
        AABB {
            min: self.min,
            max: self.max,
        }
    }
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

    pub fn from_tris(tris: &[Triangle]) -> AABB {
        let mut aabb = AABB::new();

        tris.iter().for_each(|t| {
            aabb.include(t.verts[0]);
            aabb.include(t.verts[1]);
            aabb.include(t.verts[2]);
        });

        aabb
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let tx1 = (self.min.x - ray.origin.x) / ray.direction.x;
        let tx2 = (self.max.x - ray.origin.x) / ray.direction.x;
        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);

        let ty1 = (self.min.y - ray.origin.y) / ray.direction.y;
        let ty2 = (self.max.y - ray.origin.y) / ray.direction.y;

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));

        let tz1 = (self.min.z - ray.origin.z) / ray.direction.z;
        let tz2 = (self.max.z - ray.origin.z) / ray.direction.z;

        tmin = tmin.max(ty1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));

        tmax >= tmin && tmin < 1e30f32 && tmax > 0f32
    }
}
