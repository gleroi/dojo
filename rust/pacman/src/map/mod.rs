
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
}

use game::*;

pub struct Map {
    pub cells: Vec<Cell>,
    pub size: Size,
    pub width: usize,
    pub height: usize,
}

impl Map {

    pub fn new(width: usize, height: usize) -> Map {
        let map = Map {
            cells: vec![Cell::Empty; width * height],
            width: width,
            height: height,
            size: Size {
                width: width as u32,
                height: height as u32,
            },
        };
        return map;
    }
}

use std::ops::{Index, IndexMut};

impl<'a> Index<&'a Position> for Map {
    type Output = Cell;
    fn index(&self, index: &Position) -> &Cell {
        return &self.cells[index.to_map_index(&self.size)];
    }
}


impl<'a> IndexMut<&'a Position> for Map {
    fn index_mut(&mut self, index: &Position) -> &mut Cell {
        return &mut self.cells[index.to_map_index(&self.size)];
    }
}

#[derive(Default)]
pub struct MazeCell {
    value: u32,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl MazeCell {
    fn new(val: u32) -> MazeCell {
        MazeCell {
            value: val,
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }
}

#[derive(Clone)]
enum Door {
    North,
    West,
    South,
    East,
}

impl From<usize> for Door {
    fn from(index: usize) -> Door {
        match index {
            0 => Door::North,
            1 => Door::West,
            2 => Door::South,
            3 => Door::East,
            _ => panic!("unknown door index {}", index),
        }
    }
}

pub trait Generator {
    fn generate(&mut self, seed: usize);

    fn maze(&self) -> &Option<Vec<MazeCell>>;
    fn size(&self) -> &Size;

    fn extract_map(&self) -> Map {
        if let &Some(ref maze) = self.maze() {
            let mut map = Map::new((2 * self.size().width + 1) as usize,
                                   (2 * self.size().height + 1) as usize);
            for index in 0..maze.len() {
                let maze_pos = Position::from_map_index(self.size(), index);
                let map_pos = Position::new(2 * maze_pos.x + 1, 2 * maze_pos.y + 1);
                let maze_cell = &maze[index];
                if !maze_cell.north {
                    let mut door_pos = &map_pos + &Position::new(0, -1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(-1, -1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(1, -1);
                    map[&door_pos] = Cell::Wall;
                }
                if !maze_cell.south {
                    let mut door_pos = &map_pos + &Position::new(0, 1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(-1, 1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(1, 1);
                    map[&door_pos] = Cell::Wall;
                }
                if !maze_cell.east {
                    let mut door_pos = &map_pos + &Position::new(1, 0);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(1, 1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(1, -1);
                    map[&door_pos] = Cell::Wall;
                }
                if !maze_cell.west {
                    let mut door_pos = &map_pos + &Position::new(-1, 0);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(-1, 1);
                    map[&door_pos] = Cell::Wall;
                    door_pos = &map_pos + &Position::new(-1, -1);
                    map[&door_pos] = Cell::Wall;
                }
            }
            return map;
        
        }
        panic!("call generate before extract map");
    }
}

mod randomwalk_generator;
mod randomfusion_generator;
pub use self::randomfusion_generator::*;
pub use self::randomwalk_generator::*;