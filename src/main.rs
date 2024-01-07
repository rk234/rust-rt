use raylib::prelude::*;
use rendering::{RayCamera, Framebuffer, Renderer};
use scene::Scene;
use std::{rc::Rc, ffi::CString};

use crate::{scene::Sphere, rendering::{RTMaterial, LambertianMaterial}};
mod rendering;
mod scene;
mod utils;

fn main() {
    const HEIGHT: i32 = 500;
    const WIDTH: i32 = 16*HEIGHT/9;
    const RES_SCALE: f32 = 0.7f32;

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
    
    let mat: Rc<dyn RTMaterial> = Rc::new(LambertianMaterial::new(Vector3::new(0.8f32, 0.8f32, 0.8f32)));
    scene.add_object(Box::new(Sphere::new(Vector3::new(0f32, 3f32, 10f32), 3f32, Rc::clone(&mat))));
    scene.add_object(Box::new(Sphere::new(Vector3::new(0f32, -50f32, 10f32), 50f32, Rc::clone(&mat))));

    let mut renderer = Renderer::new(&scene, &mut cam);

    let mut tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");

    rl.set_target_fps(60);
    let mut prev_s_width = rl.get_screen_width();
    let mut prev_s_height = rl.get_screen_height();

    while !rl.window_should_close() {
        let s_width = rl.get_screen_width();
        let s_height = rl.get_screen_height();

        if s_width != prev_s_width || s_height != prev_s_height {
            framebuf = Framebuffer::new((s_width as f32 * RES_SCALE) as usize, (s_height as f32 *RES_SCALE) as usize);
            tex = rl.load_texture_from_image(&thread, &Image::gen_image_color(framebuf.width as i32, framebuf.height as i32, Color::BLACK)).expect("Failed to allocate texture");
            renderer.reset();
        }

        //let now = Instant::now();
        renderer.render_sample(framebuf.width, framebuf.height, &mut framebuf);
        tex.update_texture(&framebuf.to_bytes_s(renderer.num_samples as f32));
        //println!("Outer elapsed {}ms", now.elapsed().as_millis());


        let fps = rl.get_fps();
        let dt = rl.get_frame_time() * 1000f32;
        

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        //d.draw_texture(&tex, 0, 0, Color::WHITE);
        d.draw_texture_pro(&tex, Rectangle::new(0f32, 0f32, framebuf.width as f32, framebuf.height as f32), Rectangle::new(0f32,0f32, s_width as f32, s_height as f32), Vector2::new(0f32, 0f32), 0f32, Color::WHITE);

        d.draw_text(format!("FPS: {}; dt: {:.2}ms; samples: {}", fps, dt, renderer.num_samples).as_str(), 12, 12, 20, Color::BLACK);
        
        if d.gui_button(Rectangle::new(0f32,(s_height-50) as f32, 100f32, 50f32), Some(CString::new("Hello World").unwrap().as_c_str())) {
            println!("Pressed!");
        }
        prev_s_width = s_width;
        prev_s_height = s_height;
    }
}