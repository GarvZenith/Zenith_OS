#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zenith_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
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
    println!("[INFO] Bootstrapping Phase 0 Bare-Metal Kernel...");
    println!("[INFO] BootInfo memory map entries: {:?}", boot_info.memory_map.iter().count());
    println!("[INFO] CPU GDT, TSS, & IDT exception tables initialized.");
    println!("[INFO] VGA Text Mode Buffer (0xb8000) active.");
    println!("[INFO] Serial Port COM1 (0x3F8) connected.");

    // 3. Serial Port Output for QEMU / Console logging
    serial_println!("===========================================");
    serial_println!("ZENITH OS Bare-Metal Kernel Phase 0 Booted!");
    serial_println!("===========================================");

    // 4. Test Breakpoint Exception Handler (#BP)
    x86_64::instructions::interrupts::int3();
    println!("[SUCCESS] Breakpoint exception (#BP) handled cleanly!");
    serial_println!("[SUCCESS] Exception handling verified.");

    #[cfg(test)]
    test_main();

    println!("[STATUS] Zenith OS Kernel active. Entering HLT CPU loop...");

    // 5. Halt CPU until next interrupt
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
