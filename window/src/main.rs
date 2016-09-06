extern crate glutin;

use glutin::{Window, Event};

fn main() {
    let window = Window::new().unwrap();
    window.set_title("rust window");

    let _ = unsafe { window.make_current() };

    println!("window is current: {}", window.is_current());

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => { return; },
                _ => { println!("event: {:?}", event); } 
            }
        }
    }
}
