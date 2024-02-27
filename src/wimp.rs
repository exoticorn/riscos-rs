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
}

impl Drop for WIMP {
    fn drop(&mut self) {
        sys::wimp::shutdown(self.task_handle);
    }
}

pub enum Event {
    Null,
    UserMessage {
        sender: u32,
        my_ref: u32,
        message: UserMessage,
    },
}

pub enum UserMessage {
    Quit,
}
