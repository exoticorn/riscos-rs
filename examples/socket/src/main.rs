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
    let _socket =
        net::TcpStream::connect(net::SocketAddr::new([127, 0, 0, 1], 3000)).map_err(|e| {
            println!("Failed to connect: {:?}", e);
        })?;

    Ok(())
}
