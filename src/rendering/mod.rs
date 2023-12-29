use std::ops::Add;
use crate::scene::Scene;
use raylib::prelude::*;

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
    pub fn gen_primary_ray(&self) -> Ray {
        todo!()
        //return Ray::new(origin, direction)
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
                frame_buffer.accum_pixel(x, y, Vector3::new(1f32, 0f32, 0f32))
            }
        }

        self.num_samples+=1;
    }
}