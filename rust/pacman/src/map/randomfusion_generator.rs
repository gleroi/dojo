extern crate rand;

use game::*;
use map::{Generator, Map, Cell, MazeCell, Door};

pub struct RandomFusionGenerator {
    size: Size,
    maze: Option<Vec<MazeCell>>,
}

use self::rand::{SeedableRng, StdRng};
use self::rand::distributions::{IndependentSample, Range};

impl RandomFusionGenerator {
    pub fn new(width: u32, height: u32) -> RandomFusionGenerator {
        RandomFusionGenerator {
            size: Size::new(width, height),
            maze: None,
        }
    }

    fn is_unified(&self, maze: &Vec<MazeCell>) -> bool {
        let value = maze[0].value;
        for cell in maze {
            if cell.value != value {
                return false;
            }
        }
        return true;
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

impl Generator for RandomFusionGenerator {

    fn maze(&self) -> &Option<Vec<MazeCell>> {
        &self.maze
    }
    
    fn size(&self) -> &Size {
        &self.size
    }

    fn generate(&mut self, seed: usize) {
        let size = self.size.width * self.size.height;
        let mut maze: Vec<MazeCell> = (0..size).map(|val| MazeCell::new(val)).collect();
        let arr_seed: &[_] = &[seed];
        let mut rand: StdRng = SeedableRng::from_seed(arr_seed);
        let maze_range: Range<usize> = Range::new(0, size as usize);
        let door_range: Range<usize> = Range::new(0, 4);

        while !self.is_unified(&maze) {
            let cell_index = maze_range.ind_sample(&mut rand);
            let door = Door::from(door_range.ind_sample(&mut rand));
            if let Some(other_index) = self.find_other_cell(&maze, cell_index, &door) {
                let other_door = match door {
                    Door::North => Door::South,
                    Door::South => Door::North,
                    Door::East => Door::West,
                    Door::West => Door::East, 
                };

                let cell_value = maze[cell_index].value;
                let other_value = maze[other_index].value;

                if cell_value != other_value {
                    self.open_door(&mut maze, cell_index, &door);
                    self.open_door(&mut maze, other_index, &other_door);

                    for each_cell in &mut maze {
                        if each_cell.value == other_value {
                            each_cell.value = cell_value;
                        }
                    }
                }                
            }
        }
        self.maze = Some(maze);
    }
}

#[cfg(test)]
mod tests {
    use map::*;

    #[test]
    fn generate_small_maze_should_not_failed() {
        let mut generator = RandomFusionGenerator::new(5, 5);
        generator.generate(1);
    }
}