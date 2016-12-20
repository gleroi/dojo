extern crate gl;
use gl::types::*;

use std;
use std::os::raw::c_void;
use std::str::FromStr;

use vector3f::Vector3f;

pub struct Tut {
    vbo: GLuint,
    shaderProgram: GLuint,
    scaleLocation: GLint,
    vertices: [Vector3f; 3]
}

static mut scale :f32 = 0.0;

impl Tut {

    pub fn new() -> Tut {
        Tut {
            vbo: 0,
            shaderProgram: 0,
            scaleLocation: 0,
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
            Tut::read_file("tut4/shader.vs", &mut vs);
            let mut fs = String::new();
            Tut::read_file("tut4/shader.fs", &mut fs);

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

            self.scaleLocation = gl::GetUniformLocation(self.shaderProgram, "gScale" as *const str as * const i8);
        }
    }

    fn add_shader(&self, source: &str, shader_type: GLenum) {
        unsafe {
            
            let shader = gl::CreateShader(shader_type);
            if shader == 0 {
                panic!("Error creating shader type {}", shader);
            }

            let csource = std::ffi::CString::new(source).unwrap();
            let sources : [* const i8; 1] = [csource.as_ptr()];
            let lengths = [source.len() as i32];
            gl::ShaderSource(shader, 1, &sources as *const *const i8, &lengths as *const i32);
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut error : [u8; 1024] = [0; 1024];
                gl::GetProgramInfoLog(shader, error.len() as i32, std::ptr::null_mut(), &mut error as * mut [u8] as *mut i8);
                panic!("Error compiling shader type {}: {}", shader_type, String::from_utf8_lossy(&error));
            }
            gl::AttachShader(self.shaderProgram, shader);
        }
    }

    fn read_file(path: &str, buffer: &mut String) {
        use std::io::Read;

        let mut file = std::fs::File::open(path).unwrap();
        file.read_to_string(buffer);
    }

    pub fn render(&self) {
        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            scale += 0.0001;
            gl::Uniform1f(self.scaleLocation, scale.sin());

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(0);
        }
    }
}