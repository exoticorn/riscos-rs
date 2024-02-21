#![no_std]

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
}
