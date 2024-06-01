use libc::{EINVAL, pid_t, regex_t};
use redhook::{hook, real};
use std::process::Command;

hook! {
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
        // getpid()
}
}

hook! {
    unsafe fn getpid() -> pid_t => gp_stub {
        println!("Your about to know everything!");
        let res = real!(getpid)();
        println!("reality: {res}")
        return res + 2
    }
}
