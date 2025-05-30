use crate::rendering::{RTMaterial, Ray};
use crate::scene::{HitData, SceneObject};
use raylib::math::Vector3;
use std::sync::Arc;

pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
    pub material: Arc<dyn RTMaterial>,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f32, material: Arc<dyn RTMaterial>) -> Sphere {
        return Sphere {
            position,
            radius,
            material,
        };
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let l = ray.origin - self.position;
        let a = 1f32;
        let b = 2f32 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius * self.radius;
        let disc = b * b - 4f32 * a * c;

        if disc > 0f32 {
            let t0 = (-b + disc.sqrt()) / (2f32 * a);
            let t1 = (-b - disc.sqrt()) / (2f32 * a);
            let t = if t0 < t1 { t0 } else { t1 };
            if t > 0f32 {
                Some(HitData::new(
                    ray.at(t),
                    (ray.at(t) - self.position).normalized(),
                    Vector3::zero(),
                    self.material(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material);
    }

    fn update(&self, _: f32) {
        //todo!()
    }
}
