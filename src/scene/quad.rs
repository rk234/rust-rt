use crate::rendering::{RTMaterial, Ray};
use crate::scene::{plane, HitData, SceneObject};
use raylib::math::Vector3;
use std::sync::Arc;

pub struct Quad {
    position: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    normal: Vector3,
    material: Arc<dyn RTMaterial>,
}

impl Quad {
    pub fn new(position: Vector3, u: Vector3, v: Vector3, material: Arc<dyn RTMaterial>) -> Quad {
        //println!("Normal {}", u.cross(v));
        let n = u.cross(v);
        println!(
            "Normal: ({}, {}, {})",
            n.normalized().x,
            n.normalized().y,
            n.normalized().z
        );
        return Quad {
            position,
            u,
            v,
            w: n / n.dot(n),
            normal: n.normalized(),
            material,
        };
    }
}

impl SceneObject for Quad {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let hit = plane::ray_plane_intersection(ray, self.position, self.normal);
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
            }
            None => None,
        };
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        Arc::clone(&self.material)
    }

    fn update(&self, _dt: f32) {
        todo!()
    }
}
