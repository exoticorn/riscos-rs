use core::arch::asm;

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
