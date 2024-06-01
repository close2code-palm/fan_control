use std::ffi::CString;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use nix::sys::fanotify::FanotifyEvent;

#[cfg(target_os = "linux")]
fn read_events() -> Vec<FanotifyEvent> {
    let cs = CString::new("read_events").unwrap();
    Command::new("wall").arg("Someone is checking...");
    // let hooked = unsafe { dlsym(RTLD_NEXT,  cs.as_ptr())};
    sleep(Duration::from_secs(10));
    return vec![];
}

