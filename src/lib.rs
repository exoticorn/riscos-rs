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
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::os::VDUWriter, core::format_args!($($args)*));
    }
}

#[macro_export]
/// send a formatted string + newline to VDU
macro_rules! println {
    ($($args:tt)*) => {
        _ = core::fmt::write(&mut $crate::os::VDUWriter, core::format_args!($($args)*));
        $crate::os::write(b"\r\n");
    }
}

struct Allocator(u32);

impl Allocator {
    fn grow_heap(&self, size: u32) -> bool {
        unsafe {
            let mut slot_size: u32;
            asm!(
                "swi 0x400ec",
                in("r0") -1,
                in("r1") -1,
                lateout("r0") slot_size,
                lateout("r1") _,
                lateout("r2") _,
                options(nostack)
            );
            let mut new_slot_size = (slot_size + size + 4095) & !4095;
            asm!(
                "swi 0x400ec",
                inout("r0") new_slot_size,
                in("r1") -1,
                lateout("r1") _,
                lateout("r2") _,
                options(nostack)
            );
            asm!(
                "swi 0x1d",
                in("r0") 5,
                in("r1") self.0,
                in("r3") new_slot_size - slot_size,
                options(nostack)
            );

            new_slot_size > slot_size
        }
    }
}

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut result: *mut u8 = core::ptr::null_mut();

        for i in 0..2 {
            asm!(
                "swi 0x2001d",
                in("r0") 2,
                in("r1") self.0,
                in("r3") (layout.size() + 3) & !3,
                out("r2") result,
                options(nostack)
            );
            if result.is_null() && i == 0 {
                self.grow_heap(layout.size() as u32);
            } else {
                break;
            }
        }
       
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

    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        let size_change = new_size as i32 - layout.size() as i32;
        let mut new_ptr: *mut u8 = core::ptr::null_mut();
        let mut grow_amount = 4096;
        for _ in 0..55 {
            asm!(
                "swi 0x2001d",
                "movvs r2, #0",
                in("r0") 4,
                in("r1") self.0,
                in("r2") ptr,
                in("r3") size_change,
                lateout("r2") new_ptr,
                options(nostack)
            );
            if new_ptr.is_null() && size_change > 0 {
                if !self.grow_heap(grow_amount) {
                    break;
                }
                grow_amount += grow_amount / 4;
            } else {
                break;
            }
        }
        new_ptr
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
            in("r3") size,
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
            if c == 10 as char {
                write_c(13);
            }
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
