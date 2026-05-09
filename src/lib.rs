use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use mdream::types::HTMLToMarkdownOptions;

/// Convert HTML to Markdown.
///
/// Returns a pointer to a null-terminated UTF-8 string.
/// Caller must free it with `mdream_free`.
///
/// Returns null on error (e.g. invalid UTF-8 input).
#[no_mangle]
pub extern "C" fn mdream_convert(
    html: *const c_char,
    origin: *const c_char,
) -> *mut c_char {
    if html.is_null() {
        return ptr::null_mut();
    }

    let html_str = match unsafe { CStr::from_ptr(html) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let mut options = HTMLToMarkdownOptions::default();

    if !origin.is_null() {
        match unsafe { CStr::from_ptr(origin) }.to_str() {
            Ok(s) => options.origin = Some(s.to_string()),
            Err(_) => {}
        }
    }

    let markdown = mdream::html_to_markdown(html_str, options);

    match CString::new(markdown) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string previously returned by `mdream_convert`.
#[no_mangle]
pub extern "C" fn mdream_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
