#![no_std]
#![no_main]

extern crate alloc;

use riscos::wimp::{
    Event, IconBarPosition, IconData, IconFlags, IconPlacement, Rect, UserMessage, WIMP,
};

#[no_mangle]
pub extern "C" fn main() {
    let mut wimp = WIMP::new("RustApp");
    let taskbar_icon = wimp.create_icon(
        IconPlacement::IconBar(IconBarPosition::Right),
        Rect::with_size(68, 68),
        IconFlags::default(),
        IconData::DirectSprite("application"),
    );
    loop {
        match wimp.poll() {
            Event::MouseClick { icon, .. } if icon == taskbar_icon => break,
            Event::UserMessage {
                message: UserMessage::Quit,
                ..
            } => break,
            _ => (),
        }
    }
}
