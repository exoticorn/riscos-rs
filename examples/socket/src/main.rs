#![no_std]
#![no_main]

extern crate alloc;

use riscos::net;
use riscos::prelude::*;

#[no_mangle]
pub extern "C" fn main() {
    _ = run();
}

fn run() -> Result<(), ()> {
    let host = "example.com";
    let addr: net::IpAddr = host.parse().unwrap();
    println!("{}: {}", host, addr);
    let _socket = net::TcpStream::connect(net::SocketAddr::new(addr, 80)).map_err(|e| {
        println!("Failed to connect: {:?}", e);
    })?;

    Ok(())
}
