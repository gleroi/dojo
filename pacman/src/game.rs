use map::*;

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
        let grid = Map::new();
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
        let position = &mut pacman.position;
        match pacman.direction {
            Direction::Up => position.y = max(0, position.y - 1),
            Direction::Down => position.y = min((map.height - 1) as i32, position.y + 1),
            Direction::Left => position.x = max(0, position.x - 1),
            Direction::Right => position.x = min((map.width - 1) as i32, position.x + 1),
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