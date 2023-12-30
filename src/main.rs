use raylib::prelude::*;
mod rendering;
mod scene;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    let img = Image::gen_image_color(640, 480, Color::RED);
    let tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load texture");
    
    while !rl.window_should_close() {
        /*img.draw_pixel(rl.get_mouse_x(), rl.get_mouse_y(), Color::BLUE);
        tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load");*/
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&tex, 0, 0, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}