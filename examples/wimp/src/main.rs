#![no_std]
#![no_main]

extern crate alloc;

use riscos::wimp::{Event, UserMessage, WIMP};

#[no_mangle]
pub extern "C" fn main() {
    let mut wimp = WIMP::new("RustApp");
    loop {
        match wimp.poll() {
            Event::UserMessage {
                message: UserMessage::Quit,
                ..
            } => break,
            _ => (),
        }
    }
}
