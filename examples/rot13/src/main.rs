#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use riscos::io::{ReadExt as _, Write as _};
use riscos::prelude::*;
use riscos::{
    env::{self, arg},
    fs, os,
};

#[no_mangle]
pub extern "C" fn main() {
    let Some((input, output)) = env::parse_args((
        arg::Required(arg::Named(b"input", arg::GSTrans)),
        arg::Required(arg::Named(b"output", arg::GSTrans)),
    )) else {
        println!("failed to parse arguments");
        os::exit();
    };

    println!("{} -> {}", &input, &output);

    let mut content = Vec::new();
    fs::File::open(&input)
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

    fs::File::create(&output)
        .unwrap()
        .write_all(&content)
        .unwrap();
    fs::set_type(&output, 0xfff).unwrap();

    println!("Encrypted {} bytes of text!", content.len());
}
