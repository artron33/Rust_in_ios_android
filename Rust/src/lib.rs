use std::os::raw::c_char;
use std::ffi::{CString, CStr};

mod main;
//use main::start;


#[no_mangle]
pub extern fn rust_helloo(searched: *const c_char, s1: *const c_char, s2: *const c_char, s3: *const c_char) -> *mut c_char {
    let s1_str = unsafe { CStr::from_ptr(s1) };
    let s2_str = unsafe { CStr::from_ptr(s2) };
    let s3_str = unsafe { CStr::from_ptr(s3) };
    let searched_str = unsafe { CStr::from_ptr(searched) };

    let path1 = convert_to_string(s1);
    let path2 = convert_to_string(s2);
    let path3 = convert_to_string(s3);
    let input = convert_to_string(searched);

    // Call the Start function from alt.rs
    let result = main::start(
        input,
        path1,
        path2,
        path3
        );

    // Convert the result String to a C-compatible string
    CString::new(result).unwrap().into_raw()
}

fn convert_to_string(searched: *const std::os::raw::c_char) -> String {
    unsafe {
        if searched.is_null() {
            // Handle the case where the pointer is null
            return String::new();
        }

        // Convert the C string to a Rust string
        let c_str = CStr::from_ptr(searched);
        let result_str = match c_str.to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => String::new(), // Handle the case where the conversion fails
        };

        result_str
    }
}

#[no_mangle]
pub extern fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s);
    }
}