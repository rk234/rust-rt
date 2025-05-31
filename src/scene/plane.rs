use crate::rendering::{RTMaterial, Ray};
use crate::scene::{HitData, SceneObject};
use raylib::math::Vector3;
use std::sync::Arc;

pub struct Plane {
    position: Vector3,
    normal: Vector3,
    material: Arc<dyn RTMaterial>,
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, material: Arc<dyn RTMaterial>) -> Plane {
        return Plane {
            position,
            normal,
            material,
        };
    }
}

impl SceneObject for Plane {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let pos = ray_plane_intersection(ray, self.position, self.normal);
        return match pos {
            Some(hit) => Some(HitData::new(hit, self.normal, Vector3::zero())),
            None => None,
        };
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material);
    }

    fn update(&self, _dt: f32) {}
}

pub fn ray_plane_intersection(ray: &Ray, position: Vector3, normal: Vector3) -> Option<Vector3> {
    let denom = normal.dot(ray.direction);
    if denom.abs() > 1e-6 {
        let p0l0 = position - ray.origin;
        let t = p0l0.dot(normal) / denom;
        if t > 0f32 {
            return Some(ray.at(t));
        }
    }

    return None;
}
