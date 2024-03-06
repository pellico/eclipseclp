#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub(crate) mod eclipse_ffi;
pub mod engine;
pub mod error;

#[cfg(test)]
mod test {
    use crate::eclipse_ffi::*;
    use std::{
        ffi::CString,
        ptr::{null, null_mut},
    };
    #[test]
    fn basic_test() {
        unsafe {
            let mut engine_ptr: *mut ec_eng_t = null_mut();
            let res = ecl_init(null_mut(), &mut engine_ptr);

            assert!((res as u32) == PSUCCEED);
            // let res  = ecl_acquire_engine(engine_ptr);
            // println!("{}",res);
            // assert!(res as u32 ==PSUCCEED);
            // ecl_relinquish_engine(engine_ptr);
            let my_string: CString = CString::from_vec_unchecked(b"write(helloWorld)".to_vec());
            ecl_post_string(engine_ptr, my_string.as_ptr());
            ecl_resume(engine_ptr);
            // ecl_post_goal(arg1, arg2)
            // ecl_post_goal(engine_ptr,ecl_term(engine_ptr,ec_did(my_string.as_ptr(),1),"hello world"));
            //     EC_resume();
            ec_cleanup();
            // println!("{}",a)
        }
    }
}
