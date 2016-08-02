use map::*;

#[derive(PartialEq, Debug)]
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

pub struct GameState {
    pub map: Map,
    pub pacman: Pacman,
}

use std::time::{Instant, Duration};
use std::cmp::{max, min};

const DEFAULT_DIRECTION: Direction = Direction::Up;

impl GameState {
    pub fn new() -> GameState {
        let grid = Map::new_default();
        let pacman = Pacman {
            position: Position {
                x: grid.width as i32 / 2,
                y: grid.height as i32 / 2,
            },
            direction: DEFAULT_DIRECTION,
        };
        return GameState {
            map: grid,
            pacman: pacman,
        };
    }

    fn update_state(map: &mut Map, pacman: &mut Pacman, timer: &Instant) -> bool {
        let horizontal_time_slice = Duration::from_millis(64);
        let vertival_time_slice = horizontal_time_slice * 2;

        let elapsed = timer.elapsed();
        let update = match pacman.direction {
            Direction::Up | Direction::Down if elapsed > vertival_time_slice => true,
            Direction::Left | Direction::Right if elapsed > horizontal_time_slice => true,
            _ => false,
        };
        if update {
            GameState::update_pacman_position(pacman, &map);
        }
        return update;
    }

    fn update_pacman_position(pacman: &mut Pacman, map: &Map) {
        let new_position = match pacman.direction {
            Direction::Up => Position { y: max(0, pacman.position.y - 1), ..pacman.position },
            Direction::Down => Position { y: min((map.height - 1) as i32, pacman.position.y + 1), ..pacman.position },
            Direction::Left => Position { x: max(0, pacman.position.x - 1), ..pacman.position },
            Direction::Right => Position { x: min((map.width - 1) as i32, pacman.position.x + 1), ..pacman.position },
        };
        let map_position = (new_position.y * map.width as i32 + new_position.x) as usize;
        if map.cells[map_position] == Cell::Empty {
            pacman.position = new_position;
        }
    }
}

use GameUpdate;

impl GameUpdate for GameState {
    fn update(&mut self, timer: &Instant, new_direction: Option<Direction>) -> bool {
        if let Some(direction) = new_direction {
            self.pacman.direction = direction;
        }
        GameState::update_state(&mut self.map, &mut self.pacman, timer)
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
}