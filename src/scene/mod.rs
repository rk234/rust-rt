use std::sync::Arc;

use crate::rendering::{Ray, RTMaterial};
use raylib::prelude::*;

pub struct Scene {
    scene_objects: Vec<Box<dyn SceneObject>>
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            scene_objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn SceneObject>) {
        self.scene_objects.push(obj);
    }

    pub fn update(&self, dt: f32) {
        for obj in &self.scene_objects {
            obj.update(dt);
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let mut hit_data: Option<HitData> = None;
        let mut min_hit_dist: f32 = 10e9f32;
        for obj in &self.scene_objects {
            match obj.intersect(ray) {
                Some(data) => {
                    let dist = data.position.distance_to(ray.origin);
                    if dist < min_hit_dist {
                        hit_data = Some(data);
                        min_hit_dist = dist;
                    }
                }
                None => continue
            }
        }

        return hit_data;
    }
}

pub struct HitData {
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Arc<dyn RTMaterial>
}

impl HitData {
    pub fn new(position: Vector3, normal: Vector3, material: Arc<dyn RTMaterial>) -> HitData {
        HitData { position, normal, material }
    }
}

pub trait SceneObject: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<HitData>;
    fn material(&self) -> Arc<dyn RTMaterial>;
    fn update(&self, dt: f32);
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
    pub material: Arc<dyn RTMaterial>
}

impl Sphere {
    pub fn new(position: Vector3, radius: f32, material: Arc<dyn RTMaterial>) -> Sphere {
        return Sphere {
            position,
            radius,
            material
        }
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let l = ray.origin - self.position;
        let a = 1f32;
        let b = 2f32 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius*self.radius;
        let disc = b*b - 4f32*a*c;

        if disc > 0f32 {
            let t0 = (-b+disc.sqrt()) / (2f32*a);
            let t1 = (-b-disc.sqrt()) / (2f32*a);
            let t = if t0 < t1 {t0} else {t1};
            if t > 0f32 {
                Some(HitData::new(ray.at(t), (ray.at(t)-self.position).normalized(), self.material()))
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

struct Plane {
    postion: Vector3,
    normal: Vector3
}

impl Plane {
}