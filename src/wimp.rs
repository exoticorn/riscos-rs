use core::ptr;

use crate::{helper::ROString, sys};

pub struct WIMP {
    task_handle: u32,
    poll_buffer: [u32; 64],
}

impl WIMP {
    pub fn new(name: &str) -> WIMP {
        let (_, task_handle) =
            unsafe { sys::wimp::initialize(528, ROString::from_str(name).as_ptr(), ptr::null()) };
        WIMP {
            task_handle,
            poll_buffer: [0; 64],
        }
    }

    pub fn poll(&mut self) -> Event {
        loop {
            let (reason_code, _) =
                unsafe { sys::wimp::poll(0, self.poll_buffer.as_mut_ptr(), ptr::null()) };
            match reason_code {
                0 => return Event::Null,
                6 => {
                    let x = self.poll_buffer[0] as i32;
                    let y = self.poll_buffer[1] as i32;
                    let button = self.poll_buffer[2];
                    let window = self.poll_buffer[3] as i32;
                    let icon = self.poll_buffer[4] as i32;
                    return Event::MouseClick {
                        x,
                        y,
                        button,
                        icon: Icon { window, icon },
                    };
                }
                17 => {
                    let sender = self.poll_buffer[1];
                    let my_ref = self.poll_buffer[2];
                    let message = match self.poll_buffer[4] {
                        0 => Some(UserMessage::Quit),
                        _ => None,
                    };
                    if let Some(message) = message {
                        return Event::UserMessage {
                            sender,
                            my_ref,
                            message,
                        };
                    }
                }
                _ => (),
            }
        }
    }

    pub fn create_icon(
        &self,
        placement: IconPlacement,
        rect: Rect,
        flags: IconFlags,
        data: IconData,
    ) -> Icon {
        let mut block = [0i32; 9];
        block[1] = rect.min_x;
        block[2] = rect.min_y;
        block[3] = rect.max_x;
        block[4] = rect.max_y;
        let priority = 0;
        let window;
        match placement {
            IconPlacement::IconBar(pos) => match pos {
                IconBarPosition::Left => {
                    block[0] = -2;
                    window = -2;
                }
                IconBarPosition::Right => {
                    block[0] = -1;
                    window = -2;
                }
            },
        }

        fn write_direct_text(data: &mut [i32], s: &str) {
            let mut buffer = [0u8; 12];
            for (i, c) in s.chars().enumerate() {
                buffer[i] = c as u8;
            }
            for (i, w) in data.iter_mut().enumerate() {
                *w = i32::from_le_bytes(buffer[i * 4..i * 4 + 4].try_into().unwrap());
            }
        }

        let mut flags = flags.0;
        match data {
            IconData::DirectText(s) => {
                write_direct_text(&mut block[6..], s);
                flags |= 1 << 0;
            }
            IconData::DirectSprite(s) => {
                write_direct_text(&mut block[6..], s);
                flags |= 1 << 1;
            }
        }
        block[5] = flags as i32;

        let icon = unsafe { sys::wimp::create_icon(priority, block.as_ptr()) };
        Icon { window, icon }
    }
}

impl Drop for WIMP {
    fn drop(&mut self) {
        sys::wimp::shutdown(self.task_handle);
    }
}

pub enum Event {
    Null,
    MouseClick {
        x: i32,
        y: i32,
        button: u32,
        icon: Icon,
    },
    UserMessage {
        sender: u32,
        my_ref: u32,
        message: UserMessage,
    },
}

pub enum UserMessage {
    Quit,
}

pub enum IconPlacement {
    IconBar(IconBarPosition),
}

pub enum IconBarPosition {
    Left,
    Right,
}

pub struct Rect {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rect {
    pub fn with_size(width: i32, height: i32) -> Rect {
        Rect {
            min_x: 0,
            min_y: 0,
            max_x: width,
            max_y: height,
        }
    }
}

pub struct IconFlags(u32);

impl Default for IconFlags {
    fn default() -> Self {
        IconFlags(0)
    }
}

pub enum IconData<'a> {
    DirectText(&'a str),
    DirectSprite(&'a str),
}

#[derive(PartialEq, Eq)]
pub struct Icon {
    window: i32,
    icon: i32,
}

impl Icon {
    pub fn delete(self) {
        sys::wimp::delete_icon(self.window, self.icon);
    }
}
