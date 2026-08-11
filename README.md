# Zenith OS - The Intelligent Operating System

> **"Your Mind. Your Space. Your Control."**  
> *Everything is Connected. Everything is Intelligent.*

Zenith OS is a standalone, neural-brain-based Operating System written from scratch in Rust (`#![no_std]`). It features a bare-metal kernel foundation and a 3D Neural Brain visual shell.

---

## 📁 Repository Structure

```text
E:\Project\NewOS\
├── Reference/                      # Visual design storyboards & boot animation concepts
└── zenith_os/                      # Main Operating System Project Root
    ├── IMPLEMENTATION_PLAN.md       # Master specification & roadmap document
    ├── README.md                    # Project documentation & usage guide
    ├── LICENSE                      # MIT Open-Source License
    ├── zenith_build_check.bat       # One-click build script
    └── kernel/                      # Zenith Bare-Metal Kernel (Rust #![no_std])
        ├── Cargo.toml               # Kernel dependencies & bootimage config
        ├── .cargo/config.toml       # Target runner & linker configuration
        └── src/
            ├── main.rs              # Kernel entrypoint (_start) & panic handler
            ├── lib.rs               # Kernel library core & test harness
            ├── vga_buffer.rs        # VGA text mode (0xb8000) & formatting macros
            ├── serial.rs            # UART 16550 serial port logging (COM1)
            ├── gdt.rs               # GDT, TSS, IST & Ring 3 User Descriptors
            ├── interrupts.rs        # IDT & CPU exception handlers (#BP, #DF, #PF)
            ├── memory.rs            # Physical Frame Allocator & Paging page tables
            ├── allocator.rs         # Kernel Heap Allocator (Box, Vec, BTreeMap)
            ├── pic.rs               # 8259 PIC Hardware Interrupt Controller Remap
            ├── syscall/             # Fast Syscall Boundary & Dispatcher
            │   ├── mod.rs           # MSR registers (EFER, LSTAR, SFMASK) init
            │   └── dispatch.rs      # SYS_WRITE, SYS_YIELD, SYS_EXIT handlers
            └── task/                # Multitasking & Concurrency Engine
                ├── mod.rs           # TaskId & Task async abstractions
                └── simple_executor.rs # Kernel Async Task Executor
```

---

## 🗺️ Progress Roadmap

- [x] **Phase 0: Kernel Bootstrap & Foundation**
  - Bare-metal `#![no_std]` x86_64 entrypoint (`kernel_main`)
  - VGA text mode driver (`0xb8000`) & COM1 serial port logging
  - Global Descriptor Table (GDT), TSS, & Interrupt Stack Tables (IST)
  - Interrupt Descriptor Table (IDT) with `#BP`, `#DF`, `#PF` handlers
- [x] **Phase 1: Memory Management & Heap Allocator**
  - Physical Frame Allocator & Level 4 Page Table mapping (`src/memory.rs`)
  - 100 KiB Kernel Heap Allocator (`src/allocator.rs`)
  - Dynamic memory allocations (`Box`, `Vec`) in kernel space
- [x] **Phase 2: Preemptive Task Scheduler & Concurrency**
  - 8259 PIC Interrupt Controller Remapping (`src/pic.rs`)
  - Hardware Timer (IRQ 0) & Raw PS/2 Keyboard (IRQ 1) handlers
  - `Task` & `TaskId` async abstractions (`src/task/mod.rs`)
  - Kernel Async Task Executor & Multitasking (`src/task/simple_executor.rs`)
- [x] **Phase 3: Ring 3 User Mode & Syscall Boundary**
  - GDT User Code (`0x18|3`) & User Data (`0x20|3`) segment descriptors
  - Fast hardware syscall MSR configuration (`EFER`, `LStar`, `SFMask`)
  - Syscall Dispatch Table (`SYS_WRITE`, `SYS_YIELD`, `SYS_EXIT`)
- [ ] **Phase 4: Drivers, VFS & Filesystem**
- [ ] **Phase 5: Zenith Shell & Userland Init**
- [ ] **Phase 6: 3D Brain Compositor & Visual Shell**

---

## 🚀 Building & Running Zenith OS

### Prerequisites
1. **Rust Nightly Toolchain**:
   ```bash
   rustup default nightly
   rustup target add x86_64-unknown-none
   cargo install bootimage
   ```
2. **QEMU Emulator** (for visual execution):
   Ensure `qemu-system-x86_64` is installed.

### Build Kernel Disk Image
```bash
cd kernel
cargo bootimage --target x86_64-unknown-none
```

### Run in QEMU
```bash
"C:\Program Files\qemu\qemu-system-x86_64.exe" -drive format=raw,file=target\x86_64-unknown-none\debug\bootimage-zenith_kernel.bin -serial stdio
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
