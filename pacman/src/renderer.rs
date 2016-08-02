use console::*;

use game::*;
use map::*;

const FOREGROUND_WHITE: u16 = (FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN |
                               FOREGROUND_BLUE) as u16;
const BACKGROUND_WHITE: u16 = (BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN |
                               BACKGROUND_BLUE) as u16;
const BACKGROUND_GRAY: u16 = (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE) as u16;

const EMPTY: CHAR_INFO = CHAR_INFO {
    UnicodeChar: 0,
    Attributes: BACKGROUND_BLUE as u16,
};

pub struct ConsoleRenderer {
    output: ConsoleOutput,
    buffer: Vec<CHAR_INFO>,
    width: usize,
}

impl ConsoleRenderer {
    pub fn new(width: usize, height: usize) -> ConsoleRenderer {
        ConsoleRenderer {
            buffer: vec![EMPTY; width * height],
            output: ConsoleOutput::current(),
            width: width,
        }
    }

    fn clear_buffer(buffer: &mut [CHAR_INFO]) {
        for it in buffer {
            *it = EMPTY;
        }
    }

    fn update_buffer(buffer: &mut [CHAR_INFO], map: &Map, pacman: &Pacman) {
        for position in 0..buffer.len() {
            match map.cells[position] {
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

        let position = (pacman.position.y as usize * map.width +
                        pacman.position.x as usize) as usize;
        buffer[position].UnicodeChar = match pacman.direction {
            Direction::Down => 'A',
            Direction::Left => 'D',
            Direction::Up => 'V',
            Direction::Right => 'C',
        } as u16;
        buffer[position].Attributes |= FOREGROUND_WHITE;
    }

    fn print_buffer(output: &ConsoleOutput, buffer: &[CHAR_INFO], width: usize) {
        output.write_rect(buffer, width);
    }
}

use Render;

impl Render for ConsoleRenderer {
    fn update(&mut self, state: &GameState) {
        ConsoleRenderer::clear_buffer(&mut self.buffer);
        ConsoleRenderer::update_buffer(&mut self.buffer, &state.map, &state.pacman);
    }

    fn render(&mut self) {
        ConsoleRenderer::print_buffer(&self.output, &self.buffer, self.width);
    }
}
