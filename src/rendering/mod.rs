pub struct Renderer {
    num_samples: u32,
}

impl Renderer {
    pub fn new() -> Self {
        return Renderer {
            num_samples: 1
        }
    }
}