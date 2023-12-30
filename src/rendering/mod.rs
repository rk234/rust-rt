use crate::scene::Scene;
use rand::Rng;
use raylib::prelude::*;
use std::{ops::Add, time::Instant};

pub const EPSILON: f32 = 0.00001f32;

pub struct RayCamera {
    position: Vector3,
    direction: Vector3,
    pitch: f32,
    yaw: f32,
    near_plane: f32,
    viewport_size: Vector3,
}

impl RayCamera {
    pub fn new(position: Vector3) -> RayCamera {
        RayCamera {
            position,
            direction: Vector3::new(0f32, 0f32, 1f32),
            pitch: 0f32,
            yaw: 0f32,
            near_plane: 1f32,
            viewport_size: Vector3::new(2f32 * 16f32 / 9f32, 2f32, 0f32),
        }
    }

    pub fn update_viewport(&mut self, screen_width: usize, screen_height: usize) {
        self.viewport_size = Vector3::new(
            (screen_width as f32 / screen_height as f32) * 1.5f32,
            1.5f32,
            0f32,
        )
    }

    pub fn gen_primary_ray(
        &self,
        screen_x: usize,
        screen_y: usize,
        screen_width: usize,
        screen_height: usize,
    ) -> Ray {
        let mut rng = rand::thread_rng();

        let adjacent = Vector3::new(0f32, 1f32, 0f32)
            .cross(self.direction)
            .normalized()
            .scale_by(
                (self.viewport_size.x)
                    * (screen_x as f32 + rng.gen_range(-0.5f32..0.5f32) / screen_width as f32),
            );
        let local_up = adjacent.cross(self.direction).normalized().scale_by(
            (self.viewport_size.y)
                * (screen_y as f32 + rng.gen_range(-0.5f32..0.5f32) / screen_height as f32),
        );

        let bottom_left = adjacent
            .scale_by(-self.viewport_size.x as f32 / 2f32)
            .add(local_up.scale_by(-self.viewport_size.y as f32 / 2f32));
        let norm_dir = self.direction.scale_by(self.near_plane);

        let dir = Vector3::new(
            bottom_left.x + adjacent.x + local_up.x + norm_dir.x,
            bottom_left.y + adjacent.y + local_up.y + norm_dir.y,
            bottom_left.z + adjacent.z + local_up.z + norm_dir.z,
        )
        .normalized();

        return Ray::new(self.position, dir);
    }
}

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        return Self {
            origin,
            direction: direction.normalized(),
        };
    }

    pub fn at(&self, t: f32) -> Vector3 {
        return self.origin.add(self.direction.scale_by(t));
    }
}

pub struct Framebuffer {
    data: Vec<Vector3>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![Vector3::new(0f32, 0f32, 0f32); width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y * self.width] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vector3 {
        self.data[x + y * self.width]
    }

    pub fn accum_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y * self.width] += color;
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Vector3::new(0f32, 0f32, 0f32);
        }
    }

    pub fn normalize(&mut self, scale: f32) {
        for i in 0..self.data.len() {
            self.data[i] /= scale;
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for color in &self.data {
            bytes.push((color.x * 255f32) as u8);
            bytes.push((color.y * 255f32) as u8);
            bytes.push((color.z * 255f32) as u8);
            bytes.push(255);
        }

        return bytes.to_owned();
    }
}

pub struct Renderer<'a> {
    num_samples: u32,
    scene: &'a Scene,
    camera: &'a RayCamera,
    num_bounces: u32,
}

impl Renderer<'_> {
    pub fn new<'a>(scene: &'a Scene, camera: &'a RayCamera) -> Renderer<'a> {
        return Renderer {
            num_samples: 0,
            scene,
            camera,
            num_bounces: 4,
        };
    }

    pub fn render_sample(&mut self, width: usize, height: usize, frame_buffer: &mut Framebuffer) {
        //let now = Instant::now();

        for y in 0..frame_buffer.height {
            for x in 0..frame_buffer.width {
                //println!("({}, {})", x, y);
                let ray = self.camera.gen_primary_ray(x, y, width, height);
                frame_buffer.accum_pixel(x, y, sky_color(ray));
            }
        }
        //println!("\tOuter Elapsed: {}", now.elapsed().as_micros());

        self.num_samples += 1;
    }

    pub fn render_full(
        &mut self,
        width: usize,
        height: usize,
        frame_buffer: &mut Framebuffer,
        samples: u32,
    ) {
        frame_buffer.clear();

        for _ in 0..samples {
            self.render_sample(width, height, frame_buffer);
        }

        frame_buffer.normalize(self.num_samples as f32);

        self.num_samples = 0;
    }
}

fn sky_color(ray: Ray) -> Vector3 {
    let t = 0.5f32 * (ray.direction.y + 1.0f32);
    return Vector3::new(
        (1f32 - t) + (t * 138f32 / 255f32),
        (1f32 - t) + (t * 188f32 / 255f32),
        1f32,
    );
}
