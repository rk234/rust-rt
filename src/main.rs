use raylib::prelude::*;
use rendering::{RayCamera, Framebuffer, Renderer};
use scene::Scene;
mod rendering;
mod scene;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    let mut img = Image::gen_image_color(640, 480, Color::RED);

    let cam = RayCamera::new(Vector3::new(0f32, 0f32, 0f32));
    let mut framebuf = Framebuffer::new(640, 480);
    let scene = Scene::new();
    let mut renderer = Renderer::new(&scene, &cam);

    renderer.render_full(640, 480, &mut framebuf, 10);



    rl.set_target_fps(60);
    while !rl.window_should_close() {
        for y in 0..framebuf.height {
            for x in 0..framebuf.width {
                let c_vec = framebuf.get_pixel(x, y);
                img.draw_pixel(x as i32, y as i32, Color::new((c_vec.x*255f32) as u8, (c_vec.y*255f32) as u8, (c_vec.z*255f32) as u8, 255));
            }
        }
        let tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");
        let fps = rl.get_fps();

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        
        d.draw_texture(&tex, 0, 0, Color::WHITE);

        d.draw_text(format!("FPS: {}", fps).as_str(), 12, 12, 20, Color::BLACK);
    }
}