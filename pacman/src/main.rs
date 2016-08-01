mod console;

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Wall,
}

const GRID_WIDTH : usize = 52;
const GRID_HEIGHT : usize = 26;

struct Grid {
    cells: [Cell; GRID_HEIGHT * GRID_WIDTH],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: [Cell::Empty; GRID_HEIGHT * GRID_WIDTH],
        }
    }
}

struct Position {x: i32, y: i32}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Pacman {
    position: Position,
    direction: Direction,
}

struct Gum {
    position: Position,
}

fn clear_buffer(buffer: &mut [char]) {
    for it in buffer {
        *it = ' ';
    }
}

fn update_buffer(buffer: &mut [char], grid: &Grid, pacman: &Pacman) {
    for position in 0..GRID_HEIGHT * GRID_WIDTH {
        match grid.cells[position] {
            Cell::Empty => buffer[position] = ' ',
            Cell::Wall => buffer[position] = '|',
        }
    }

    let position = (pacman.position.y as usize * GRID_WIDTH + pacman.position.x as usize) as usize;
    buffer[position] = match pacman.direction {
        Direction::Down => '\u{1F601}',
        Direction::Left => '3',
        Direction::Up => 'V',
        Direction::Right => 'K',
    }
}

fn print_buffer(output: &ConsoleOutput, buffer: &[char]) {
    output.write_rect(buffer, GRID_WIDTH);
}

use std::thread;
use std::sync::mpsc;
use console::*;

const DEFAULT_DIRECTION : Direction = Direction::Up;

fn run_input_thread(tx: mpsc::Sender<Direction>) {
    let console = ConsoleInput::current();
    let mut direction = DEFAULT_DIRECTION;
    loop {

        let val = console.read();

        const UP : char = 'z';
        const DOWN : char = 's';
        const LEFT : char = 'q';
        const RIGHT : char = 'd';

        direction = match val {
            Some(UP) => Direction::Up,
            Some(DOWN) => Direction::Down,
            Some(LEFT) => Direction::Left,
            Some(RIGHT) => Direction::Right,
            _ => direction,
        };
        match tx.send(direction) {
            Ok(_) => {},
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

fn main() {

    let (tx,rx) = mpsc::channel();
    
    thread::spawn(|| { run_input_thread(tx); });

    let mut grid = Grid::new();
    let mut pacman = Pacman {
        position: Position { x: GRID_WIDTH as i32 / 2, y: GRID_HEIGHT as i32 / 2},
        direction: DEFAULT_DIRECTION,
    };
    let mut buffer = [0 as char; GRID_WIDTH * GRID_HEIGHT];
    let output = ConsoleOutput::current();
    let mut timer = Instant::now();

    loop {
        match rx.try_recv() {
            Ok(direction) => pacman.direction = direction,
            Err(error) => match error {
                mpsc::TryRecvError::Empty => { /*nothing* */ },
                mpsc::TryRecvError::Disconnected => panic!("input thread has quit!"),
            }
        }
        clear_buffer(&mut buffer);
        update_state(&mut grid, &mut pacman, &mut timer);
        update_buffer(&mut buffer, &grid, &pacman);
        print_buffer(&output, &buffer);
    }
}
