use console::*;

use game::*;
use map::*;

const FOREGROUND_BLACK: u16 = 0;
const FOREGROUND_WHITE: u16 = (FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN |
                               FOREGROUND_BLUE) as u16;
const FOREGROUND_YELLOW: u16 = (FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN) as u16;

const BACKGROUND_WHITE: u16 = (BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN |
                               BACKGROUND_BLUE) as u16;

const BACKGROUND_YELLOW: u16 = (BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN) as u16;

const BACKGROUND_GRAY: u16 = (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE) as u16;

const EMPTY: CHAR_INFO = CHAR_INFO {
    UnicodeChar: 0,
    Attributes: 0,
};

pub struct ConsoleRenderer {
    output: ConsoleOutput,
    buffer: Vec<CHAR_INFO>,
    width: usize,
    height: usize,
    size: Size
}

impl ConsoleRenderer {
    pub fn new() -> ConsoleRenderer {
        let output = ConsoleOutput::current();
        let size = output.screen_size();
        ConsoleRenderer {
            output: output,
            buffer: vec![EMPTY; (size.width * size.height) as usize],
            width: size.width as usize,
            height: size.height as usize,
            size: size,
        }
    }
    
    fn clear_buffer(buffer: &mut [CHAR_INFO]) {
        for it in buffer {
            *it = EMPTY;
        }
    }

    fn update_map(&mut self, state: &GameState) {
        let map = &state.map;
        let buffer = &mut self.buffer;
        let center = Position::new(self.width as i32 / 2, self.height as i32 / 2);
        let map_origin = Position::new(center.x - map.width as i32 / 2, center.y - map.height as i32 / 2);

        for map_index in 0..map.cells.len() {
            let map_pos = Position::from_map_index(&map.size, map_index);
            let buffer_pos = Position::new(map_origin.x + map_pos.x, map_origin.y + map_pos.y);
            let position = buffer_pos.to_map_index(&self.size);
            match map.cells[map_index] {
                Cell::Empty => {
                    buffer[position] = CHAR_INFO {
                        UnicodeChar: ' ' as u16,
                        Attributes: BACKGROUND_BLUE as u16,
                    }
                }
                Cell::Wall => {
                    buffer[position] = CHAR_INFO {
                        UnicodeChar: ' ' as u16,
                        Attributes: BACKGROUND_GRAY,
                    }
                }
            }
        }

        let pacman = &state.pacman;
        let pacman_pos = Position::new(map_origin.x + pacman.position.x, map_origin.y + pacman.position.y);
        let position = (pacman_pos.y as usize * self.width +
                        pacman_pos.x as usize) as usize;
        buffer[position].UnicodeChar = match pacman.direction {
            Direction::Down => 'A',
            Direction::Left => 'D',
            Direction::Up => 'V',
            Direction::Right => 'C',
        } as u16;
        buffer[position].Attributes = FOREGROUND_BLACK | BACKGROUND_YELLOW;

        let gums = &state.gums;
        for gum in gums {
            let gum_pos = Position::new(map_origin.x + gum.position.x, map_origin.y + gum.position.y);
            let map_index = gum_pos.y as usize * self.width + gum_pos.x as usize;
            buffer[map_index].UnicodeChar = '.' as u16;
            buffer[map_index].Attributes |= FOREGROUND_WHITE;
        }
    }

    fn print_buffer(output: &ConsoleOutput, buffer: &[CHAR_INFO], width: usize) {
        output.write_rect(buffer, width);
    }
}

use Render;

impl Render for ConsoleRenderer {
    fn update(&mut self, state: &GameState) {
        ConsoleRenderer::clear_buffer(&mut self.buffer);
        self.update_map(&state);
    }

    fn render(&mut self) {
        ConsoleRenderer::print_buffer(&self.output, &self.buffer, self.width);
    }
}
