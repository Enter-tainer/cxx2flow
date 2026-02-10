use std::{cell::RefCell, ptr, slice, str};

use crate::{
    display::{GraphDisplayBackend, dot::Dot},
    generate,
};

thread_local! {
    static LAST_RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static LAST_ERROR: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn set_last_result(bytes: Vec<u8>) {
    LAST_RESULT.with(|slot| {
        *slot.borrow_mut() = bytes;
    });
}

fn set_last_error(bytes: Vec<u8>) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = bytes;
    });
}

fn read_bytes(ptr: *const u8, len: usize) -> Result<&'static [u8], String> {
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err("received null pointer with non-zero length".to_owned());
    }

    // SAFETY: Caller provides a valid buffer in wasm linear memory.
    Ok(unsafe { slice::from_raw_parts(ptr, len) })
}

fn generate_dot_inner(
    content_ptr: *const u8,
    content_len: usize,
    function_ptr: *const u8,
    function_len: usize,
    curly: bool,
) -> Result<String, String> {
    let content = read_bytes(content_ptr, content_len)?;
    let function = read_bytes(function_ptr, function_len)?;
    let function_name = if function.is_empty() {
        None
    } else {
        Some(
            str::from_utf8(function)
                .map_err(|err| format!("invalid UTF-8 in function name: {err}"))?
                .to_owned(),
        )
    };

    generate(
        content,
        "input.cpp",
        function_name,
        GraphDisplayBackend::Dot(Dot::new(curly)),
    )
    .map_err(|err| err.to_string())
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }
    let mut bytes = Vec::<u8>::with_capacity(size);
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    // SAFETY: Pointer and capacity come from `cxx2flow_alloc`.
    unsafe {
        drop(Vec::from_raw_parts(ptr, 0, size));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_generate_dot(
    content_ptr: *const u8,
    content_len: usize,
    function_ptr: *const u8,
    function_len: usize,
    curly: u32,
) -> i32 {
    match std::panic::catch_unwind(|| {
        generate_dot_inner(
            content_ptr,
            content_len,
            function_ptr,
            function_len,
            curly != 0,
        )
    }) {
        Ok(Ok(dot)) => {
            set_last_result(dot.into_bytes());
            set_last_error(Vec::new());
            0
        }
        Ok(Err(err)) => {
            set_last_result(Vec::new());
            set_last_error(err.into_bytes());
            1
        }
        Err(_) => {
            set_last_result(Vec::new());
            set_last_error(b"panic while generating flowchart".to_vec());
            2
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_result_ptr() -> *const u8 {
    LAST_RESULT.with(|slot| {
        let bytes = slot.borrow();
        if bytes.is_empty() {
            ptr::null()
        } else {
            bytes.as_ptr()
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_result_len() -> usize {
    LAST_RESULT.with(|slot| slot.borrow().len())
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_error_ptr() -> *const u8 {
    LAST_ERROR.with(|slot| {
        let bytes = slot.borrow();
        if bytes.is_empty() {
            ptr::null()
        } else {
            bytes.as_ptr()
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn cxx2flow_error_len() -> usize {
    LAST_ERROR.with(|slot| slot.borrow().len())
}
