use crate::{println, serial_println};

pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_YIELD: u64 = 24;
pub const SYS_EXIT: u64 = 60;

pub fn handle_syscall(syscall_num: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    match syscall_num {
        SYS_READ => sys_read(arg1 as i32, arg2 as *mut u8, arg3 as usize),
        SYS_WRITE => sys_write(arg1 as *const u8, arg2 as usize),
        SYS_OPEN => sys_open(arg1 as *const u8, arg2 as usize),
        SYS_CLOSE => sys_close(arg1 as i32),
        SYS_YIELD => sys_yield(),
        SYS_EXIT => sys_exit(arg1 as i32),
        _ => {
            println!("[SYSCALL UNKNOWN] Unknown syscall ID: {}", syscall_num);
            u64::MAX
        }
    }
}

fn sys_open(ptr: *const u8, len: usize) -> u64 {
    if ptr.is_null() || len == 0 {
        return u64::MAX;
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    if let Ok(filename) = core::str::from_utf8(slice) {
        println!("[SYSCALL SYS_OPEN] Opening file: {}", filename);
        serial_println!("[SYSCALL SYS_OPEN] Opening file: {}", filename);
        3 // Return file descriptor 3
    } else {
        u64::MAX
    }
}

fn sys_read(fd: i32, buf: *mut u8, count: usize) -> u64 {
    if buf.is_null() || count == 0 {
        return 0;
    }
    println!("[SYSCALL SYS_READ] Reading {} bytes from FD {}", count, fd);
    serial_println!("[SYSCALL SYS_READ] Reading {} bytes from FD {}", count, fd);
    0
}

fn sys_close(fd: i32) -> u64 {
    println!("[SYSCALL SYS_CLOSE] Closing FD {}", fd);
    serial_println!("[SYSCALL SYS_CLOSE] Closing FD {}", fd);
    0
}

fn sys_write(ptr: *const u8, len: usize) -> u64 {
    if ptr.is_null() || len == 0 {
        return 0;
    }

    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    if let Ok(text) = core::str::from_utf8(slice) {
        println!("[SYSCALL SYS_WRITE] {}", text);
        serial_println!("[SYSCALL SYS_WRITE] {}", text);
        len as u64
    } else {
        0
    }
}

fn sys_yield() -> u64 {
    serial_println!("[SYSCALL SYS_YIELD] Process yielding CPU");
    0
}

fn sys_exit(code: i32) -> u64 {
    println!("[SYSCALL SYS_EXIT] Process exited with status code: {}", code);
    serial_println!("[SYSCALL SYS_EXIT] Process exited with status code: {}", code);
    0
}
