extern crate rand;

use self::rand::{Rng, SeedableRng, StdRng};
use self::rand::distributions::{IndependentSample, Range};

use std;

use map::*;

#[derive(PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {x: x, y: y}
    }

    pub fn from_map_index(map: &Size, index: usize) -> Position {
        Position {
            x: (index % map.width as usize) as i32,
            y: (index / map.width as usize) as i32,
        }
    }

    pub fn to_map_index(&self, map: &Size) -> usize {
        return self.y as usize * map.width as usize + self.x as usize;
    }
}

use std::ops::Add;

impl <'a,'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, rhs: &'b Position) -> Position {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Size {
        Size { width: width, height: height }
    }
}


#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use std::slice::Iter;

impl Direction {
    fn iter() -> Iter<'static, Direction> {
        static VALUES : [Direction; 4] = [Direction::Up,Direction::Right, Direction::Down, Direction::Left];
        return VALUES.into_iter();
    }
}

pub struct Pacman {
    pub position: Position,
    pub direction: Direction,
}

const GUM_SCORE : i32 = 10;
pub struct Gum {
    pub position: Position,
}

pub struct Monster {
    pub position: Position,
}

pub struct GameState {
    pub map: Map,
    pub pacman: Pacman,
    pub gums: Vec<Gum>,
    pub monsters: Vec<Monster>,
    pub score: i32,
}

use std::time::{Instant, Duration};
use std::cmp::{max, min};

const DEFAULT_DIRECTION: Direction = Direction::Up;

fn find_near_free_position(map: &Map, monsters: &Vec<Monster>, initial: &Position) -> Position {
    let initial_index = initial.to_map_index(&map.size);
    let length = map.cells.len();
    for offset in 0..length {
        let index = (initial_index + offset) % length;
        let position = Position::from_map_index(&map.size, index); 
        if map[&position] == Cell::Empty && monsters.iter().all(|other| other.position != position) {
            return position;
        }
    }
    panic!("no available free position");
}

impl GameState {
    pub fn new(seed: usize) -> GameState {
        let generator : &mut Generator = &mut RandomWalkGenerator::new(28, 13);
        generator.generate(seed);
        let grid = generator.extract_map();
        let mut monsters : Vec<Monster> = Vec::new();

        let pacman = Pacman {
            position: find_near_free_position(&grid, &monsters, &Position::new(grid.size.width  as i32 / 2, grid.size.height as i32 / 2 )),
            direction: DEFAULT_DIRECTION,
        };
        let mut gums : Vec<Gum> = Vec::new();
        for (index, &cell) in grid.cells.iter().enumerate() {
            if cell == Cell::Empty && index % 3 == 0 {
                gums.push(Gum { position: Position::from_map_index(&grid.size, index) });
            }
        }

        let arr_seed: &[_] = &[seed];
        let mut rand: StdRng = SeedableRng::from_seed(arr_seed);
        let x_range: Range<i32> = Range::new(0, grid.width as i32);
        let y_range: Range<i32> = Range::new(0, grid.height as i32);
        
        for _ in 0..4 {
            let initial = Position::new(x_range.ind_sample(&mut rand), y_range.ind_sample(&mut rand));
            let position = find_near_free_position(&grid, &monsters, &initial);
            monsters.push(Monster { position: position });
        }

        return GameState {
            map: grid,
            pacman: pacman,
            gums: gums,
            monsters: monsters,
            score: 0,
        };
    }

    fn move_position(map: &Map, position: &Position, direction: &Direction) -> Position {
        match *direction {
            Direction::Up => Position { y: max(0, position.y - 1), ..*position },
            Direction::Down => Position { y: min((map.height - 1) as i32, position.y + 1), ..*position },
            Direction::Left => Position { x: max(0, position.x - 1), ..*position },
            Direction::Right => Position { x: min((map.width - 1) as i32, position.x + 1), ..*position },
        }
    }

    fn update_pacman_position(pacman: &mut Pacman, map: &Map) {
        let new_position = GameState::move_position(map, &pacman.position, &pacman.direction);
        let map_position = new_position.to_map_index(&map.size);
        if map.cells[map_position] == Cell::Empty {
            pacman.position = new_position;
        }
    }

