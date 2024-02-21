use core::ffi::CStr;

pub const MAX_PATH_LENGTH: usize = 256;

pub enum Error {
    PathTooLong,
    InvalidPath,
}

pub struct Path<'a>(&'a [u8]);

impl<'a> From<&'a str> for Path<'a> {
    fn from(s: &'a str) -> Path<'a> {
        Path(s.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for Path<'a> {
    fn from(s: &'a [u8]) -> Path<'a> {
        Path(s)
    }
}

impl<'a> Path<'a> {
    pub fn to_c_str<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b CStr, Error> {
        let path_len = self.0.len();
        if path_len < buffer.len() {
            buffer[..path_len].copy_from_slice(self.0);
            buffer[path_len] = 0;
            CStr::from_bytes_with_nul(&buffer[..path_len + 1]).map_err(|_| Error::InvalidPath)
        } else {
            Err(Error::PathTooLong)
        }
    }
}
