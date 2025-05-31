use crate::rendering::{RTMaterial, Ray};
use raylib::math::Vector3;
use std::sync::Arc;

pub struct Scene {
    scene_objects: Vec<Box<dyn SceneObject>>,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            scene_objects: Vec::new(),
        };
    }

    pub fn add_object(&mut self, obj: Box<dyn SceneObject>) {
        self.scene_objects.push(obj);
    }

    pub fn update(&self, dt: f32) {
        for obj in &self.scene_objects {
            obj.update(dt);
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(&Box<dyn SceneObject>, HitData)> {
        let mut hit_data: Option<(&Box<dyn SceneObject>, HitData)> = None;
        let mut min_hit_dist: f32 = 10e9f32;
        for obj in &self.scene_objects {
            match obj.intersect(ray) {
                Some(data) => {
                    let dist = data.position.distance_to(ray.origin);
                    if dist < min_hit_dist {
                        hit_data = Some((obj, data));
                        min_hit_dist = dist;
                    }
                }
                None => continue,
            }
        }

        return hit_data;
    }
}

pub struct HitData {
    pub position: Vector3,
    pub normal: Vector3,
    pub bary: Vector3,
    pub node_hits: u32,
}

impl HitData {
    pub fn new(position: Vector3, normal: Vector3, bary: Vector3) -> HitData {
        HitData {
            position,
            normal,
            bary,
            node_hits: 0,
        }
    }
}

pub trait SceneObject: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<HitData>;
    fn material(&self) -> Arc<dyn RTMaterial>;
    fn update(&self, dt: f32);
}
