use std::{ffi::CStr, str::Utf8Error};
use std::fmt::Debug;
use crate::eclipse_ffi::ec_error_string;

pub struct Error {
    error: i32,
}

impl Error {
    pub(crate) fn new(error: i32) -> Self {
        Error { error }
    }

    pub fn to_string(&self) -> Result<&'static str, Utf8Error> {
        unsafe {
            let result = ec_error_string(self.error);
            let my_cstr: &'static CStr = CStr::from_ptr(result);
            my_cstr.to_str()
        }
    }
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        Error { error: value }
    }
}

impl Debug for Error{
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let err_msg = self.to_string();
    match err_msg {
        Ok(err_str) => f.write_str(err_str),
        Err(_) => Err(std::fmt::Error)

    }
    
}
}