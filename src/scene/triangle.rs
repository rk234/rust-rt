use std::sync::Arc;

use crate::rendering::RTMaterial;
use raylib::ffi::Vector2;
use raylib::math::Vector3;

pub struct Triangle {
    pub verts: [Vector3; 3],
    pub normals: [Vector3; 3],
    pub uvs: [Vector2; 3],
    pub material: Arc<dyn RTMaterial>,
}

impl Triangle {
    pub fn centroid(&self) -> Vector3 {}
}
