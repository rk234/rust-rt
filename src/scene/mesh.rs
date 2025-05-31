use std::{fs, sync::Arc};

use raylib::math::{Vector2, Vector3};
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
                                        (v0, t0, n0),
                                        (v1, t1, n1),
                                        (v2, t2, n2),
                                    ) => {
                                        let v0 = obj.vertices[v0];
                                        let v1 = obj.vertices[v1];
                                        let v2 = obj.vertices[v2];

                                        let mut tri = Triangle {
                                            uvs: None,
                                            normals: None,
                                            verts: [
                                                Vector3::new(v0.x as f32, v0.y as f32, v0.z as f32),
                                                Vector3::new(v1.x as f32, v1.y as f32, v1.z as f32),
                                                Vector3::new(v2.x as f32, v2.y as f32, v2.z as f32),
                                            ],
                                        };

                                        match (t0, t1, t2) {
                                            (Some(t0), Some(t1), Some(t2)) => {
                                                let t0 = obj.tex_vertices[t0];
                                                let t1 = obj.tex_vertices[t1];
                                                let t2 = obj.tex_vertices[t2];
                                                tri.uvs = Some([
                                                    Vector3::new(
                                                        t0.u as f32,
                                                        t0.v as f32,
                                                        t0.w as f32,
                                                    ),
                                                    Vector3::new(
                                                        t1.u as f32,
                                                        t1.v as f32,
                                                        t1.w as f32,
                                                    ),
                                                    Vector3::new(
                                                        t2.u as f32,
                                                        t2.v as f32,
                                                        t2.w as f32,
                                                    ),
                                                ])
                                            }
                                            _ => {}
                                        }

                                        match (n0, n1, n2) {
                                            (Some(n0), Some(n1), Some(n2)) => {
                                                let n0 = obj.normals[n0];
                                                let n1 = obj.normals[n1];
                                                let n2 = obj.normals[n2];
                                                tri.normals = Some([
                                                    Vector3::new(
                                                        n0.x as f32,
                                                        n0.y as f32,
                                                        n0.z as f32,
                                                    ),
                                                    Vector3::new(
                                                        n1.x as f32,
                                                        n1.y as f32,
                                                        n1.z as f32,
                                                    ),
                                                    Vector3::new(
                                                        n2.x as f32,
                                                        n2.y as f32,
                                                        n2.z as f32,
                                                    ),
                                                ])
                                            }
                                            _ => {}
                                        }

                                        Some(tri)
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
