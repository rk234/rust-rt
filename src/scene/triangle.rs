use std::{f32::EPSILON, sync::Arc};

use crate::rendering::{RTMaterial, Ray};
// use raylib::ffi::Vector2;
use raylib::math::Vector3;

use super::{HitData, SceneObject};

pub struct Triangle {
    pub verts: [Vector3; 3],
    // pub normals: [Vector3; 3],
    // pub uvs: [Vector2; 3],
    pub material: Arc<dyn RTMaterial>,
}

impl Triangle {
    pub fn new(verts: [Vector3; 3], material: Arc<dyn RTMaterial>) -> Triangle {
        return Triangle { verts, material };
    }

    pub fn centroid(&self) -> Vector3 {
        let mut sum = Vector3::zero();

        self.verts.map(|v| {
            sum += v;
        });

        return sum / 3.0;
    }
}

impl SceneObject for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let edge1 = self.verts[1] - self.verts[0];
        let edge2 = self.verts[2] - self.verts[0];

        let ray_cross_e2 = ray.direction.cross(edge2);
        let det = edge1.dot(ray_cross_e2);

        if det > -EPSILON && det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.verts[0];
        let u = inv_det * s.dot(ray_cross_e2);

        if (u < 0.0 && u.abs() > EPSILON) || ((u > 1.0) && (u - 1.0).abs() > EPSILON) {
            return None;
        }

        let s_cross_e1 = s.cross(edge1);
        let v = inv_det * ray.direction.dot(s_cross_e1);
        if (v < 0.0 && v.abs() > EPSILON) || (u + v > 1.0 && (u + v - 1.0).abs() > EPSILON) {
            return None;
        }

        let t = inv_det * edge2.dot(s_cross_e1);

        if t > EPSILON {
            let p = ray.origin + ray.direction.scale_by(t);
            let n = edge1.cross(edge2).normalized();
            return Some(HitData::new(p, n, Arc::clone(&self.material)));
        } else {
            return None;
        }
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material);
    }

    fn update(&self, _: f32) {}
}
