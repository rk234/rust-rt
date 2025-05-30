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
        let normal = edge1.cross(edge2).normalized();
        let pvec = ray.direction.cross(edge2);
        let det = edge1.dot(pvec);

        if det < EPSILON {
            return None;
        }

        let inv_det = 1f32 / det;
        let tvec = ray.origin - self.verts[0];
        let u = tvec.dot(pvec) * inv_det;

        if u < 0f32 || u > 1f32 {
            return None;
        }

        let qvec = tvec.cross(edge1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0f32 || u + v > 1f32 {
            return None;
        }

        let t = edge2.dot(qvec) * inv_det;

        if t < 0f32 {
            return None;
        }

        Some(HitData::new(
            ray.origin + ray.direction.scale_by(t),
            normal,
            Vector3::new(u, v, 1f32 - u - v),
            Arc::clone(&self.material),
        ))
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material);
    }

    fn update(&self, _: f32) {}
}
