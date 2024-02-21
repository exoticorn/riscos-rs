use core::arch::asm;

pub struct SlotSize {
    pub current: usize,
    pub next: usize,
    pub free: usize
}

pub fn slot_size(new_current: Option<usize>, new_next: Option<usize>) -> SlotSize {
    let mut current: usize;
    let mut next: usize;
    let mut free: usize;

    unsafe {
        asm!(
            "swi 0x400ec",
            in("r0") new_current.map(|v| v.min(i32::MAX as usize) as i32).unwrap_or(-1),
            in("r1") new_next.map(|v| v.min(i32::MAX as usize) as i32).unwrap_or(-1),
            lateout("r0") current,
            lateout("r1") next,
            lateout("r2") free,
            options(nostack)
        );
    }

    SlotSize { current, next, free }
}
