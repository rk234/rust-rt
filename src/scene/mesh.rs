use std::{fs, sync::Arc};

use raylib::math::Vector3;
use wavefront_obj::obj;

use crate::{
    math::Transform,
    rendering::{RTMaterial, Ray},
};

use super::{
    bvh::{self, BVH},
    HitData, SceneObject, Triangle,
};

pub struct Mesh {
    bvh: BVH,
    transform: Transform,
    material: Arc<dyn RTMaterial>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>, transform: Transform, material: Arc<dyn RTMaterial>) -> Self {
        Self {
            transform,
            material,
            bvh: BVH::new(tris),
        }
    }

    pub fn from_obj(path: &str, transform: Transform, material: Arc<dyn RTMaterial>) -> Self {
        if let Ok(file) = fs::read_to_string(path) {
            match obj::parse(file) {
                Ok(res) => {
                    if let Some(obj) = res.objects.get(0) {
                        println!("Loading {} from obj file at {path}", obj.name);
                        let tris: Vec<Triangle> = obj
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
                            .collect();

                        let mut mesh = Mesh {
                            bvh: BVH::new(tris),
                            transform,
                            material,
                        };
                        mesh.bvh.build();

                        return mesh;
                    }
                }
                Err(err) => {
                    println!("Failed to parse: {err}")
                }
            }
        }
        return Mesh {
            bvh: BVH::new(Vec::new()),
            transform,
            material,
        };
    }
}

impl SceneObject for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<super::HitData> {
        let t_ray = ray.transform(&self.transform);

        if let Some(hit) = self.bvh.intersect(&t_ray) {
            return Some(HitData::new(
                hit.position.transform_with(self.transform.m),
                hit.normal.transform_with(self.transform.inv.transposed()),
                hit.bary,
            ));
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
