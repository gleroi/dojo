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


fn update_state(grid: &mut Grid, pacman: &mut Pacman, timer: &mut Instant) {
    let horizontal_time_slice = Duration::from_millis(64);
    let vertival_time_slice = horizontal_time_slice * 2;

    let elapsed = timer.elapsed();
    let update = match pacman.direction {
        Direction::Up | Direction::Down if elapsed > vertival_time_slice => true,
        Direction::Left | Direction::Right if elapsed > horizontal_time_slice => true,
        _ => false,
    };
    if update {
        update_pacman_position(pacman, &grid);

        *timer = Instant::now();
    }
}

use std::cmp::{max, min};

fn update_pacman_position(pacman: &mut Pacman, grid: &Grid) {
    let position = &mut pacman.position;
    match pacman.direction {
        Direction::Up => position.y = max(0, position.y - 1),
        Direction::Down => position.y = min((GRID_HEIGHT - 1) as i32, position.y + 1),
        Direction::Left => position.x = max(0, position.x - 1),
        Direction::Right => position.x = min((GRID_WIDTH - 1) as i32, position.x + 1),
    }
}

use std::time::{Instant, Duration};

trait Render {
    fn update(&mut self, grid: &Grid, pacman: &Pacman);
    fn render(&mut self);
}

fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(|| {
        run_input_thread(tx);
    });

    let mut grid = Grid::new();
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
        update_state(&mut grid, &mut pacman, &mut timer);
        
        renderer.update(&grid, &pacman);
        renderer.render();
    }
}
