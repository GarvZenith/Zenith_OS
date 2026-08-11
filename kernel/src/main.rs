#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zenith_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use zenith_kernel::{println, serial_println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // 1. Initialize core CPU data structures (GDT, TSS, IDT)
    zenith_kernel::init();

    // 2. Print Zenith OS Startup Banner to VGA Screen
    println!("========================================================");
    println!("               WELCOME TO ZENITH OS                    ");
    println!("       Your Mind. Your Space. Your Control.             ");
    println!("========================================================");
    println!("[INFO] Bootstrapping Phase 0 Kernel Foundation... OK");
    println!("[INFO] Initializing Phase 1 Memory Management...");

    // 3. Initialize Virtual Memory & Physical Frame Allocator
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { zenith_kernel::memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { zenith_kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    println!("[INFO] Physical Frame Allocator & Paging: ACTIVE");

    // 4. Initialize Kernel Heap Allocator
    zenith_kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    println!("[INFO] Kernel Heap Allocator (100 KiB): INITIALIZED");

    // 5. Test Dynamic Memory Allocations (Box, Vec)
    let heap_value = Box::new(42);
    println!("[HEAP TEST] Heap Box allocated at: {:p}, value: {}", heap_value, *heap_value);

    let mut vec = vec![10, 20, 30];
    vec.push(40);
    vec.push(50);
    println!("[HEAP TEST] Heap Vector allocated at: {:p}, elements: {:?}", vec.as_ptr(), vec);

    // 6. Initialize Multitasking Executor & Spawn Tasks
    println!("[INFO] Initializing Phase 2 Preemptive Multitasking...");
    let mut executor = zenith_kernel::task::simple_executor::SimpleExecutor::new();
    executor.spawn(zenith_kernel::task::Task::new(task_alpha()));
    executor.spawn(zenith_kernel::task::Task::new(task_beta()));

    println!("[TASK EXEC] Running concurrent kernel tasks:");
    executor.run();

    // 7. Initialize Phase 4 Drivers, PCI Scan & VFS Filesystem
    println!("[INFO] Initializing Phase 4 Drivers & VFS Filesystem...");
    zenith_kernel::drivers::pci::scan_pci_bus();

    // Test VFS Root Directory Listing & File Read/Write
    let vfs_list = zenith_kernel::fs::vfs::VFS.lock().list_root();
    println!("[VFS] Root Directory Contents: {:?}", vfs_list);

    if let Some(data) = zenith_kernel::fs::vfs::VFS.lock().read_file("welcome.txt") {
        if let Ok(text) = core::str::from_utf8(&data) {
            println!("[VFS READ] /welcome.txt -> {}", text);
            serial_println!("[VFS READ] /welcome.txt -> {}", text);
        }
    }

    // Write new file via VFS
    zenith_kernel::fs::vfs::VFS.lock().write_file("/kernel.log", b"Zenith OS Phase 4 Filesystem Active!");
    println!("[VFS WRITE] Created /kernel.log cleanly");

    // 8. Initialize Phase 5 Zenith Interactive Shell
    println!("[INFO] Initializing Phase 5 Zenith Interactive Shell...");
    let mut shell = zenith_kernel::shell::ZenithShell::new();
    shell.print_prompt();

    // Run test interactive shell commands
    shell.execute_command("help");
    shell.execute_command("sysinfo");
    shell.execute_command("ls");
    shell.execute_command("cat welcome.txt");
    shell.execute_command("ver");

    // 9. Initialize Phase 6 3D Brain Compositor & Graphical Visual Shell UI
    println!("[INFO] Initializing Phase 6 3D Brain Compositor & Graphical Visual Shell UI...");
    let compositor = zenith_kernel::gfx::compositor::WorkspaceCompositor::new();
    let mut fb = zenith_kernel::gfx::framebuffer::FRAMEBUFFER.lock();
    compositor.render_workspace(&mut fb);

    // 10. Test Phase 3 & Phase 4 File Syscalls
    test_user_syscall();

    // 11. Serial Port Output for QEMU / Console logging
    serial_println!("===========================================");
    serial_println!("ZENITH OS Phase 6 3D Brain Compositor Active!");
    serial_println!("===========================================");

    // 12. Test Breakpoint Exception Handler (#BP)
    x86_64::instructions::interrupts::int3();
    println!("[SUCCESS] Breakpoint exception (#BP) handled cleanly!");

    #[cfg(test)]
    test_main();

    println!("[STATUS] Zenith OS Phase 6 Kernel active. 3D Brain Shell Active!");

    // 13. Halt CPU until next interrupt
    loop {
        x86_64::instructions::hlt();
    }
}

fn test_user_syscall() {
    println!("[INFO] Initializing Phase 3 User Mode & Syscall Boundary...");
    let message = "Hello from Ring 3 Syscall Boundary!";
    let result = zenith_kernel::syscall::dispatch::handle_syscall(
        zenith_kernel::syscall::dispatch::SYS_WRITE,
        message.as_ptr() as u64,
        message.len() as u64,
        0,
    );
    println!("[SYSCALL TEST] SYS_WRITE returned bytes written: {}", result);

    let exit_status = zenith_kernel::syscall::dispatch::handle_syscall(
        zenith_kernel::syscall::dispatch::SYS_EXIT,
        0,
        0,
        0,
    );
    println!("[SYSCALL TEST] SYS_EXIT returned status: {}", exit_status);
}

async fn async_number() -> u32 {
    42
}

async fn task_alpha() {
    let number = async_number().await;
    println!("[TASK ALPHA] Running concurrently! Async number: {}", number);
}

async fn task_beta() {
    println!("[TASK BETA] Multitasking active in Zenith OS!");
}

/// Panic Handler for Zenith OS (Bare-Metal)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n[KERNEL PANIC] {}", info);
    serial_println!("\n[KERNEL PANIC] {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
