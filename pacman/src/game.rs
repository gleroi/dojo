
#[derive(Copy, Clone)]
pub enum Cell {
    Empty,
    Wall,
}

pub const GRID_WIDTH: usize = 52;
pub const GRID_HEIGHT: usize = 26;

pub struct Grid {
    pub cells: [Cell; GRID_HEIGHT * GRID_WIDTH],
}

impl Grid {
    pub fn new() -> Grid {
        Grid { cells: [Cell::Empty; GRID_HEIGHT * GRID_WIDTH] }
    }
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Pacman {
    pub position: Position,
    pub direction: Direction,
}

pub struct Gum {
    position: Position,
}
