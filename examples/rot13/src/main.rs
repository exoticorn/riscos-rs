#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use riscos::fs;
use riscos::io::{ReadExt as _, Write as _};
use riscos::prelude::*;

#[no_mangle]
pub extern "C" fn main() {
    let mut content = Vec::new();
    fs::File::open("RAM:$.ReadMe")
        .unwrap()
        .read_to_end(&mut content)
        .unwrap();

    for c in content.iter_mut() {
        if *c >= b'A' && *c <= b'Z' {
            *c = (*c - b'A' + 13) % 26 + b'A';
        } else if *c >= b'a' && *c <= b'z' {
            *c = (*c - b'a' + 13) % 26 + b'a';
        }
    }

    fs::File::create("RAM:$.ReadMe_r13")
        .unwrap()
        .write_all(&content)
        .unwrap();
    fs::set_type("RAM:$.ReadMe_r13", 0xfff).unwrap();

    println!("Encrypted {} bytes of text!", content.len());
}
