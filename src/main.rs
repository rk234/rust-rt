use raylib::prelude::*;
use rendering::{RayCamera, Framebuffer, Renderer};
use scene::Scene;
use std::{ffi::CString, sync::Arc};

use crate::{scene::{Sphere, Plane, Quad, SceneObject}, rendering::{RTMaterial, LambertianMaterial, EmissiveMaterial, MetalMaterial}};
mod rendering;
mod scene;
mod utils;

fn main() {
    const HEIGHT: i32 = 500;
    const WIDTH: i32 = 16*HEIGHT/9;
    let mut res_scale: f32 = 0.8f32;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .resizable()
        .title("Rust Ray Tracing")
        .build();

    rl.gui_enable();

    let img = Image::gen_image_color(WIDTH, HEIGHT, Color::RED);

    let mut cam = RayCamera::new(Vector3::new(0f32, 3f32, 0f32));
    let mut framebuf = Framebuffer::new(WIDTH as usize, HEIGHT as usize);
    let mut scene = Scene::new();
    
    let white_diffuse_mat: Arc<dyn RTMaterial> = Arc::new(LambertianMaterial::new(Vector3::new(0.8f32, 0.8f32, 0.8f32)));
    let green_diffuse_mat: Arc<dyn RTMaterial> = Arc::new(LambertianMaterial::new(Vector3::new(0.0f32, 1.0f32, 0.0f32)));
    let red_diffuse_mat: Arc<dyn RTMaterial> = Arc::new(LambertianMaterial::new(Vector3::new(1.0f32, 0.0f32, 0.0f32)));
    let metal: Arc<dyn RTMaterial> = Arc::new(MetalMaterial::new(Vector3::new(1.0, 1.0, 1.0), 0.0f32));
    let light: Arc<dyn RTMaterial> = Arc::new(EmissiveMaterial::new(Vector3::new(10.0, 10.0, 10.0)));

    let left_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(-1.0, 3.0, 3.0), Vector3::new(0.0, 3.0, 0.0), Vector3::new(0.0, 0.0, 2.0), Arc::clone(&green_diffuse_mat)));
    let right_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(1.0, 3.0, 3.0), Vector3::new(0.0, 3.0, 0.0), Vector3::new(0.0, 0.0, 2.0), Arc::clone(&red_diffuse_mat)));
    let back_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(0.0, 3.0, 3.0), Vector3::new(0.0, 3.0, 0.0), Vector3::new(2.0, 0.0, 0.0), Arc::clone(&white_diffuse_mat)));
    //let front_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(0.0, 4.5, 0.0), Vector3::new(0.0, 3.0, 0.0), Vector3::new(5.0, 0.0, 0.0), Arc::clone(&white_diffuse_mat)));
    let bottom_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(0.0, 1.5, 3.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 2.0), Arc::clone(&white_diffuse_mat)));
    let top_wall: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(0.0, 4.5, 3.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 2.0), Arc::clone(&white_diffuse_mat)));
    let light_source: Box<dyn SceneObject> = Box::new(Quad::new(Vector3::new(0.0, 4.499, 1.0), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Arc::clone(&light)));

    let sphere_a: Box<dyn SceneObject> = Box::new(Sphere::new(Vector3::new(0.5, 2.0,2.5), 0.5, Arc::clone(&metal)));
    let sphere_b: Box<dyn SceneObject> = Box::new(Sphere::new(Vector3::new(-0.5, 2.0,2.0), 0.5, Arc::clone(&white_diffuse_mat)));

    scene.add_object(left_wall);
    scene.add_object(right_wall);
    scene.add_object(back_wall);
    //scene.add_object(front_wall);
    scene.add_object(bottom_wall);
    scene.add_object(top_wall);
    scene.add_object(light_source);
    scene.add_object(sphere_a);
    scene.add_object(sphere_b);

    
    let mut renderer = Renderer::new(&scene);

    let mut tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");

    rl.set_target_fps(60);
    let mut prev_s_width = rl.get_screen_width();
    let mut prev_s_height = rl.get_screen_height();
    let mut prev_scale: f32 = res_scale;
    let mut continue_rendering = true;
    let mut prev_cam_dir = cam.direction;
    let mut prev_cam_pos = cam.position;

    while !rl.window_should_close() {
        let s_width = rl.get_screen_width();
        let s_height = rl.get_screen_height();

        if s_width != prev_s_width || s_height != prev_s_height || prev_scale != res_scale || prev_cam_dir != cam.direction || prev_cam_pos != cam.position{
            framebuf = Framebuffer::new((s_width as f32 * res_scale) as usize, (s_height as f32 * res_scale) as usize);
            tex = rl.load_texture_from_image(&thread, &Image::gen_image_color(framebuf.width as i32, framebuf.height as i32, Color::BLACK)).expect("Failed to allocate texture");
            renderer.reset();
            prev_scale = res_scale;
            prev_cam_dir = cam.direction;
            prev_cam_pos = cam.position;
        }
        
        if continue_rendering {
            renderer.render_sample(framebuf.width, framebuf.height, &mut framebuf, &mut cam);
            tex.update_texture(&framebuf.to_bytes_s(renderer.num_samples as f32));
        }

        let fps = rl.get_fps();
        let dt = rl.get_frame_time() * 1000f32;

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        d.draw_texture_pro(&tex, Rectangle::new(0f32, 0f32, framebuf.width as f32, framebuf.height as f32), Rectangle::new(0f32,0f32, s_width as f32, s_height as f32), Vector2::new(0f32, 0f32), 0f32, Color::WHITE);

        d.draw_text(format!("FPS: {}; dt: {:.2}ms; samples: {}", fps, dt, renderer.num_samples).as_str(), 12, 12, 20, Color::GREEN);
        
        if d.gui_button(Rectangle::new(0f32,(s_height-50) as f32, 100f32, 50f32), Some(CString::new("Reset Renderer").unwrap().as_c_str())) {
            renderer.reset();
            framebuf.clear();
        }

        if d.gui_button(Rectangle::new(100f32,(s_height-50) as f32, 100f32, 50f32), Some(CString::new("Toggle Rendering").unwrap().as_c_str())) {
            continue_rendering = !continue_rendering;
        }

        res_scale = d.gui_slider_bar(Rectangle::new(200f32, (s_height-20) as f32, 200f32, 20f32), None, None, res_scale, 0.01f32, 1f32);

        
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mut dx = -d.get_mouse_delta().x;
            let mut dy = -d.get_mouse_delta().y;

            let sensitivity = 0.3f32;
            dx *= sensitivity;
		    dy *= sensitivity;
            
		    cam.yaw   += dx;
		    cam.pitch += dy;
	
		    if cam.pitch > 89.0 {
		        cam.pitch = 89.0;
            }
		    if cam.pitch < -89.0 {
		        cam.pitch = -89.0;
            }
		    
            cam.direction = Vector3::new(cam.yaw.to_radians().cos() * cam.pitch.to_radians().cos(), cam.pitch.to_radians().sin(), cam.yaw.to_radians().sin() * cam.pitch.to_radians().cos());
        }

        if d.is_key_down(KeyboardKey::KEY_W) {
            cam.position += cam.direction*0.05;
        } else if d.is_key_down(KeyboardKey::KEY_S) {
            cam.position -= cam.direction*0.05;
        } else if d.is_key_down(KeyboardKey::KEY_A) {

        } else if d.is_key_down(KeyboardKey::KEY_D) {

        }

        prev_s_width = s_width;
        prev_s_height = s_height;
    }
}