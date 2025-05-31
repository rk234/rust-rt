use std::f32::EPSILON;

use crate::rendering::Ray;
use raylib::{math::Vector2, math::Vector3};

pub struct Triangle {
    pub verts: [Vector3; 3],
    pub normals: Option<[Vector3; 3]>,
    pub uvs: Option<[Vector2; 3]>,
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            verts: self.verts,
            normals: self.normals,
            uvs: self.uvs,
        }
    }
}

pub struct TriangleHitData {
    pub p: Vector3,
    pub normal: Vector3,
    pub bary: Vector3,
}

impl TriangleHitData {
    pub fn new(p: Vector3, normal: Vector3, bary: Vector3) -> TriangleHitData {
        return TriangleHitData { p, normal, bary };
    }
}

impl Triangle {
    pub fn new(verts: [Vector3; 3], normals: [Vector3; 3], uvs: [Vector2; 3]) -> Triangle {
        return Triangle {
            verts,
            normals: Some(normals),
            uvs: Some(uvs),
        };
    }

    pub fn centroid(&self) -> Vector3 {
        let mut sum = Vector3::zero();

        self.verts.map(|v| {
            sum += v;
        });

        return sum / 3.0;
    }

    pub fn intersect(&self, ray: &Ray) -> Option<TriangleHitData> {
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

        Some(TriangleHitData::new(
            ray.origin + ray.direction.scale_by(t),
            normal,
            Vector3::new(u, v, 1f32 - u - v),
        ))
    }
}
