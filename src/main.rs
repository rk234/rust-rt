use raylib::prelude::*;
use rendering::{RayCamera, Framebuffer, Renderer};
use scene::Scene;
use std::rc::Rc;

use crate::{scene::Sphere, rendering::{RTMaterial, LambertianMaterial}};
mod rendering;
mod scene;
mod utils;

fn main() {
    const HEIGHT: i32 = 500;
    const WIDTH: i32 = 16*HEIGHT/9;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .resizable()
        .title("Rust Ray Tracing")
        .build();

    let img = Image::gen_image_color(WIDTH, HEIGHT, Color::RED);

    let mut cam = RayCamera::new(Vector3::new(0f32, 3f32, 0f32));
    let mut framebuf = Framebuffer::new(WIDTH as usize, HEIGHT as usize);
    let mut scene = Scene::new();
    
    let mat: Rc<dyn RTMaterial> = Rc::new(LambertianMaterial::new(Vector3::new(0.8f32, 0.8f32, 0.8f32)));
    scene.add_object(Box::new(Sphere::new(Vector3::new(0f32, 3f32, 10f32), 3f32, Rc::clone(&mat))));
    scene.add_object(Box::new(Sphere::new(Vector3::new(0f32, -50f32, 10f32), 50f32, Rc::clone(&mat))));

    let mut renderer = Renderer::new(&scene, &mut cam);

    let mut tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        //let now = Instant::now();
        renderer.render_sample(framebuf.width, framebuf.height, &mut framebuf);
        tex.update_texture(&framebuf.to_bytes_s(renderer.num_samples as f32));
        //println!("Outer elapsed {}ms", now.elapsed().as_millis());


        let fps = rl.get_fps();
        let dt = rl.get_frame_time() * 1000f32;

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&tex, 0, 0, Color::WHITE);

        d.draw_text(format!("FPS: {}; dt: {:.2}ms", fps, dt).as_str(), 12, 12, 20, Color::BLACK);
    }
}