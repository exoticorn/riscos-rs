#![no_std]
#![no_main]

extern crate alloc;

use riscos::io::Write;
use riscos::prelude::*;
use riscos::{
    env::{self, arg},
    net,
};

#[no_mangle]
pub extern "C" fn main() {
    _ = run();
}

fn run() -> Result<(), ()> {
    let Some(addr) = env::parse_args(arg::Required(arg::String)) else {
        println!("Failed to parse args");
        return Err(());
    };
    let addr: net::SocketAddr = addr.parse().unwrap();
    println!("connecting to {}", addr);
    let mut socket = net::TcpStream::connect(addr).map_err(|e| {
        println!("Failed to connect: {:?}", e);
    })?;

    socket.write_all(b"Hello, World!").map_err(|e| {
        println!("Failde to write to socket: {:?}", e);
    })?;

    Ok(())
}
