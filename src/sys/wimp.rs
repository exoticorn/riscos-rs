use core::arch::asm;

pub struct SlotSize {
    pub current: usize,
    pub next: usize,
    pub free: usize,
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

    SlotSize {
        current,
        next,
        free,
    }
}

pub unsafe fn initialize(version: u32, name: *const u8, user_messages: *const u32) -> (u32, u32) {
    let mut current_version;
    let mut handle;

    asm!(
        "swi 0x400c0",
        in("r0") version,
        in("r1") 0x4B534154,
        in("r2") name,
        in("r3") user_messages,
        lateout("r0") current_version,
        lateout("r1") handle,
        options(nostack)
    );

    (current_version, handle)
}

pub fn shutdown(handle: u32) {
    unsafe {
        asm!(
            "swi 0x400dd",
            in("r0") handle,
            in("r1") 0x4B534154,
            lateout("r0") _,
            options(nostack)
        );
    }
}

pub unsafe fn poll(poll_mask: u32, block: *mut u32, poll_word: *const u32) -> (u32, u32) {
    let mut reason_code;
    let mut sender;
    asm!(
        "swi 0x400c7",
        in("r0") poll_mask,
        in("r1") block,
        in("r3") poll_word,
        lateout("r0") reason_code,
        out("r2") sender,
        options(nostack)
    );
    (reason_code, sender)
}

pub unsafe fn create_icon(priority: i32, block: *const i32) -> i32 {
    let mut handle;
    asm!(
        "swi 0x400c2",
        in("r0") priority,
        in("r1") block,
        lateout("r0") handle,
        options(nostack)
    );
    handle
}

pub fn delete_icon(window: i32, icon: i32) {
    let block = [window, icon];
    unsafe {
        asm!(
            "swi 0x400c4",
            in("r1") &block,
            lateout("r1") _,
            options(nostack)
        );
    }
}
