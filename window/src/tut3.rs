extern crate gl;
use gl::types::*;

use std;
use std::os::raw::c_void;
use std::str::FromStr;

use vector3f::Vector3f;

pub struct Tut {
    vbo: GLuint,
    shaderProgram: GLuint,
    vertices: [Vector3f; 3]
}

impl Tut {

    pub fn new() -> Tut {
        Tut {
            vbo: 0,
            shaderProgram: 0,
            vertices: [Vector3f::default(),Vector3f::default(),Vector3f::default()],
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
        self.compile_shaders();
    }

    pub fn compile_shaders(&mut self) {
        unsafe {
            self.shaderProgram = gl::CreateProgram();
            if self.shaderProgram == 0 {
                panic!("Error creating shader program");
            }

            let mut vs = String::new();
            Tut::read_file("shader.vs", &mut vs);
            let mut fs = String::new();
            Tut::read_file("shader.fs", &mut fs);

            self.add_shader(&vs, gl::VERTEX_SHADER);
            self.add_shader(&fs, gl::FRAGMENT_SHADER);

            let mut success : GLint = 0;
            let mut error : [u8; 1024] = [0; 1024];

            gl::LinkProgram(self.shaderProgram);
            gl::GetProgramiv(self.shaderProgram, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(self.shaderProgram, error.len() as i32, std::ptr::null_mut(), &mut error as * mut [u8] as *mut i8);
                panic!("Error linking shader program: {}", String::from_utf8_lossy(&error));
            }

            gl::ValidateProgram(self.shaderProgram);
            gl::GetProgramiv(self.shaderProgram, gl::VALIDATE_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(self.shaderProgram, error.len() as i32, std::ptr::null_mut(), &mut error as * mut [u8] as *mut i8);
                panic!("Error invalid shader program: {}", String::from_utf8_lossy(&error));
            }

            gl::UseProgram(self.shaderProgram);
        }
    }

    fn add_shader(&self, source: &str, shader_type: GLenum) {

    }

    fn read_file(path: &str, buffer: &mut String) {
        use std::io::Read;

        let mut file = std::fs::File::open(path).unwrap();
        file.read_to_string(buffer);
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