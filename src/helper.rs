use alloc::vec::Vec;

pub struct ROString(Vec<u8>);

impl ROString {
    pub fn from_str(s: &str) -> ROString {
        let mut v = Vec::with_capacity(s.len() + 1);
        for c in s.chars() {
            v.push(c as u8);
        }
        v.push(0);
        ROString(v)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}
