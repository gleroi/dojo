mod console;
mod game;
mod renderer;


use std::thread;
use std::sync::mpsc;

use console::*;
use renderer::*;
use game::*;

const DEFAULT_DIRECTION: Direction = Direction::Up;

fn run_input_thread(tx: mpsc::Sender<Direction>) {
    let console = ConsoleInput::current();
    let mut direction = DEFAULT_DIRECTION;
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


use std::time::{Instant, Duration};

trait Render {
    fn update(&mut self, grid: &Map, pacman: &Pacman);
    fn render(&mut self);
}

fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(|| {
        run_input_thread(tx);
    });

    let mut grid = Map::new();
    let mut pacman = Pacman {
        position: Position {
            x: GRID_WIDTH as i32 / 2,
            y: GRID_HEIGHT as i32 / 2,
        },
        direction: DEFAULT_DIRECTION,
    };

    let mut renderer = ConsoleRenderer::new(GRID_WIDTH, GRID_HEIGHT);
    let mut timer = Instant::now();

    loop {
        match rx.try_recv() {
            Ok(direction) => pacman.direction = direction,
            Err(error) => {
                match error {
                    mpsc::TryRecvError::Empty => {
                        // nothing*
                    }
                    mpsc::TryRecvError::Disconnected => panic!("input thread has quit!"),
                }
            }
        }

        if update_state(&mut grid, &mut pacman, &timer) {
            renderer.update(&grid, &pacman);
            renderer.render();
            timer = Instant::now();
        }
    }
}
