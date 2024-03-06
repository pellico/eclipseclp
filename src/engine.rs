use crate::eclipse_ffi::*;
use crate::error::*;
use std::{
    ffi::CString,
    ptr::{null, null_mut},
};

pub struct Engine {
    engine_ptr: *const ec_eng_t,
}

impl Engine {
    pub fn new() -> Result<Engine, Error> {
        let mut engine_ptr: *mut ec_eng_t = null_mut();
        let res;
        unsafe {
            res = ecl_init(null_mut(), &mut engine_ptr);
        }
        match res as u32 {
            PSUCCEED => Ok(Engine { engine_ptr }),
            _ => Err(Error::new(res))
        }
        
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new().expect("Failed to create new engine")
    }
}

impl Drop for Engine{
    fn drop(&mut self) {
        
    }
}
