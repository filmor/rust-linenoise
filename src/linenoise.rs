// Crate linkage metadata
#![link(name = "linenoise", vers = "0.2", author = "davbo, donbrowne")]

// Make a library ("bin" is the default)
#![crate_type = "lib"]

extern crate libc;
use std::c_str::CString;

/*
struct linenoiseCompletions {
    len: libc::size_t,
    cvec: *mut *mut libc::c_char,
}
*/

#[link(name="linenoise", kind="static")]
extern {
    fn linenoise(prompt: *mut libc::c_char) -> *mut libc::c_char;
    fn linenoiseHistoryAdd(line: *mut libc::c_char) -> libc::c_int;
    fn linenoiseHistorySetMaxLen(length: libc::c_int) -> libc::c_int;
    fn linenoiseHistorySave(filename: *mut libc::c_char) -> libc::c_int;
    fn linenoiseHistoryLoad(filename: *mut libc::c_char) -> libc::c_int;
    fn linenoiseSetMultiLine(enabled: libc::c_int) -> libc::c_void;
    fn linenoiseClearScreen() -> libc::c_void;
    fn linenoisePrintKeyCodes() -> libc::c_void;
}

pub fn init(prompt: &str) -> String {
    prompt.with_c_str(|cstr| {
        unsafe {
            let buf = linenoise(cstr as *mut i8);
            let line = CString::new(buf as *const i8, true);
            String::from_str(line.as_str().unwrap())
        }
    })
}

pub fn history_add(line: &str) -> bool {
    line.with_c_str(|cstr| {
        unsafe {
            linenoiseHistoryAdd(cstr as *mut i8)
        }
    }) as int == 1
}

pub fn history_save(filename: &str) -> bool {
    filename.with_c_str(|cstr| {
        unsafe {
            linenoiseHistorySave(cstr as *mut i8)
        }
    }) as int == 0
}

pub fn set_multiline(enable: bool) {
    if enable {
        unsafe {
            linenoiseSetMultiLine(1);
        }
    } else {
        unsafe {
            linenoiseSetMultiLine(0);
        }
    }
}

pub fn clear_screen() {
    unsafe {
        linenoiseClearScreen();
    }
}
