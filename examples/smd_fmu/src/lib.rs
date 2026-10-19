mod fmu;
mod model;

use fmi3::{function_types::*, platform_types::*};
use std::ffi::{c_char, c_void};

// #region common functions
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3GetVersion() -> *const c_char {
    std::ffi::CStr::from_bytes_with_nul(b"3.0\0")
        .unwrap()
        .as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fmi3SetDebugLogging(
    _: FMI3Instance,
    _: FMI3Boolean,
    _: usize,
    _: *const FMI3String,
) {
    // Do nothing for now
}
// #endregion common functions
