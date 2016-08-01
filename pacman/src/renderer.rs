use console::*;

use game::*;

const FOREGROUND_WHITE: u16 = (FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN |
                               FOREGROUND_BLUE) as u16;
const BACKGROUND_WHITE: u16 = (BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN |
                               BACKGROUND_BLUE) as u16;

const EMPTY: CHAR_INFO = CHAR_INFO {
    UnicodeChar: 0,
    Attributes: BACKGROUND_BLUE as u16,
};

pub struct ConsoleRenderer {
    output: ConsoleOutput,
    buffer: Vec<CHAR_INFO>,
}

impl ConsoleRenderer {
    pub fn new(width: usize, height: usize) -> ConsoleRenderer {
        ConsoleRenderer {
            buffer: vec![EMPTY; width * height],
            output: ConsoleOutput::current(),
        }
    }

    fn clear_buffer(buffer: &mut [CHAR_INFO]) {
        for it in buffer {
            *it = EMPTY;
        }
    }

    fn update_buffer(buffer: &mut [CHAR_INFO], grid: &Map, pacman: &Pacman) {
        for position in 0..GRID_HEIGHT * GRID_WIDTH {
            match grid.cells[position] {
                Cell::Empty => {
                    buffer[position] = CHAR_INFO {
                        UnicodeChar: ' ' as u16,
                        Attributes: BACKGROUND_BLUE as u16,
                    }
                }
                Cell::Wall => {
                    buffer[position] = CHAR_INFO {
                        UnicodeChar: ' ' as u16,
                        Attributes: BACKGROUND_WHITE,
                    }
                }
            }
        }

        let position = (pacman.position.y as usize * GRID_WIDTH +
                        pacman.position.x as usize) as usize;
        buffer[position].UnicodeChar = match pacman.direction {
            Direction::Down => 'A',
            Direction::Left => 'D',
            Direction::Up => 'V',
            Direction::Right => 'C',
        } as u16;
        buffer[position].Attributes |= FOREGROUND_WHITE;
    }

    fn print_buffer(output: &ConsoleOutput, buffer: &[CHAR_INFO]) {
        output.write_rect(buffer, GRID_WIDTH);
    }
}

use Render;

impl Render for ConsoleRenderer {
    fn update(&mut self, grid: &Map, pacman: &Pacman) {
        ConsoleRenderer::clear_buffer(&mut self.buffer);
        ConsoleRenderer::update_buffer(&mut self.buffer, grid, pacman);
    }

    fn render(&mut self) {
        ConsoleRenderer::print_buffer(&self.output, &self.buffer);
    }
}
