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
}

pub struct HitData<'a> {
    pub position: Vector3,
    pub normal: Vector3,
    pub material: &'a RTMaterial
}

impl HitData<'_> {
    pub fn new(position: Vector3, normal: Vector3, material: &RTMaterial) -> HitData {
        HitData { position, normal, material }
    }
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<HitData>;
    fn material(&self) -> &RTMaterial;
    fn update(&self, dt: f32);
}

pub struct Sphere<'a> {
    pub position: Vector3,
    pub radius: f32,
    pub material: &'a RTMaterial
}

impl SceneObject for Sphere<'_> {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let l = ray.origin - self.position;
        let a = 1f32;
        let b = 2f32 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius*self.radius;
        let disc = b*b - 4f32*a*c;

        if disc > 0f32 {
            let t = (-b+disc.sqrt()) / (2f32*a);
            
            if t > 0f32 {
                Some(HitData::new(ray.at(t), (ray.at(t)-self.position).normalized(), self.material()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> &RTMaterial {
        return self.material;
    }

    fn update(&self, dt: f32) {
        //todo!()
    }
}