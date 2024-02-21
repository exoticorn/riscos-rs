use core::arch::global_asm;

use crate::sys;

global_asm! {
    ".section .text.entry",
    "ldr sp, =_stack_end",
    "mov r0, sp",
    "bl _init_allocator",
    "bl main",
    "mov r0, r0", // TODO: make this a valid AIF header
    "swi 0x11",
}

#[panic_handler]
fn panic(panic: &core::panic::PanicInfo) -> ! {
    crate::println!("Panic: {}", panic);
    crate::os::exit();
}

struct Allocator(*mut u8);

impl Allocator {
    fn grow_heap(&self, size: usize) -> bool {
        let slot_size = sys::wimp::slot_size(None, None).current;
        let wanted_slot_size = (slot_size + size + 4095) & !4095;
        let new_slot_size = sys::wimp::slot_size(Some(wanted_slot_size), None).current;
        unsafe {
            sys::os::heap_resize(self.0, (new_slot_size - slot_size) as isize);
        }
        new_slot_size > slot_size
    }
}

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = (layout.size() + 3) & !3;
        let mut result = sys::os::heap_claim(self.0, size);

        if result.is_null() {
            self.grow_heap(size);
            result = sys::os::heap_claim(self.0, size);
        }

        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        sys::os::heap_release(self.0, ptr);
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let size_change = new_size as isize - layout.size() as isize;
        let mut new_ptr: *mut u8 = core::ptr::null_mut();
        let mut grow_amount = 4096;
        for _ in 0..55 {
            new_ptr = sys::os::heap_resize_block(self.0, ptr, size_change);

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
pub extern "C" fn _init_allocator(base: *mut u8) {
    let top = sys::os::get_env().ram_limit;
    let size = (top as usize - base as usize) & !3;
    unsafe {
        sys::os::heap_initialise(base, size);
        GLOBAL_ALLOCATOR.0 = base;
    }
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: Allocator = Allocator(core::ptr::null_mut());

#[no_mangle]
pub extern "C" fn __atomic_load_4(ptr: *const u32, _memorder: i32) -> u32 {
    unsafe { *ptr }
}
