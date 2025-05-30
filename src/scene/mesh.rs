use std::{fs, sync::Arc};

use raylib::math::Vector3;
use wavefront_obj::obj;

use crate::{math::Transform, rendering::RTMaterial};

use super::{HitData, SceneObject, Triangle};

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub transform: Transform,
    pub material: Arc<dyn RTMaterial>,
}

impl Mesh {
    pub fn new(transform: Transform, material: Arc<dyn RTMaterial>) -> Self {
        Self {
            tris: vec![],
            transform,
            material,
        }
    }

    pub fn load_obj(&mut self, path: &str) {
        if let Ok(file) = fs::read_to_string(path) {
            match obj::parse(file) {
                Ok(res) => {
                    if let Some(obj) = res.objects.get(0) {
                        println!("Loading {} from obj file at {path}", obj.name);
                        self.tris = obj
                            .geometry
                            .iter()
                            .flat_map(|g| {
                                g.shapes.iter().filter_map(|s| match s.primitive {
                                    obj::Primitive::Triangle(
                                        (v0, _, _),
                                        (v1, _, _),
                                        (v2, _, _),
                                    ) => {
                                        let v0 = obj.vertices[v0];
                                        let v1 = obj.vertices[v1];
                                        let v2 = obj.vertices[v2];
                                        Some(Triangle {
                                            uvs: None,
                                            normals: None,
                                            verts: [
                                                Vector3::new(v0.x as f32, v0.y as f32, v0.z as f32),
                                                Vector3::new(v1.x as f32, v1.y as f32, v1.z as f32),
                                                Vector3::new(v2.x as f32, v2.y as f32, v2.z as f32),
                                            ],
                                        })
                                    }
                                    _ => None,
                                })
                            })
                            .collect()
                    }
                }
                Err(err) => {
                    println!("Failed to parse: {err}")
                }
            }
        }
    }
}

impl SceneObject for Mesh {
    fn intersect(&self, ray: &crate::rendering::Ray) -> Option<super::HitData> {
        for tri in &self.tris {
            if let Some(hit) = tri.intersect(&ray.transform(&self.transform)) {
                return Some(HitData::new(
                    hit.p.transform_with(self.transform.m),
                    hit.normal.transform_with(self.transform.inv.transposed()),
                    hit.bary,
                    Arc::clone(&self.material),
                ));
            }
        }
        None
    }

    fn material(&self) -> Arc<dyn RTMaterial> {
        return Arc::clone(&self.material);
    }

    fn update(&self, _: f32) {
        todo!()
    }
}
