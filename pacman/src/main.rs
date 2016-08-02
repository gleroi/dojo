mod console;
mod game;
mod renderer;
mod map;

use std::thread;
use std::sync::mpsc;

use console::*;
use renderer::*;
use game::*;

fn run_input_thread(tx: mpsc::Sender<Direction>, default_direction: Direction) {
    let console = ConsoleInput::current();
    let mut direction = default_direction;
    loop {

        let val = console.read();

        const UP: char = 'z';
        const DOWN: char = 's';
        const LEFT: char = 'q';
        const RIGHT: char = 'd';

        direction = match val {
            Some(UP) => Direction::Up,
            Some(DOWN) => Direction::Down,
            Some(LEFT) => Direction::Left,
            Some(RIGHT) => Direction::Right,
            _ => direction,
        };
        match tx.send(direction) {
            Ok(_) => {}
            Err(error) => panic!("error on input thread: {}", error),
        }
    }
}


use std::time::Instant;

trait Render {
    fn update(&mut self, state: &GameState);
    fn render(&mut self);
}

trait GameUpdate {
    fn update(&mut self, timer: &Instant, new_direction: Option<Direction>) -> bool;
}

fn main() {
    let mut game_state = GameState::new();
    let (tx, rx) = mpsc::channel();
    let default_direction = game_state.pacman.direction;

    thread::spawn(move || {
        run_input_thread(tx, default_direction);
    });

    let mut renderer = ConsoleRenderer::new(game_state.map.width, game_state.map.height);
    let mut timer = Instant::now();

    loop {
        let direction = match rx.try_recv() {
            Ok(direction) => Some(direction),
            Err(error) => {
                match error {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => panic!("input thread has quit!"),
                }
            }
        };

        if game_state.update(&timer, direction) {
            renderer.update(&game_state);
            renderer.render();
            timer = Instant::now();
        }
    }
}