    fn update_gums(gums: &mut Vec<Gum>, pacman: &Pacman) -> bool {
        let len = gums.len();        
        gums.retain(|ref gum| gum.position != pacman.position);
        return len != gums.len();
    }

    fn update_monsters_positions(monsters: &mut Vec<Monster>, map: &Map) {
        for monster in monsters.iter_mut() {
            for direction in Direction::iter() {
                let position = GameState::move_position(map, &monster.position, &direction);
                if map[&position] == Cell::Empty {
                    monster.position = position;
                    break;
                }
            }
        }
    }
    
}

use GameUpdate;

impl GameUpdate for GameState {
    fn update(&mut self, timer: &Instant, new_direction: Option<Direction>) -> bool {
        if let Some(direction) = new_direction {
            self.pacman.direction = direction;
        }

        let horizontal_time_slice = Duration::from_millis(72);
        let vertival_time_slice = horizontal_time_slice * 2;
        let elapsed = timer.elapsed();
        let update = match self.pacman.direction {
            Direction::Up | Direction::Down if elapsed > vertival_time_slice => true,
            Direction::Left | Direction::Right if elapsed > horizontal_time_slice => true,
            _ => false,
        };

        if update {
            GameState::update_pacman_position(&mut self.pacman, &self.map);
            GameState::update_monsters_positions(&mut self.monsters, &self.map);
            if GameState::update_gums(&mut self.gums, &self.pacman) {
                self.score += GUM_SCORE;
            }
        }
        return update;
    }
}

#[cfg(test)]
mod tests {
    use map::*;
    use super::*;

    #[test]
    fn pacman_cannot_leave_map() {
        let map = Map::new(1, 1);
        let mut pacman = Pacman {
            position: Position { x: 0, y: 0 },
            direction: Direction::Up,
        };

        pacman.direction = Direction::Up;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 0, y: 0 });

        pacman.direction = Direction::Down;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 0, y: 0 });

        pacman.direction = Direction::Left;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 0, y: 0 });

        pacman.direction = Direction::Right;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 0, y: 0 });
    }

    #[test]
    fn pacman_cannot_traverse_walls() {
        let mut map = Map::new(3, 3);
        for index in 0..9 {
            if index != 4 {
                map.cells[index] = Cell::Wall;
            }
        }
        let mut pacman = Pacman { position: Position { x: 1, y: 1}, direction: Direction::Up };

        pacman.direction = Direction::Up;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 1, y: 1 });

        pacman.direction = Direction::Down;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 1, y: 1 });

        pacman.direction = Direction::Left;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 1, y: 1 });

        pacman.direction = Direction::Right;
        GameState::update_pacman_position(&mut pacman, &map);
        assert_eq!(pacman.position, Position { x: 1, y: 1 });
    }

    #[test]
    fn pacman_eat_gum_when_on_it() {
        let mut map = Map::new(3,3);
        let mut pacman = Pacman { position: Position {x:1, y: 1}, direction: Direction::Up };
        let mut gums : Vec<Gum> = Vec::with_capacity(8);
        for index in 0..9 {
            if index != 4 {
                gums.push(Gum { position: Position::from_map_index(&map.size, index) });
            }
        }

        let gums_count = gums.len();

        pacman.position = Position::new(1, 1);
        GameState::update_gums(&mut gums, &pacman);
        assert_eq!(gums_count, gums.len());

        pacman.position = Position::new(1, 0);
        GameState::update_gums(&mut gums, &pacman);
        assert_eq!(gums_count - 1, gums.len());

        pacman.position = Position::new(1, 2);
        GameState::update_gums(&mut gums, &pacman);
        assert_eq!(gums_count - 2, gums.len());

        pacman.position = Position::new(0, 1);
        GameState::update_gums(&mut gums, &pacman);
        assert_eq!(gums_count - 3, gums.len());

        pacman.position = Position::new(2, 1);
        GameState::update_gums(&mut gums, &pacman);
        assert_eq!(gums_count - 4, gums.len());
    }
}