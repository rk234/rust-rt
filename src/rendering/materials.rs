use raylib::math::Vector3;
use crate::rendering;
use crate::rendering::renderer::EPSILON;
use crate::utils::{rand_in_hemisphere, rand_unit_vec, reflect};

pub trait RTMaterial: Send + Sync {
    fn attenuation(&self, position: Vector3, normal: Vector3) -> Vector3;
    fn scatter(&self, in_ray: rendering::Ray, position: Vector3, normal: Vector3) -> Option<rendering::Ray>;
    fn emissive(&self, position: Vector3, normal: Vector3) -> bool;
}

pub struct LambertianMaterial {
    albedo: Vector3,
}

impl LambertianMaterial {
    pub fn new(albedo: Vector3) -> LambertianMaterial {
        return LambertianMaterial { albedo }
    }
}

impl RTMaterial for LambertianMaterial {
    fn attenuation(&self, _: Vector3, _: Vector3) -> Vector3 {
        return self.albedo;
    }

    fn scatter(&self, _: rendering::Ray, position: Vector3, normal: Vector3) -> Option<rendering::Ray> {
        return Some(rendering::Ray::new(position, normal + rand_in_hemisphere(normal)))
    }

    fn emissive(&self, _: Vector3, _: Vector3) -> bool {
        false
    }
}

pub struct EmissiveMaterial {
    emit: Vector3
}

impl EmissiveMaterial {
    pub fn new(emit: Vector3) -> EmissiveMaterial {
        return EmissiveMaterial { emit }
    }
}

impl RTMaterial for EmissiveMaterial {
    fn attenuation(&self, _: Vector3, _: Vector3) -> Vector3 {
        self.emit
    }

    fn scatter(&self, _: rendering::Ray, _: Vector3, _: Vector3) -> Option<rendering::Ray> {
        None
    }

    fn emissive(&self, _: Vector3, _: Vector3) -> bool {
        true
    }
}

pub struct MetalMaterial {
    roughness: f32,
    albedo: Vector3
}

impl MetalMaterial {
    pub fn new(albedo: Vector3, roughness: f32) -> MetalMaterial {
        return MetalMaterial {
            roughness,
            albedo
        };
    }
}

impl RTMaterial for MetalMaterial {
    fn attenuation(&self, _position: Vector3, _normal: Vector3) -> Vector3 {
        return self.albedo;
    }

    fn scatter(&self, in_ray: rendering::Ray, position: Vector3, normal: Vector3) -> Option<rendering::Ray> {
        return Some(rendering::Ray::new(
            position + (normal*EPSILON),
            reflect(in_ray.direction, normal) + (rand_unit_vec()*self.roughness)
        ))
    }

    fn emissive(&self, _position: Vector3, _normal: Vector3) -> bool {
        return false
    }
}
