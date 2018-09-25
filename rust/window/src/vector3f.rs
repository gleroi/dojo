
#[derive(Default)]
#[repr(C)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z:f32) -> Vector3f {
        Vector3f {
            x: x,
            y: y,
            z: z
        }
    }
}