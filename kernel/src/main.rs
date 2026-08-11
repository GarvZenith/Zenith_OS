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

    // 6. Serial Port Output for QEMU / Console logging
    serial_println!("===========================================");
    serial_println!("ZENITH OS Phase 1 Memory Management Active!");
    serial_println!("Box value: {}, Vec len: {}", *heap_value, vec.len());
    serial_println!("===========================================");

    // 7. Test Breakpoint Exception Handler (#BP)
    x86_64::instructions::interrupts::int3();
    println!("[SUCCESS] Breakpoint exception (#BP) handled cleanly!");

    #[cfg(test)]
    test_main();

    println!("[STATUS] Zenith OS Phase 1 Kernel active. CPU HLT loop...");

    // 8. Halt CPU until next interrupt
    loop {
        x86_64::instructions::hlt();
    }
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
