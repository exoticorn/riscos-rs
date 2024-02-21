use embedded_io::{ErrorType, Read};

use crate::{
    path::{self, Path},
    sys,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    NotFound,
    InvalidPath,
    PermissionDenied,
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        match *self {
            Error::NotFound => embedded_io::ErrorKind::NotFound,
            Error::InvalidPath => embedded_io::ErrorKind::InvalidInput,
            Error::PermissionDenied => embedded_io::ErrorKind::PermissionDenied,
        }
    }
}

pub struct File(u32);

impl File {
    pub fn open<'a, P: Into<Path<'a>>>(path: P) -> Result<File, Error> {
        let mut buffer = [0u8; path::MAX_PATH_LENGTH];
        let path_str = path
            .into()
            .to_c_str(&mut buffer)
            .map_err(|_| Error::InvalidPath)?;
        let handle = unsafe { sys::os::find_open(path_str.as_ptr()) };
        if handle == 0 {
            Err(Error::NotFound)
        } else {
            Ok(File(handle))
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        sys::os::find_close(self.0);
    }
}

impl ErrorType for File {
    type Error = Error;
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let (count, success) = unsafe { sys::os::gbpb_read(buf.as_mut_ptr(), buf.len(), self.0) };
        if !success {
            Err(Error::PermissionDenied)
        } else {
            Ok(count)
        }
    }
}
