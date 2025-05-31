use crate::rendering;
use crate::rendering::{Framebuffer, RTMaterial, RayCamera};
use crate::scene::models::Scene;
use raylib::math::Vector3;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub const EPSILON: f32 = 0.0001f32;

pub struct Renderer<'a> {
    pub num_samples: u32,
    scene: &'a Scene,
    num_bounces: i32,
}

impl Renderer<'_> {
    pub fn new(scene: &Scene) -> Renderer {
        return Renderer {
            num_samples: 0,
            scene,
            num_bounces: 10,
        };
    }

    pub fn reset(&mut self) {
        self.num_samples = 0;
    }

    pub fn render_normals(
        &mut self,
        width: usize,
        height: usize,
        normal_buffer: &mut Framebuffer,
        camera: &mut RayCamera,
    ) {
        camera.update_viewport(width, height);

        normal_buffer
            .data
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, normal)| {
                let x = i % width;
                let y = i / width;
                let ray = camera.gen_primary_ray(x, y, width, height);

                let hit_opt = self.scene.intersect(&ray);
                match hit_opt {
                    Some((_, hit)) => {
                        *normal = hit.normal;
                    }
                    None => *normal = Vector3::new(0.0, 0.0, 0.0),
                }
            })
    }
    pub fn render_bvh_hits(
        &mut self,
        width: usize,
        height: usize,
        hit_buffer: &mut Framebuffer,
        camera: &mut RayCamera,
    ) {
        camera.update_viewport(width, height);

        hit_buffer
            .data
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, normal)| {
                let x = i % width;
                let y = i / width;
                let ray = camera.gen_primary_ray(x, y, width, height);

                let hit_opt = self.scene.intersect(&ray);
                match hit_opt {
                    Some((_, hit)) => {
                        let s = hit.node_hits as f32 / 10f32;
                        let hits = Vector3::new(s, s, s);
                        *normal = hits;
                    }
                    None => *normal = Vector3::new(0.0, 0.0, 0.0),
                }
            })
    }

    pub fn render_object_mask(
        &mut self,
        width: usize,
        height: usize,
        mask_buffer: &mut Framebuffer,
        camera: &mut RayCamera,
    ) {
        todo!()
    }

    pub fn render_sample(
        &mut self,
        width: usize,
        height: usize,
        frame_buffer: &mut Framebuffer,
        //snormal_buffer: &mut Framebuffer,
        camera: &mut RayCamera,
    ) {
        camera.update_viewport(width, height);

        frame_buffer
            .data
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let x = i % width;
                let y = i / width;
                let ray = camera.gen_primary_ray(x, y, width, height);

                *pixel += self.cast_iter(ray, self.num_bounces as i32);
            });

        self.num_samples += 1;
    }

    fn cast_iter(&self, ray: rendering::Ray, depth: i32) -> Vector3 {
        let mut result = Vector3::new(1f32, 1f32, 1f32);
        let mut current_ray = ray;

        for _ in 0..depth {
            let hit = self.scene.intersect(&current_ray);
            match hit {
                Some((obj, hit_data)) => {
                    let material = obj.material();
                    let position = hit_data.position;
                    let normal = hit_data.normal;

                    let scatter =
                        material.scatter(current_ray, position + (normal * EPSILON), normal);
                    let attenuation = material.attenuation(position, normal);
                    let emissive = material.emissive(position, normal);

                    match scatter {
                        Some(scatter_ray) => {
                            result *= attenuation;
                            current_ray.origin = scatter_ray.origin;
                            current_ray.direction = scatter_ray.direction;
                        }
                        None => {
                            if emissive {
                                return result * attenuation;
                            } else {
                                return Vector3::new(0f32, 0f32, 0f32);
                            }
                        }
                    }
                }
                None => return result * sky_color(&current_ray), // None => Vector3::new(0.0, 0.0, 0.0), //sky_color(ray)
            }
        }

        return Vector3::new(0f32, 0f32, 0f32);
    }

    fn cast(&self, ray: rendering::Ray, depth: i32) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0f32, 0f32, 0f32);
        }

        let hit = self.scene.intersect(&ray);
        match hit {
            Some((obj, hit_data)) => {
                let material = obj.material();
                let position = hit_data.position;
                let normal = hit_data.normal;

                let scatter = material.scatter(ray, position + (normal * EPSILON), normal);
                let attenuation = material.attenuation(position, normal);
                let emissive = material.emissive(position, normal);

                match scatter {
                    Some(scatter_ray) => attenuation * self.cast(scatter_ray, depth - 1),
                    None => {
                        if emissive {
                            attenuation
                        } else {
                            Vector3::new(0f32, 0f32, 0f32)
                        }
                    }
                }
            }
            None => sky_color(&ray), // None => Vector3::new(0.0, 0.0, 0.0), //sky_color(ray)
        }
    }

    pub fn render_full(
        &mut self,
        width: usize,
        height: usize,
        frame_buffer: &mut Framebuffer,
        samples: u32,
        camera: &mut RayCamera,
    ) {
        frame_buffer.clear();

        for _ in 0..samples {
            self.render_sample(width, height, frame_buffer, camera);
        }

        frame_buffer.normalize(self.num_samples as f32);

        self.num_samples = 0;
    }
}

fn sky_color(ray: &rendering::Ray) -> Vector3 {
    let t = 0.5f32 * (ray.direction.y + 1.0f32);
    return Vector3::new(
        (1f32 - t) + (t * 138f32 / 255f32),
        (1f32 - t) + (t * 188f32 / 255f32),
        1f32,
    );
}
