use raylib::prelude::*;
pub struct RayCamera {
    position: Vector3,
    direction: Vector3,
    pitch: f32,
    yaw: f32,
    near_plane: f32,
    viewport_size: Vector3
}

pub struct Renderer {
    num_samples: u32,
    scene: &Scene,
    camera: &RayCamera,
    num_bounces: u32,
    EPSILON: f64
}

impl Renderer {
    pub fn new(scene: &Scene, camera: &RayCamera) -> Self {
        return Renderer {
            num_samples: 1,
            
        }
    }

    pub fn render_sample(&self, width: usize, height: usize, accum_buffer: &mut [Vector3]) {
        for y in 0..height {
            for x in 0..width {
                accum_buffer[x + width*y] = Vector3::new(0f32, 0f32, 0f32);
            }
        }
        self.num_samples+=1;
    }
}