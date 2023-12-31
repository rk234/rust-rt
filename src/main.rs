use raylib::prelude::*;
use rendering::{RayCamera, Framebuffer, Renderer};
use scene::Scene;
mod rendering;
mod scene;
fn main() {
    const WIDTH: i32 = 16*480/9;
    const HEIGHT: i32 = 480;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Rust Ray Tracing")
        .build();
    let img = Image::gen_image_color(WIDTH, HEIGHT, Color::RED);

    let cam = RayCamera::new(Vector3::new(0f32, 0f32, 0f32));
    let mut framebuf = Framebuffer::new(WIDTH as usize, HEIGHT as usize);
    let scene = Scene::new();
    let mut renderer = Renderer::new(&scene, &cam);

    let mut tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        //let now = Instant::now();
        renderer.render_full(framebuf.width, framebuf.height, &mut framebuf, 1);
        tex.update_texture(&framebuf.to_bytes());
        //println!("Outer elapsed {}ms", now.elapsed().as_millis());


        let fps = rl.get_fps();
        let dt = rl.get_frame_time() * 1000f32;

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&tex, 0, 0, Color::WHITE);

        d.draw_text(format!("FPS: {}; Frame time: {}", fps, dt).as_str(), 12, 12, 20, Color::BLACK);
    }
}