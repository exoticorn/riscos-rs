use crate::sys;

#[macro_export]
/// send a formatted string to VDU
macro_rules! print {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::vdu::VDUWriter, core::format_args!($($args)*));
    }
}

#[macro_export]
/// send a formatted string + newline to VDU
macro_rules! println {
    ($($args:tt)*) => {
        _ = core::fmt::write(&mut $crate::vdu::VDUWriter, core::format_args!($($args)*));
        $crate::vdu::write(b"\r\n");
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

/// writes a byte string to VDU
pub fn write(s: &[u8]) {
    unsafe {
        sys::os::write_n(s.as_ptr(), s.len());
    }
}

pub use sys::os::write_c;

/// writes the characters from a string to VDU
///
/// output for characters outside the range 0..256 is undefined
pub fn write_str(s: &str) {
    for c in s.chars() {
        if c == 10 as char {
            write_c(13);
        }
        write_c(c as u8);
    }
}
