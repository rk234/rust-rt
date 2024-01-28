use crate::rendering;
use crate::rendering::{Framebuffer, RTMaterial, RayCamera};
use crate::scene::Scene;
use raylib::math::Vector3;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub const EPSILON: f32 = 0.0001f32;

pub struct Renderer<'a> {
    pub num_samples: u32,
    scene: &'a Scene,
    num_bounces: u32,
}

impl Renderer<'_> {
    pub fn new(scene: &Scene) -> Renderer {
        return Renderer {
            num_samples: 0,
            scene,
            num_bounces: 4,
        };
    }

    pub fn reset(&mut self) {
        self.num_samples = 0;
    }

    pub fn render_sample(
        &mut self,
        width: usize,
        height: usize,
        frame_buffer: &mut Framebuffer,
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

                *pixel += self.cast(ray, self.num_bounces as i32);
            });

        self.num_samples += 1;
    }

    fn cast(&self, ray: rendering::Ray, depth: i32) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0f32, 0f32, 0f32);
        }

        let hit = self.scene.intersect(&ray);
        match hit {
            Some(hit_data) => {
                let material = hit_data.material;
                let position = hit_data.position;
                let normal = hit_data.normal;

                let scatter = material.scatter(ray, position + (normal * EPSILON), normal);
                let attenuation = material.attenuation(position, normal);
                let emissive = material.emissive(position, normal);

                return match scatter {
                    Some(scatter_ray) => attenuation * self.cast(scatter_ray, depth - 1),
                    None => {
                        if emissive {
                            attenuation
                        } else {
                            Vector3::new(0f32, 0f32, 0f32)
                        }
                    }
                };
            }
            None => Vector3::new(0.0, 0.0, 0.0), //sky_color(ray)
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

fn sky_color(ray: rendering::Ray) -> Vector3 {
    let t = 0.5f32 * (ray.direction.y + 1.0f32);
    return Vector3::new(
        (1f32 - t) + (t * 138f32 / 255f32),
        (1f32 - t) + (t * 188f32 / 255f32),
        1f32,
    );
}
