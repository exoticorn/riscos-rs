#![no_std]

extern crate alloc;

mod rt;
mod sys;

pub mod fs;
pub mod path;
pub mod vdu;

pub mod os {
    pub use crate::sys::os::exit;
}

pub mod prelude {
    pub use crate::{print, println};
}

pub mod io {
    pub use embedded_io::{
        BufRead, Error, ErrorKind, Read, ReadExactError, SliceWriteError, Write, WriteFmtError,
    };

    pub trait ReadExt: embedded_io::ErrorType {
        fn read_to_end(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize, Self::Error>;
    }
}
