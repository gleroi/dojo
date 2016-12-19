extern crate gl;

use std;
use std::os::raw::c_void;

use vector3f::Vector3f;

pub struct Tut2 {
    vbo: u32,
    vertices: [Vector3f; 3]
}

impl Tut2 {

    pub fn new() -> Tut2 {
        Tut2 {
            vbo: 0,
            vertices: [Vector3f::default(),Vector3f::default(),Vector3f::default()]
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.vertices = [
                Vector3f::new(-1.0,-1.0,0.0),
                Vector3f::new(1.0,-1.0,0.0),
                Vector3f::new(0.0,1.0,0.0)
            ];
            
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * std::mem::size_of::<Vector3f>()) as isize, &self.vertices as * const Vector3f as * const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn render(&self) {
        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(0);
        }
    }
}