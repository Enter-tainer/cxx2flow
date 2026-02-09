use std::alloc::{
    Layout, alloc as rust_alloc, alloc_zeroed as rust_alloc_zeroed, dealloc as rust_dealloc,
    realloc as rust_realloc,
};
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

const ALIGNMENT: usize = std::mem::size_of::<usize>();
const HEADER_SIZE: usize = std::mem::size_of::<usize>();

fn layout_for_allocation(size: usize) -> Option<Layout> {
    size.checked_add(HEADER_SIZE)
        .and_then(|total| Layout::from_size_align(total, ALIGNMENT).ok())
}

unsafe fn base_ptr_and_size(user_ptr: *mut u8) -> Option<(*mut u8, usize)> {
    if user_ptr.is_null() {
        return None;
    }

    let base_ptr = unsafe { user_ptr.sub(HEADER_SIZE) };
    let size = unsafe { ptr::read(base_ptr as *const usize) };
    Some((base_ptr, size))
}

unsafe fn store_size(base_ptr: *mut u8, size: usize) {
    unsafe { ptr::write(base_ptr as *mut usize, size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }

    let layout = match layout_for_allocation(size) {
        Some(layout) => layout,
        None => return ptr::null_mut(),
    };
    let base_ptr = unsafe { rust_alloc(layout) };
    if base_ptr.is_null() {
        return ptr::null_mut();
    }

    unsafe { store_size(base_ptr, size) };
    unsafe { base_ptr.add(HEADER_SIZE) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    let user_size = match nmemb.checked_mul(size) {
        Some(total) if total != 0 => total,
        _ => return ptr::null_mut(),
    };

    let layout = match layout_for_allocation(user_size) {
        Some(layout) => layout,
        None => return ptr::null_mut(),
    };

    let base_ptr = unsafe { rust_alloc_zeroed(layout) };
    if base_ptr.is_null() {
        return ptr::null_mut();
    }

    unsafe { store_size(base_ptr, user_size) };
    unsafe { base_ptr.add(HEADER_SIZE) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    if ptr.is_null() {
        return unsafe { malloc(new_size) };
    }

    if new_size == 0 {
        unsafe { free(ptr) };
        return ptr::null_mut();
    }

    let (base_ptr, old_size) = match unsafe { base_ptr_and_size(ptr) } {
        Some(v) => v,
        None => return ptr::null_mut(),
    };
    let old_layout = match layout_for_allocation(old_size) {
        Some(layout) => layout,
        None => return ptr::null_mut(),
    };
    let new_layout = match layout_for_allocation(new_size) {
        Some(layout) => layout,
        None => return ptr::null_mut(),
    };

    let new_base = unsafe { rust_realloc(base_ptr, old_layout, new_layout.size()) };
    if new_base.is_null() {
        return ptr::null_mut();
    }

    unsafe { store_size(new_base, new_size) };
    unsafe { new_base.add(HEADER_SIZE) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    if let Some((base_ptr, size)) = unsafe { base_ptr_and_size(ptr) } {
        if let Some(layout) = layout_for_allocation(size) {
            unsafe { rust_dealloc(base_ptr, layout) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    if s1.is_null() || s2.is_null() {
        return 0;
    }

    for index in 0..n {
        let c1 = unsafe { *s1.add(index) as u8 };
        let c2 = unsafe { *s2.add(index) as u8 };

        if c1 == 0 || c2 == 0 || c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    if s1.is_null() || s2.is_null() {
        return 0;
    }

    let mut index = 0usize;
    loop {
        let c1 = unsafe { *s1.add(index) as u8 };
        let c2 = unsafe { *s2.add(index) as u8 };
        if c1 == 0 || c2 == 0 || c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    for index in 0..n {
        let c = unsafe { *src.add(index) };
        unsafe { *dest.add(index) = c };
        if c == 0 {
            break;
        }
    }

    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void {
    if s.is_null() || n == 0 {
        return ptr::null_mut();
    }

    let bytes = s as *const u8;
    let needle = c as u8;
    for index in 0..n {
        if unsafe { *bytes.add(index) } == needle {
            return unsafe { bytes.add(index) as *mut c_void };
        }
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn fclose(_stream: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fdopen(_fd: c_int, _mode: *const c_char) -> *mut c_void {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn clock() -> usize {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fwrite(
    _ptr: *const c_void,
    _size: usize,
    _nmemb: usize,
    _stream: *mut c_void,
) -> usize {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fputc(_c: c_int, _stream: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn iswspace(wc: u32) -> c_int {
    matches!(
        wc,
        0x20
            | 0x09..=0x0D
            | 0xA0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
    ) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn iswalnum(wc: u32) -> c_int {
    (iswalpha(wc) != 0 || iswdigit(wc) != 0) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn iswdigit(wc: u32) -> c_int {
    matches!(wc, 0x30..=0x39) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn iswxdigit(wc: u32) -> c_int {
    (iswdigit(wc) != 0 || matches!(wc, 0x41..=0x46 | 0x61..=0x66)) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn iswupper(wc: u32) -> c_int {
    if (0x41..=0x5A).contains(&wc) {
        return 1;
    }
    if (0xC0..=0xD6).contains(&wc) || (0xD8..=0xDE).contains(&wc) {
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn iswlower(wc: u32) -> c_int {
    if (0x61..=0x7A).contains(&wc) {
        return 1;
    }
    if (0xE0..=0xF6).contains(&wc) || (0xF8..=0xFF).contains(&wc) {
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn iswpunct(wc: u32) -> c_int {
    matches!(wc, 0x21..=0x2F | 0x3A..=0x40 | 0x5B..=0x60 | 0x7B..=0x7E) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn iswalpha(wc: u32) -> c_int {
    matches!(
        wc,
        0x41..=0x5A
            | 0x61..=0x7A
            | 0xAA
            | 0xB5
            | 0xBA
            | 0xC0..=0xD6
            | 0xD8..=0xF6
            | 0xF8..=0x2C1
            | 0x2C6..=0x2D1
            | 0x2E0..=0x2E4
            | 0x2EC
            | 0x2EE
            | 0x370..=0x374
            | 0x376..=0x377
            | 0x37A..=0x37D
            | 0x37F
            | 0x386
            | 0x388..=0x38A
            | 0x38C
            | 0x38E..=0x3A1
            | 0x3A3..=0x3F5
            | 0x3F7..=0x481
            | 0x48A..=0x52F
            | 0x531..=0x556
            | 0x559
            | 0x560..=0x588
    ) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn towlower(wc: u32) -> u32 {
    if (0x41..=0x5A).contains(&wc) || (0xC0..=0xD6).contains(&wc) || (0xD8..=0xDE).contains(&wc) {
        return wc + 32;
    }
    wc
}

#[unsafe(no_mangle)]
pub extern "C" fn towupper(wc: u32) -> u32 {
    if (0x61..=0x7A).contains(&wc) || (0xE0..=0xF6).contains(&wc) || (0xF8..=0xFE).contains(&wc) {
        return wc - 32;
    }
    wc
}

#[unsafe(no_mangle)]
pub extern "C" fn fputs(_s: *const c_char, _stream: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() {}

#[unsafe(no_mangle)]
pub extern "C" fn dup(_fd: c_int) -> c_int {
    -1
}

#[cfg(target_family = "wasm")]
#[used]
static _FORCE_INCLUDE: () = {
    let _ = strncmp as *const ();
    let _ = strcmp as *const ();
    let _ = strncpy as *const ();
    let _ = memchr as *const ();
    let _ = fclose as *const ();
    let _ = fdopen as *const ();
    let _ = clock as *const ();
    let _ = fwrite as *const ();
    let _ = fputc as *const ();
    let _ = iswspace as *const ();
    let _ = iswalnum as *const ();
    let _ = iswdigit as *const ();
    let _ = iswxdigit as *const ();
    let _ = iswupper as *const ();
    let _ = iswlower as *const ();
    let _ = iswpunct as *const ();
    let _ = iswalpha as *const ();
    let _ = towlower as *const ();
    let _ = towupper as *const ();
    let _ = fputs as *const ();
    let _ = abort as *const ();
    let _ = dup as *const ();
};
