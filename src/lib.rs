use std::ffi::CString;
use libc::{dlsym, RTLD_NEXT};
use nix::sys::fanotify::FanotifyEvent;

#[cfg(target_os = "linux")]
fn read_events() -> Vec<FanotifyEvent> {
    let cs = CString::new("read_events").unwrap();
    let hooked = unsafe { dlsym(RTLD_NEXT,  cs.as_ptr())};
    return vec![];
}

