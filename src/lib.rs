#![no_std]

use core::arch::{asm, global_asm };

global_asm!{
    ".section .text.entry",
    "ldr sp, =_stack_end",
    "mov r0, sp",
    "bl _init_allocator",
    "bl main",
    "mov r0, r0",
    "swi 0x11",
}

#[macro_export]
/// send a formatted string to VDU
macro_rules! print {
    ($args:tt) => {
        core::fmt::write(&mut $crate::os::VDUWriter, core::format_args!($args));
    }
}

#[macro_export]
/// send a formatted string + newline to VDU
macro_rules! println {
    ($($args:tt),*) => {
        _ = core::fmt::write(&mut $crate::os::VDUWriter, core::format_args!($($args),*));
        $crate::os::write(b"\r\n");
    }
}

struct Allocator(u32);

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut result;
        asm!(
            "swi 0x1d",
            in("r0") 2,
            in("r1") self.0,
            in("r3") (layout.size() + 3) & !3,
            out("r2") result,
            options(nostack)
        );
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        asm!(
            "swi 0x1d",
            in("r0") 3,
            in("r1") self.0,
            in("r2") ptr,
            options(nostack)
        );
    }
}

#[no_mangle]
pub extern "C" fn _init_allocator(base: u32) {
    let mut top: u32;
    unsafe {
        asm!(
            "swi 0x10",
            out("r0") _,
            out("r1") top,
            out("r2") _,
            options(nostack)
        );
    }
    let size = (top - base) & !3;
    unsafe {
        asm!(
            "swi 0x1d",
            in("r0") 0,
            in("r1") base,
            in("r2") size,
            options(nostack)
        );
        GLOBAL_ALLOCATOR.0 = base;
    }
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: Allocator = Allocator(0);

pub mod os {
    use core::arch::asm;
    use core::panic::PanicInfo;

    /// Write formatted output to VDU
    pub struct VDUWriter;

    impl core::fmt::Write for VDUWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            write_str(s);
            Ok(())
        }
    }

    #[panic_handler]
    fn panic(panic: &PanicInfo) -> ! {
        println!("Panic: {}", panic);
        exit();
    }

    /// exits the application
    pub fn exit() -> ! {
        unsafe {
            asm!(
                "swi 0x11",
                options(noreturn, nostack)
            );
        }
    }

    /// writes a byte string to VDU
    pub fn write(s: &[u8]) {
        unsafe {
            asm!(
                "swi 0x46",
                in("r0") s.as_ptr(),
                in("r1") s.len(),
                options(nostack)
            );
        }
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

    /// writes the characters from a string to VDU
    ///
    /// output for characters outside the range 0..256 is undefined
    pub fn write_str(s: &str) {
        for c in s.chars() {
            write_c(c as u8);
        }
    }
}

#[no_mangle]
pub extern "C" fn __atomic_load_4(ptr: *const u32, _memorder: i32) -> u32 {
    unsafe {
        *ptr
    }
}
