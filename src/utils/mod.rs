use rand::Rng;
use raylib::prelude::Vector3;


pub fn rand_vec3() -> Vector3 {
    let mut rng = rand::thread_rng();

    return Vector3::new(rng.gen_range(-1f32..=1f32), rng.gen_range(-1f32..=1f32), rng.gen_range(-1f32..=1f32));
}

pub fn rand_in_unit_sphere() -> Vector3 {
    let mut vec = rand_vec3();

    while vec.length() > 1f32 {
        vec = rand_vec3();
    }

    return vec;
}

pub fn rand_unit_vec() -> Vector3 {
    return rand_in_unit_sphere().normalized();
}


pub fn rand_in_hemisphere(normal: Vector3) -> Vector3 {
    let vec = rand_unit_vec();

    if vec.dot(normal) > 0f32  {
        return vec;
    } else {
        return vec.scale_by(-1f32);
    }
}