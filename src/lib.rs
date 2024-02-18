#![no_std]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

global_asm!{
    ".section .text.entry",
    "ldr sp, =_stack_end",
    "bl main",
    "swi 0x11"
}

#[macro_export]
/// send a formatted string to VDU
macro_rules! print {
    ($args:tt) => {
        core::fmt::write(&mut $crate::VDUWriter, core::format_args!($args));
    }
}

#[macro_export]
/// send a formatted string + newline to VDU
macro_rules! println {
    ($($args:tt),*) => {
        _ = core::fmt::write(&mut $crate::VDUWriter, core::format_args!($($args),*));
        $crate::write(b"\r\n");
    }
}

/// Write formatted output to VDU
pub struct VDUWriter;

impl core::fmt::Write for VDUWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_str(s);
        Ok(())
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    println!("Panic: {}", panic);
    exit();
}

/// exits the application
pub fn exit() -> ! {
    unsafe {
        asm!(
            "swi 0x11",
            options(noreturn, nostack)
        );
    }
}

/// writes a byte string to VDU
pub fn write(s: &[u8]) {
    unsafe {
        asm!(
            "swi 0x46",
            in("r0") s.as_ptr(),
            in("r1") s.len(),
            options(nostack)
        );
    }
}

/// writes a character byte to VDU
pub fn write_c(c: u8) {
    unsafe {
        asm!(
            "swi 0x00",
            in("r0") c,
            options(nostack)
        );
    }
} 

/// writes the characters from a string to VDU
///
/// output for characters outside the range 0..256 is undefined
pub fn write_str(s: &str) {
    for c in s.chars() {
        write_c(c as u8);
    }
}
