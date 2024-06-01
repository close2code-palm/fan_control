use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use libc::EINVAL;

#[cfg(target_os = "linux")]
pub unsafe extern "C" fn fanotify_mark(
    fd: libc::c_int,
    flags: libc::c_uint,
    mask: u64,
    dirfd: libc::c_int,
    path: *const libc::c_char,
) -> libc::c_int {
    // let cs = CString::new("read_events").unwrap();
    println!("marking...");
    Command::new("wall").arg("Someone is checking...").spawn().expect("wall fail");
    // let hooked = unsafe { dlsym(RTLD_NEXT,  cs.as_ptr())};
    // sleep(Duration::from_secs(10));
    return EINVAL;
}
