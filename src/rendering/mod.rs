use std::ops::Add;
use crate::scene::Scene;
use raylib::prelude::*;
use rand::Rng;

pub const EPSILON: f32 = 0.00001f32;

pub struct RayCamera {
    position: Vector3,
    direction: Vector3,
    pitch: f32,
    yaw: f32,
    near_plane: f32,
    viewport_size: Vector3
}

impl RayCamera {
    pub fn update_viewport(&mut self, screen_width: usize, screen_height: usize) {
        self.viewport_size = Vector3::new((screen_width as f32/screen_height as f32)*1.5f32, 1.5f32, 0f32)
    }

    pub fn gen_primary_ray(&self, screen_x: usize, screen_y: usize, screen_width: usize, screen_height: usize) -> Ray {
        let mut rng = rand::thread_rng();
        let adjacent = Vector3::new(0f32, 1f32, 0f32).cross(self.direction).normalized();
        let local_up = adjacent.cross(self.direction).normalized();
        let bottom_left = adjacent.scale_by(-self.viewport_size.x as f32 / 2f32).add(local_up.scale_by(-self.viewport_size.y as f32 / 2f32));
        
        let dir = bottom_left
        .add(adjacent.scale_by((self.viewport_size.x) * (screen_x as f32 + rng.gen_range(-0.5f32..0.5f32) / screen_width as f32)))
        .add(local_up.scale_by((self.viewport_size.y) * (screen_y as f32 + rng.gen_range(-0.5f32..0.5f32) / screen_height as f32)))
        .add(self.direction.scale_by(self.near_plane)).normalized();
        
        return Ray::new(
            self.position,
            dir
        );
    }
}

pub struct Ray {
    origin: Vector3,
    direction: Vector3
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
    width: usize,
    height: usize
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![Vector3::new(0f32, 0f32, 0f32); width*height],
            width,
            height
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y*self.width] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vector3 {
        self.data[x+y*self.width]
    }

    pub fn accum_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y*self.width] += color;
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
            num_samples: 1,
            scene,
            camera,
            num_bounces: 4
        }
    }

    pub fn render_sample(&mut self, width: usize, height: usize, frame_buffer: &mut Framebuffer) {
        for y in 0..frame_buffer.height {
            for x in 0..frame_buffer.width {
                let ray = self.camera.gen_primary_ray(x, y, width, height);
                frame_buffer.accum_pixel(x, y, sky_color(ray))
            }
        }

        self.num_samples+=1;
    }

    pub fn render_full(&mut self, width: usize, height: usize, frame_buffer: &mut Framebuffer, samples: u32) {
        for _ in 0..samples {
            self.render_sample(width, height, frame_buffer);
        }

        for y in 0..frame_buffer.height {
            for x in 0..frame_buffer.width {
                frame_buffer.set_pixel(x, y, frame_buffer.get_pixel(x, y).scale_by(1f32/(self.num_samples as f32)));
            }
        }
    }
}


fn sky_color(ray: Ray) -> Vector3 {
    let t = 0.5f32*(ray.direction.y + 1.0f32);
    return Vector3::new(1f32-t, 1f32-t, 1f32-t) + Vector3::new((t * 138f32/255f32), (t*188f32/255f32),t);
}