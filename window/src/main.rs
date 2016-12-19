extern crate gl;
extern crate glutin;

use glutin::{Event, WindowBuilder};

mod vector3f;
mod tut3;

use tut3::Tut;

fn main() {
    let builder = WindowBuilder::new();
    let window = builder.with_title("ogl tutorials")
        .with_transparency(false)
        .with_decorations(true)
        .build()
        .unwrap();

    let _ = unsafe { window.make_current() };

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    }

    let mut tut = Tut::new();
    tut.init();

    loop {
        tut.render();

        let _ = window.swap_buffers();

        for event in window.poll_events() {
            match event {
                Event::Closed => { return; },
                _ => { println!("event: {:?}", event); } 
            }
        }
    }
}
