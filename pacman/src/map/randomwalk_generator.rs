extern crate rand;

use game::*;
use map::{Generator, Map, Cell, MazeCell, Door};

pub struct RandomWalkGenerator {
        size: Size,
    maze: Option<Vec<MazeCell>>,

}

use self::rand::{Rng, SeedableRng, StdRng};
use self::rand::distributions::{IndependentSample, Range};
use std::convert::From;

impl RandomWalkGenerator {
    pub fn new(width: u32, height: u32) -> RandomWalkGenerator {
        RandomWalkGenerator {
            size: Size::new(width, height),
            maze: None,
        }
    }

    fn mark_visited(&self, maze: &mut Vec<MazeCell>, index: usize) {
        maze[index].value = 1;
    }

    fn is_door_open(&self, maze: &Vec<MazeCell>, index: usize, door: &Door) -> bool {
        let cell = &maze[index];
        match *door {
            Door::North => cell.north,
            Door::South => cell.south,
            Door::East => cell.east,
            Door::West => cell.west,
        }
    }

    fn find_door(&self, maze: &Vec<MazeCell>, index: usize, door: Door, count: usize) -> Option<Door> {
        if count == 4 {
            return None;
        }
        if !self.is_door_open(maze, index, &door) {
            if let Some(other_index) = self.find_other_cell(maze, index, &door) {
                if !self.was_visited(maze, other_index) {
                    return Some(door);
                }
            }
        }
        let door_index : usize = door as usize;
        let next_door = Door::from((door_index + 1) % 4);
        return self.find_door(maze, index, next_door, count + 1);
    }

    fn find_other_cell(&self, maze: &Vec<MazeCell>, cell_index: usize, door_index: &Door) -> Option<usize> {
        let cell_pos = Position::from_map_index(&self.size, cell_index);
        let cell = &maze[cell_index];
        let other_cell_pos = match *door_index {
            Door::North if !cell.north => Position::new(cell_pos.x, cell_pos.y - 1),
            Door::South if !cell.south => Position::new(cell_pos.x, cell_pos.y + 1),
            Door::East if !cell.east => Position::new(cell_pos.x + 1, cell_pos.y),
            Door::West if !cell.west => Position::new(cell_pos.x - 1, cell_pos.y),
            _ => return None,
        };
        if other_cell_pos.x < 0 || other_cell_pos.x >= self.size.width as i32 ||
           other_cell_pos.y < 0 || other_cell_pos.y >= self.size.height as i32 {
            return None;
        }
        let other_cell_index = other_cell_pos.to_map_index(&self.size);
        return Some(other_cell_index);
    }

    fn was_visited(&self, maze: &Vec<MazeCell>, index: usize) -> bool {
        let cell = &maze[index];
        return cell.value == 1;
    }

    fn open_door(&self, maze: &mut Vec<MazeCell>, cell_index: usize, door_index: &Door) {
        let cell = &mut maze[cell_index];
        match *door_index {
            Door::North => cell.north = true,
            Door::South => cell.south = true,
            Door::East => cell.east = true,
            Door::West => cell.west = true, 
        }
    }
}

impl Generator for RandomWalkGenerator {
    fn generate(&mut self, seed: usize) {
        let size = self.size.width * self.size.height;
        let mut maze: Vec<MazeCell> = (0..size).map(|val| MazeCell::new(0)).collect();
        let arr_seed: &[_] = &[seed];
        let mut rand: StdRng = SeedableRng::from_seed(arr_seed);
        let maze_range: Range<usize> = Range::new(0, size as usize);
        let change_door_range: Range<usize> = Range::new(0, 10);
        let door_range: Range<usize> = Range::new(0, 4);

        let mut visiting : Vec<usize> = Vec::new();
        let mut cell_index = maze_range.ind_sample(&mut rand);
        
        visiting.push(cell_index);
        self.mark_visited(&mut maze, cell_index);

        let mut current_door = Door::from(door_range.ind_sample(&mut rand));

        while visiting.len() > 0 {
            if change_door_range.ind_sample(&mut rand) == 0 {
                current_door = Door::from(door_range.ind_sample(&mut rand));
            }
            let maybe_door = self.find_door(&maze, cell_index, current_door.clone(), 0);
            if let Some(door) = maybe_door {
                if let Some(other_index) = self.find_other_cell(&maze, cell_index, &door) {
                    let other_door = match door {
                        Door::North => Door::South,
                        Door::South => Door::North,
                        Door::East => Door::West,
                        Door::West => Door::East, 
                    };
                    self.mark_visited(&mut maze, other_index);
                    visiting.push(other_index);
                    self.open_door(&mut maze, cell_index, &door);
                    self.open_door(&mut maze, other_index, &other_door);
                    cell_index = other_index;
                }
            }
            else {
                let maybe_cell = visiting.pop();
                if maybe_cell == None {
                    break;
                }
                cell_index = maybe_cell.unwrap();
            }
        }
        self.maze = Some(maze);
    }

    fn extract_map(&self) -> Map {
        if let Some(ref maze) = self.maze {
            let mut map = Map::new((2 * self.size.width + 1) as usize,
                                   (2 * self.size.height + 1) as usize);
            for index in 0..maze.len() {
                let maze_pos = Position::from_map_index(&self.size, index);
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

#[cfg(test)]
mod tests {
    use map::*;

    #[test]
    fn randomwalk_small_maze_should_not_failed() {
        let mut generator = RandomWalkGenerator::new(5, 5);
        generator.generate(1);
    }
}