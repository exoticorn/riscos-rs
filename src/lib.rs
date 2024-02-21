#![no_std]

mod rt;
mod sys;

pub mod vdu;

pub mod os {
    pub use crate::sys::os::exit;
}

pub mod prelude {
    pub use crate::{print, println};
}
