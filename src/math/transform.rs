use raylib::math::Matrix;

pub struct Transform {
    pub m: Matrix,
    pub inv: Matrix,
}

impl Transform {
    pub fn new(m: Matrix) -> Self {
        Self {
            m,
            inv: m.inverted(),
        }
    }
}
