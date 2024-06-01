use std::process::Command;
use libc::EINVAL;
use redhook::hook;

hook!{
unsafe fn fanotify_mark(
    fd: libc::c_int,
    flags: libc::c_uint,
    mask: u64,
    dirfd: libc::c_int,
    path: *const libc::c_char
) -> libc::c_int => fan_stub {
    // let cs = CString::new("read_events").unwrap();
    println!("marking...");
    Command::new("wall").arg("Someone is checking...").spawn().expect("wall fail");
    // let hooked = unsafe { dlsym(RTLD_NEXT,  cs.as_ptr())};
    // sleep(Duration::from_secs(10));
    return EINVAL;
}
}
