use crate::rendering::Ray;
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

    pub fn update(&self) {
        for obj in &self.scene_objects {
            obj.update();
        }
    }
}

pub trait SceneObject {
    fn intersect(&self, ray: Ray) -> Vector3 {
        todo!()
    }

    fn update(&self);  
}