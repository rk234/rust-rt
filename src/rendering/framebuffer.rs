use raylib::math::Vector3;

pub struct Framebuffer {
    pub data: Vec<Vector3>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![Vector3::new(0f32, 0f32, 0f32); width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y * self.width] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vector3 {
        self.data[x + y * self.width]
    }

    pub fn accum_pixel(&mut self, x: usize, y: usize, color: Vector3) {
        self.data[x + y * self.width] += color;
    }

    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|v| {*v = Vector3::new(0f32, 0f32, 0f32)});
    }

    pub fn normalize(&mut self, scale: f32) {
        self.data.iter_mut().for_each(|v| {*v /= scale});
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        return self.to_bytes_s(1f32);
    }

    pub fn to_bytes_s(&self, scale: f32) -> Vec<u8>{
        let mut bytes: Vec<u8> = vec![0; self.width*self.height*4];

        let mut i = 0;
        for color in &self.data {
            bytes[i] = ((color.x/scale).sqrt() * 255f32) as u8;
            bytes[i+1] = ((color.y/scale).sqrt() * 255f32) as u8;
            bytes[i+2] = ((color.z/scale).sqrt() * 255f32) as u8;
            bytes[i+3] = 255;
            i+=4;
        }

        return bytes;
    }
}
