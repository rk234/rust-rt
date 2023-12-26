use raylib::prelude::*;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    let mut img = Image::gen_image_color(640, 480, Color::RED);
    
    for i in 0..10 {
        img.draw_pixel(i, i, Color::BLUE);
    }

    
    let tex = rl.load_texture_from_image(&thread, &img).expect("Failed to load texture");
    
    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&tex, 0, 0, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}