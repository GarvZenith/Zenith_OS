use crate::{println, serial_println};

pub const SYS_WRITE: u64 = 1;
pub const SYS_YIELD: u64 = 24;
pub const SYS_EXIT: u64 = 60;

pub fn handle_syscall(syscall_num: u64, arg1: u64, arg2: u64, _arg3: u64) -> u64 {
    match syscall_num {
        SYS_WRITE => sys_write(arg1 as *const u8, arg2 as usize),
        SYS_YIELD => sys_yield(),
        SYS_EXIT => sys_exit(arg1 as i32),
        _ => {
            println!("[SYSCALL UNKNOWN] Unknown syscall ID: {}", syscall_num);
            u64::MAX
        }
    }
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
