// Crate linkage metadata
#![link(name = "linenoise", vers = "0.1", author = "davbo, donbrowne")]

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

#[link(name="linenoise")]
extern {
    fn linenoise(prompt: *mut libc::c_char) -> *mut libc::c_char;
    fn linenoiseHistoryAdd(line: *mut libc::c_char) -> libc::c_int;
    //fn linenoiseHistorySetMaxLen(length: libc::c_int) -> libc::c_int;
    fn linenoiseHistorySave(filename: *mut libc::c_char) -> libc::c_int;
    //fn linenoiseHistoryLoad(filename: *mut libc::c_char) -> libc::c_int;
    fn linenoiseSetMultiLine(enabled: libc::c_int) -> libc::c_void;
    fn linenoiseClearScreen() -> libc::c_void;
}

pub fn init(prompt: &str) -> String {
    let line = prompt.with_c_str(|cstr| {
        unsafe {
            let buf = linenoise(cstr as *mut i8);
            let line = CString::new(buf as *const i8, true);
            String::from_str(line.as_str().unwrap())
        }
    });
    line
}

pub fn history_add(line: &str) -> bool {
    let added = line.with_c_str(|cstr| {
        unsafe {
            linenoiseHistoryAdd(cstr as *mut i8)
        }
    });
    added as int == 1 // number of lines added perhaps?
}

pub fn history_save(filename: &str) -> bool {
    let saved = filename.with_c_str(|cstr| {
        unsafe {
            linenoiseHistorySave(cstr as *mut i8)
        }
    });
    saved as int == 0 // Seems to always be 0 if successful?
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
