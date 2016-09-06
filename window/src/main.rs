extern crate glutin;

use glutin::{Window, Event, WindowBuilder};

fn main() {
    let builder = WindowBuilder::new();
    let window = builder.with_title("rust window")
        .with_transparency(false)
        .with_decorations(true)
        .build()
        .unwrap();

    let _ = unsafe { window.make_current() };

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => { return; },
                _ => { println!("event: {:?}", event); } 
            }
        }
    }
}
