use core::{arch::asm, ffi::c_char};

pub struct Env {
    pub command: *const u8,
    pub ram_limit: *const u8,
}

pub fn get_env() -> Env {
    let mut env = Env {
        command: core::ptr::null(),
        ram_limit: core::ptr::null(),
    };
    unsafe {
        asm!(
            "swi 0x10",
            out("r0") env.command,
            out("r1") env.ram_limit,
            out("r2") _,
            options(nostack, pure, readonly)
        );
    }
    env
}

pub unsafe fn heap_initialise(heap: *mut u8, size: usize) {
    asm!(
        "swi 0x1d",
        in("r0") 0,
        in("r1") heap,
        in("r3") size,
        options(nostack)
    );
}

pub unsafe fn heap_resize(heap: *mut u8, amount: isize) {
    asm!(
        "swi 0x1d",
        in("r0") 5,
        in("r1") heap,
        in("r3") amount,
        options(nostack)
    );
}

pub unsafe fn heap_claim(heap: *mut u8, size: usize) -> *mut u8 {
    let mut result: *mut u8;
    asm!(
        "swi 0x2001d",
        in("r0") 2,
        in("r1") heap,
        in("r3") size,
        out("r2") result,
        options(nostack)
    );
    result
}

pub unsafe fn heap_release(heap: *mut u8, block: *mut u8) {
    asm!(
        "swi 0x1d",
        in("r0") 3,
        in("r1") heap,
        in("r2") block,
        options(nostack)
    );
}

pub unsafe fn heap_resize_block(heap: *mut u8, block: *mut u8, amount: isize) -> *mut u8 {
    let mut new_ptr: *mut u8;
    asm!(
        "swi 0x2001d",
        "movvs r2, #0",
        in("r0") 4,
        in("r1") heap,
        in("r2") block,
        in("r3") amount,
        lateout("r2") new_ptr,
        options(nostack)
    );
    new_ptr
}

pub unsafe fn write_n(ptr: *const u8, size: usize) {
    asm!(
        "swi 0x46",
        in("r0") ptr,
        in("r1") size,
        options(nostack)
    );
}

/// writes a character byte to VDU
pub fn write_c(c: u8) {
    unsafe {
        asm!(
            "swi 0x00",
            in("r0") c,
            options(nostack)
        );
    }
}

/// exits the application
pub fn exit() -> ! {
    unsafe {
        asm!("swi 0x11", options(noreturn, nostack));
    }
}

pub unsafe fn find_open(path: *const c_char) -> u32 {
    let mut handle: u32;
    asm!(
        "swi 0x0d",
        in("r0") 0x43,
        in("r1") path,
        lateout("r0") handle,
        options(nostack)
    );
    handle
}

pub unsafe fn find_create(path: *const c_char) -> u32 {
    let mut handle: u32;
    asm!(
        "swi 0x0d",
        in("r0") 0x83,
        in("r1") path,
        lateout("r0") handle,
        options(nostack)
    );
    handle
}

pub fn find_close(handle: u32) {
    unsafe {
        asm!(
            "swi 0x0d",
            in("r0") 0,
            in("r1") handle,
            lateout("r0") _,
            options(nostack)
        )
    }
}

pub unsafe fn gbpb_read(buffer: *mut u8, size: usize, handle: u32) -> (usize, bool) {
    let mut bytes_left: usize;
    let mut success: u32;
    asm!(
        "swi 0x2000c",
        "movvs r0, #0",
        in("r0") 4,
        in("r1") handle,
        in("r2") buffer,
        in("r3") size,
        lateout("r0") success,
        lateout("r2") _,
        lateout("r3") bytes_left,
        lateout("r4") _,
        options(nostack)
    );
    (size - bytes_left, success != 0)
}

pub unsafe fn gbpb_write(buffer: *const u8, size: usize, handle: u32) -> (usize, bool) {
    let mut bytes_left: usize;
    let mut success: u32;
    asm!(
        "swi 0x2000c",
        "movvs r0, #0",
        in("r0") 2,
        in("r1") handle,
        in("r2") buffer,
        in("r3") size,
        lateout("r0") success,
        lateout("r2") _,
        lateout("r3") bytes_left,
        lateout("r4") _,
        options(nostack)
    );
    (size - bytes_left, success != 0)
}

pub unsafe fn file_set_type(fname: *const c_char, ftype: u32) -> bool {
    let mut success: u32;
    asm!(
        "swi 0x20008",
        "movvs r0, #0",
        in("r0") 18,
        in("r1") fname,
        in("r2") ftype,
        lateout("r0") success
    );
    success != 0
}
