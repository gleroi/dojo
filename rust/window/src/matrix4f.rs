use std;

#[repr(C)]
pub struct Matrix4f {
    values: [f32; 16]
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f {
            values: [0.0; 16],
        }
    }
}



impl std::ops::Index<(usize, usize)> for Matrix4f {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        return &self.values[index.0 * 4 + index.1];
    }
}

impl std::convert::AsRef<[f32]> for Matrix4f {
    fn as_ref(&self) -> &[f32] {
        return &self.values;
    }
}


impl std::ops::IndexMut<(usize, usize)> for Matrix4f {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        return &mut self.values[index.0 * 4 + index.1];
    }
}