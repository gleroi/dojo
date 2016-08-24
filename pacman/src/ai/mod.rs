use map::*;
use game::*;

pub fn compute_path(map: &Map, monster: &Position, pacman: &Position) -> Vec<Position> {
        let mut path : Vec<Position> = Vec::new();
        let mut current = monster.clone();
        let mut visited = vec!{false; map.cells.len()};

        while &current != pacman {
            let mut next_found = false;

            visited[current.to_map_index(&map.size)] = true;

            for direction in Direction::iter() {
                let next = GameState::move_position(&map, &current, &direction);
                if map[&next] == Cell::Empty && !visited[next.to_map_index(&map.size)] {
                    path.push(current);
                    current = next;
                    next_found = true;
                    break;
                }
            }
            // no direction found and no pacman, let backtrack
            if !next_found && &current != pacman {
                if let Some(previous) = path.pop() {
                    current = previous;
                }
                else {
                    panic!("no path found!");
                }
            }
            else if &current == pacman {
                path.push(current);
                break;
            }
        }
        return path;
    }

#[cfg(test)]
mod tests {
    use map::*;
    use game::*;

    #[test]
    fn path_finder_should_find_vertical_3_long() {
        let mut  map = Map::new(5, 5);
        for y in 0..5 {
            map[&Position::new(0, y)] = Cell::Wall;
            map[&Position::new(2, y)] = Cell::Wall;
        }
        let start = Position::new(1, 1);
        let end = Position::new(1, 3);

        let path = super::compute_path(&map, &start, &end);

        println!("{:?} {:?}", path[0], path [1]);
        assert_eq!(path.len(), 3);
        
    }
}
