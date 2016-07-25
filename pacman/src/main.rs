#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Wall,
}

const GRID_WIDTH : usize = 25;
const GRID_HEIGHT : usize = 25;

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

fn clear_buffer(buffer: &mut [u8]) {
    for it in buffer {
        *it = 0;
    }
}

fn update_buffer(buffer: &mut [u8], grid: &Grid, pacman: &Pacman) {
    for position in 0..GRID_HEIGHT * GRID_WIDTH {
        match grid.cells[position] {
            Cell::Empty => buffer[position] = ' ' as u8,
            Cell::Wall => buffer[position] = '|' as u8,
        }
    }

    let position = (pacman.position.y as usize * GRID_WIDTH + pacman.position.y as usize) as usize;
    buffer[position] = match pacman.direction {
        Direction::Down => 'W' as u8,
        Direction::Left => '3' as u8,
        Direction::Up => 'V' as u8,
        Direction::Right => 'K' as u8,
    }

}

use std::io::Write;

fn print_buffer(buffer: &[u8]) {
    let mut out = std::io::stdout();
    for y in 0..GRID_HEIGHT {
        out.write_all(&buffer[y*GRID_WIDTH..(y+1) * GRID_WIDTH]);
        out.write(&['\n' as u8; 1]);
    }
}

use std::io::Read;

fn main() {
    let grid = Grid::new();
    let pacman = Pacman {
        position: Position { x: GRID_WIDTH as i32 / 2, y: GRID_HEIGHT as i32 / 2},
        direction: Direction::Up,
    };


    let mut buffer = [0 as u8; GRID_WIDTH * GRID_HEIGHT];
    loop {
        let mut data = [0 as u8; 1];
        let mut input = std::io::stdin();
        input.read_exact(&mut data);

        println!("{}", data[0]);

        clear_buffer(&mut buffer);
        update_buffer(&mut buffer, &grid, &pacman);
        print_buffer(&buffer);
    }
}
