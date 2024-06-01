use libc::{EINVAL, pid_t};
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
    println!("marking...");
    Command::new("wall").arg("Someone is checking...").spawn().expect("wall fail");

    return EINVAL;
}
}

hook! {
    unsafe fn getpid() -> pid_t => gp_stub {
        println!("Your about to know everything!");
        let res = real!(getpid)();
        println!("reality: {res}");
        return res + 2
    }
}
