use std::sync::Arc;

use crate::rendering::RTMaterial;
use raylib::prelude::*;
use crate::rendering::Ray;

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

pub struct Plane {
    position: Vector3,
    normal: Vector3,
    material: Arc<dyn RTMaterial>
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, material: Arc<dyn RTMaterial>) -> Plane {
        return Plane {
            position,
            normal,
            material
        }
    }
}

impl SceneObject for Plane {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let pos = ray_plane_intersection(ray, self.position, self.normal);
        return match pos {
            Some(hit) => Some(HitData::new(hit, self.normal, self.material())),
            None => None
        }
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material)
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


pub struct Quad {
    position: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    normal: Vector3,
    material: Arc<dyn RTMaterial>
}

impl Quad {
    pub fn new(position: Vector3, u: Vector3, v: Vector3, material: Arc<dyn RTMaterial>) -> Quad {
        //println!("Normal {}", u.cross(v));
        let n = u.cross(v);
        println!("Normal: ({}, {}, {})", n.normalized().x, n.normalized().y, n.normalized().z);
        return Quad {
            position,
            u,
            v,
            w: n/n.dot(n),
            normal: n.normalized(),
            material
        }
    }
}

impl SceneObject for Quad {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let hit = ray_plane_intersection(ray, self.position, self.normal);
        return match hit {
            Some(p) => {
                let plane_hit_point = p - self.position;
                let alpha = self.w.dot(plane_hit_point.cross(self.v));
                let beta = self.w.dot(self.u.cross(plane_hit_point));

                if alpha >= 0.0 && alpha <= 1.0 && beta >= 0.0 && beta <= 1.0 {
                    let norm = if ray.direction.dot(self.normal) > 0.0 {
                        self.normal.scale_by(-1.0)
                    } else {
                        self.normal
                    };

                    Some(HitData::new(p, norm, Arc::clone(&self.material)))
                } else {
                    None
                }
            },
            None => None
        }
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        Arc::clone(&self.material)
    }

    fn update(&self, _dt: f32) {
        todo!()
    }
}