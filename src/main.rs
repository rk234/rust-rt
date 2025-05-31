use raylib::prelude::*;
use rust_rt::math::Transform;
use rust_rt::rendering::Framebuffer;
use rust_rt::rendering::RayCamera;
use rust_rt::rendering::Renderer;
use rust_rt::rendering::{EmissiveMaterial, LambertianMaterial, MetalMaterial, RTMaterial};
use rust_rt::scene::mesh::Mesh;
use rust_rt::scene::models::{Scene, SceneObject};
use rust_rt::scene::sphere::Sphere;
use rust_rt::scene::Plane;
use std::f32::consts::PI;
use std::{ffi::CString, sync::Arc};

fn main() {
    const HEIGHT: i32 = 500;
    const WIDTH: i32 = 16 * HEIGHT / 9;
    let mut res_scale: f32 = 0.2f32;

    let (mut rl, thread) = init()
        .size(WIDTH, HEIGHT)
        .resizable()
        .title("Rust Ray Tracing")
        .build();

    rl.gui_enable();

    let img = Image::gen_image_color(WIDTH, HEIGHT, Color::RED);

    let mut cam = RayCamera::new(Vector3::new(0f32, 2f32, 0f32));
    let mut framebuf = Framebuffer::new(WIDTH as usize, HEIGHT as usize);
    let mut scene = Scene::new();

    init_sphere_scene(&mut scene);

    let mut renderer = Renderer::new(&scene);

    let mut tex = rl
        .load_texture_from_image(&thread, &img)
        .expect("Failed to load");

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

        if s_width != prev_s_width || s_height != prev_s_height || prev_scale != res_scale {
            framebuf = Framebuffer::new(
                (s_width as f32 * res_scale) as usize,
                (s_height as f32 * res_scale) as usize,
            );
            tex = rl
                .load_texture_from_image(
                    &thread,
                    &Image::gen_image_color(
                        framebuf.width as i32,
                        framebuf.height as i32,
                        Color::BLACK,
                    ),
                )
                .expect("Failed to allocate texture");
            renderer.reset();
            prev_scale = res_scale;
        }

        if prev_cam_dir != cam.direction || prev_cam_pos != cam.position {
            renderer.reset();
            framebuf.clear();
            prev_cam_dir = cam.direction;
            prev_cam_pos = cam.position;
        }

        if continue_rendering {
            if rl.is_key_down(KeyboardKey::KEY_N) {
                renderer.render_normals(framebuf.width, framebuf.height, &mut framebuf, &mut cam);
                tex.update_texture(&framebuf.to_bytes());
            } else {
                renderer.render_sample(framebuf.width, framebuf.height, &mut framebuf, &mut cam);
                tex.update_texture(&framebuf.to_bytes_s(renderer.num_samples as f32));
            }
        }

        let fps = rl.get_fps();
        let dt = rl.get_frame_time() * 1000f32;

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        d.draw_texture_pro(
            &tex,
            Rectangle::new(0f32, 0f32, framebuf.width as f32, framebuf.height as f32),
            Rectangle::new(0f32, 0f32, s_width as f32, s_height as f32),
            Vector2::new(0f32, 0f32),
            0f32,
            Color::WHITE,
        );

        d.draw_text(
            format!(
                "FPS: {}; dt: {:.2}ms; samples: {}",
                fps, dt, renderer.num_samples
            )
            .as_str(),
            12,
            12,
            20,
            Color::GREEN,
        );

        if d.gui_button(
            Rectangle::new(0f32, (s_height - 50) as f32, 100f32, 50f32),
            Some(CString::new("Reset Renderer").unwrap().as_c_str()),
        ) {
            renderer.reset();
            framebuf.clear();
        }

        if d.gui_button(
            Rectangle::new(100f32, (s_height - 50) as f32, 100f32, 50f32),
            Some(CString::new("Toggle Rendering").unwrap().as_c_str()),
        ) {
            continue_rendering = !continue_rendering;
        }

        res_scale = d.gui_slider_bar(
            Rectangle::new(200f32, (s_height - 20) as f32, 200f32, 20f32),
            None,
            None,
            res_scale,
            0.01f32,
            1f32,
        );

        handle_input(&mut d, &mut cam);

        prev_s_width = s_width;
        prev_s_height = s_height;
    }
}

fn handle_input(d: &RaylibDrawHandle<'_>, cam: &mut RayCamera) {
    if d.is_key_down(KeyboardKey::KEY_W) {
        cam.position += cam.direction * 0.05;
    }
    if d.is_key_down(KeyboardKey::KEY_S) {
        cam.position -= cam.direction * 0.05;
    }
    if d.is_key_down(KeyboardKey::KEY_A) {
        cam.position -= Vector3::new(0.0, 1.0, 0.0).cross(cam.direction) * 0.05;
    }
    if d.is_key_down(KeyboardKey::KEY_D) {
        cam.position += Vector3::new(0.0, 1.0, 0.0).cross(cam.direction) * 0.05;
    }

    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let mut dx = -d.get_mouse_delta().x;
        let mut dy = -d.get_mouse_delta().y;

        let sensitivity = 0.3f32;
        dx *= sensitivity;
        dy *= sensitivity;

        cam.yaw += dx;
        cam.pitch += dy;

        if cam.pitch > 89.0 {
            cam.pitch = 89.0;
        }
        if cam.pitch < -89.0 {
            cam.pitch = -89.0;
        }

        cam.direction = Vector3::new(
            cam.yaw.to_radians().cos() * cam.pitch.to_radians().cos(),
            cam.pitch.to_radians().sin(),
            cam.yaw.to_radians().sin() * cam.pitch.to_radians().cos(),
        );
    }
}

fn init_sphere_scene(scene: &mut Scene) {
    let white_diffuse_mat: Arc<dyn RTMaterial> = Arc::new(LambertianMaterial::new(Vector3::new(
        0.5f32, 0.5f32, 0.5f32,
    )));
    let red_diffuse_mat: Arc<dyn RTMaterial> =
        Arc::new(LambertianMaterial::new(Vector3::new(0.65, 0.05, 0.05)));

    let metal_mat = Arc::new(MetalMaterial::new(Vector3::one(), 0.1));

    let bottom_plane: Box<dyn SceneObject> = Box::new(Plane::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Arc::clone(&white_diffuse_mat),
    ));

    let sphere_b: Box<dyn SceneObject> = Box::new(Sphere::new(
        Vector3::new(0.0, 0.75, 3.0),
        0.75,
        Arc::clone(&red_diffuse_mat),
    ));

    let t = Matrix::translate(0f32, 3f32, 5f32);

    let model = Mesh::from_obj(
        "models/dragon_simple.obj",
        Transform::new(Matrix::rotate_y(PI) * Matrix::scale(10f32, 10f32, 10f32) * t),
        Arc::clone(&white_diffuse_mat),
    );

    scene.add_object(bottom_plane);
    // scene.add_object(sphere_b);
    scene.add_object(Box::new(model));
}
