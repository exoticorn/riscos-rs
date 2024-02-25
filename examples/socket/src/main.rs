#![no_std]
#![no_main]

extern crate alloc;

use riscos::io::{Read, Write};
use riscos::prelude::*;
use riscos::{
    env::{self, arg},
    net, vdu,
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
        println!("Failed to write to socket: {:?}", e);
    })?;

    let mut buffer = [0u8; 256];
    loop {
        let count = socket.read(&mut buffer).map_err(|e| {
            println!("Failed to read from socket: {:?}", e);
        })?;
        if count == 0 {
            break;
        }
        vdu::write(&buffer[..count]);
    }

    Ok(())
}
