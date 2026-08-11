pub mod dispatch;

use x86_64::registers::model_specific::{Efer, EferFlags, LStar, SFMask};
use x86_64::VirtAddr;

pub fn init() {
    // 1. Enable System Call Extensions (SCE) in EFER register
    unsafe {
        Efer::update(|flags| {
            flags.insert(EferFlags::SYSTEM_CALL_EXTENSIONS);
        });
    }

    // 2. Configure LSTAR MSR with assembly handler entrypoint
    LStar::write(VirtAddr::new(asm_syscall_entry as *const () as u64));

    // 3. Configure SFMASK MSR to mask Interrupt Flag (IF) during syscalls
    SFMask::write(x86_64::registers::rflags::RFlags::INTERRUPT_FLAG);

    crate::serial_println!("[SYSCALL MSR] Fast Syscall Boundary initialized.");
}

extern "C" fn asm_syscall_entry() {
    // Syscall entrypoint handler
}
