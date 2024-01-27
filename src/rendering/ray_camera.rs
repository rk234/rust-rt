use std::ops::Add;
use rand::Rng;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector3;
use crate::rendering::Ray;

pub struct RayCamera {
    pub position: Vector3,
    pub direction: Vector3,
    pub pitch: f32,
    pub yaw: f32,
    near_plane: f32,
    viewport_size: Vector3,
}

impl RayCamera {
    pub fn new(position: Vector3) -> RayCamera {
        RayCamera {
            position,
            direction: Vector3::new(0f32, 0f32, 1f32),
            pitch: 0f32,
            yaw: 90f32,
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
            .normalized();
        let local_up = adjacent.cross(self.direction).normalized();
        let bottom_left = adjacent
            .scale_by(-self.viewport_size.x as f32 / 2f32)
            .add(local_up.scale_by(-self.viewport_size.y as f32 / 2f32));

        let dir = bottom_left
            .add(adjacent.scale_by(
                (self.viewport_size.x)
                    * ((screen_x as f32 + rng.gen_range(-0.5f32..0.5f32)) / screen_width as f32),
            ))
            .add(local_up.scale_by(
                (self.viewport_size.y)
                    * ((screen_y as f32 + rng.gen_range(-0.5f32..0.5f32)) / screen_height as f32),
            ))
            .add(self.direction.scale_by(self.near_plane))
            .normalized();

        return Ray::new(self.position, dir);
    }

    pub fn handle_input(&self, handle: &RaylibDrawHandle<'_>) {

    }
}