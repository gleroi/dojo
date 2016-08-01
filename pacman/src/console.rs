extern crate winapi;
extern crate kernel32;

use self::kernel32::*;
use self::winapi::winnt::*;
use self::winapi::winbase::*;
use self::winapi::wincon::*;

use std::char;

pub struct ConsoleInput {
    in_handle: HANDLE,
}

impl ConsoleInput {
    pub fn current() -> ConsoleInput {
        unsafe {
            let in_handle = GetStdHandle(STD_INPUT_HANDLE);
            return ConsoleInput {
                in_handle: in_handle,
            };
        }
    }

    pub fn read(&self) -> Option<char> {
        let mut input = INPUT_RECORD { EventType: 0, Event: [0; 4] };
        let mut readed : u32 = 0;

        unsafe {
            if ReadConsoleInputW(self.in_handle, &mut input, 1, &mut readed) != 0 {
                if readed > 0 && input.EventType == KEY_EVENT {
                    let key = input.KeyEvent();
                    return char::from_u32(key.UnicodeChar as u32);
                }
            }
        }
        return None;
    }
}

#[derive(Copy, Clone)]
pub struct ConsoleOutput {    
    out_handle: HANDLE,
}

use std::os::raw::c_void;
use std::ptr::null_mut;
use std::convert;

const FOREGROUND_WHITE: u16 = (FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE) as u16;

impl ConsoleOutput {
    pub fn current() -> ConsoleOutput {
        unsafe {
            let out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            return ConsoleOutput {
                out_handle: out_handle,
            };
        }
    }

    pub fn write(&self, buffer: &[char]) {
        let mut written = 0;
        unsafe {
            let ptr_buffer = buffer as *const [char] as *mut c_void; 
            if WriteConsoleW(self.out_handle, ptr_buffer, buffer.len() as u32, &mut written, null_mut()) == 0 {
                panic!("console output: could not write: {0}", GetLastError());
            }
        }
    }

    fn screen_info(&self) -> CONSOLE_SCREEN_BUFFER_INFO {
        let mut info = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD { X: 0, Y: 0 },
            dwCursorPosition: COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT { Top:0, Left: 0, Bottom:0, Right: 0 },
            dwMaximumWindowSize: COORD { X: 0, Y: 0 },
        };
        unsafe {
            GetConsoleScreenBufferInfo(self.out_handle, &mut info);
        }

        return info;
    }

    pub fn write_rect(&self, buffer: &[char], width: usize) {
        if buffer.len() == 0 || buffer.len() % width != 0 {
            panic!("write_rect: buffer length ({}) in not divisible by {}", buffer.len(), width);
        }
        let len = buffer.len();
        let mut data: Vec<CHAR_INFO> = Vec::with_capacity(len);

        for character in buffer {
            let cell = CHAR_INFO { UnicodeChar: *character as u16, Attributes: BACKGROUND_BLUE as u16 | FOREGROUND_WHITE as u16 };
            data.push(cell);
        }

        let iwidth : i16 = width as i16;
        let iheight = len as i16 / iwidth;
        let ptr_data : &[CHAR_INFO] = data.as_ref();

        let info = self.screen_info();
        let top_center = info.srWindow.Top + (info.srWindow.Bottom - info.srWindow.Top) / 2;
        let left_center = info.srWindow.Left + (info.srWindow.Right - info.srWindow.Left) / 2;

        let mut out_rect = &mut SMALL_RECT { 
            Top: top_center - iheight / 2, Left: left_center / 2, 
            Bottom: top_center + iheight / 2, Right: left_center + iwidth / 2 };
        unsafe {
            WriteConsoleOutputW(self.out_handle, 
                ptr_data as *const [CHAR_INFO] as *const CHAR_INFO, 
                COORD { X: iwidth, Y: iheight },
                COORD { X: 0, Y: 0 },
                out_rect);
        }
    }
}
