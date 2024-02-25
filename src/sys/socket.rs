use core::arch::asm;

pub const STREAM: i32 = 1;
// pub const DGRAM: i32 = 2;
pub const AF_INET: i32 = 2;

pub fn creat(domain: i32, type_: i32, protocol: i32) -> u32 {
    let mut handle;
    unsafe {
        asm!(
            "swi 0x41200",
            in("r0") domain,
            in("r1") type_,
            in("r2") protocol,
            lateout("r0") handle,
            options(nostack)
        );
    }
    handle
}

pub fn close(socket: u32) {
    unsafe {
        asm!(
            "swi 0x41210",
            in("r0") socket,
            lateout("r0") _,
            options(nostack)
        );
    }
}

#[repr(C)]
pub struct AddrIpv4 {
    pub family: i16,
    pub port: u16,
    pub ip: u32,
    pub pad0: u32,
    pub pad1: u32,
}

pub unsafe fn connect(socket: u32, addr: *const u8, addr_size: u32) -> bool {
    let mut success: u32;
    asm!(
        "swi 0x61204",
        "movvs r1, #0",
        in("r0") socket,
        in("r1") addr,
        in("r2") addr_size,
        lateout("r0") _,
        lateout("r1") success,
        options(nostack)
    );
    success != 0
}

pub unsafe fn write(socket: u32, buffer: *const u8, size: usize) -> (usize, bool) {
    let mut success: u32;
    let mut written: usize;
    asm!(
        "swi 0x61214",
        "movvs r1, #0",
        in("r0") socket,
        in("r1") buffer,
        in("r2") size,
        lateout("r0") written,
        lateout("r1") success,
        options(nostack)
    );
    (written, success != 0)
}

pub unsafe fn read(socket: u32, buffer: *mut u8, size: usize) -> (usize, bool) {
    let mut success: u32;
    let mut read: usize;
    asm!(
        "swi 0x61213",
        "movvs r1, #0",
        in("r0") socket,
        in("r1") buffer,
        in("r2") size,
        lateout("r0") read,
        lateout("r1") success,
        options(nostack)
    );
    (read, success != 0)
}

#[repr(C)]
pub struct HostEnt {
    pub host_name: *const u8,
    pub aliases: *const *const u8,
    pub address_type: i32,
    pub address_size: usize,
    pub addresses: *const *const u32,
}

pub unsafe fn get_host_by_name(host: *const u8) -> Option<*const HostEnt> {
    let mut entry: *const HostEnt;
    asm!(
        "swi 0x46000",
        in("r1") host,
        lateout("r1") entry,
        options(nostack)
    );
    if entry.is_null() {
        None
    } else {
        Some(entry)
    }
}
