extern crate winapi;
extern crate kernel32;

use self::kernel32::*;
use self::winapi::winnt::*;
use self::winapi::winbase::*;
use self::winapi::wincon::*;

use std::char;

#[derive(Copy, Clone)]
pub struct Console {
    in_handle: HANDLE,
    out_handle: HANDLE,
}

impl Console {
    pub fn current() -> Console {
        unsafe {
            let out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let in_handle = GetStdHandle(STD_INPUT_HANDLE);
            return Console {
                in_handle: in_handle,
                out_handle: out_handle,
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
