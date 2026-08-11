# Zenith OS - Master Technical & Implementation Plan

**Project Location**: `E:\Project\NewOS\zenith_os\`  
**Reference Alignment**: Based on `E:\Project\NewOS\Reference\` concept images and `PROJECT_HANDOVER_FOR_CLAUDE.md`.

---

## 1. Reference Images Analysis & Vision

We have analyzed all 6 concept reference images in `E:\Project\NewOS\Reference\`:

1. **Boot Sequence (`ChatGPT Image Jun 20, 2026, 10_29_41 AM.png`)**:
   - `Void` $\rightarrow$ `Spark` $\rightarrow$ `Particle Formation` $\rightarrow$ `Big Bang` $\rightarrow$ `Particle Cloud` $\rightarrow$ `Neural Network` $\rightarrow$ `3D Brain Emergence` $\rightarrow$ `Nodes Activation` $\rightarrow$ `Zenith OS Ready`.
2. **3D Brain UI & Neural Navigation (`ChatGPT Image Jun 20, 2026, 10_29_00 AM.png` & `09_25_03 PM.png`)**:
   - Central `CORE SYSTEM` glowing neural brain mesh with active system nodes: `Process Manager`, `Memory Manager`, `Device Manager`, `Power Manager`, `Registry`, `Applications`, `Display System`, `Audio System`, `Network`, `Storage`, `Security Core`, `User Profiles`.
   - Deep-dive into sub-nodes (e.g. `Audio System` $\rightarrow$ `Audio Engine` $\rightarrow$ `Sound Mixer` / `Drivers`).
   - Node information panel showing status, CPU/RAM usage, security level (`Low`, `Medium`, `High`, `Critical`), and actions (`Open`, `Monitor`, `Settings`, `Deep Dive`).
3. **Workspace Mode ("Outside the Brain") (`ChatGPT Image Jun 20, 2026, 09_24_48 PM.png`)**:
   - Multi-monitor tiled/floating desktop workspace, application dock, security center, system monitor.
   - Interactive `Zenith Shell` terminal with commands: `workspace`, `brain`, `new_app`, `save_workspace`, `load_workspace`, `lock_workspace`, `shutdown`.
4. **Security & Permissions (`ChatGPT Image Jun 20, 2026, 10_29_18 AM.png`)**:
   - Admin authentication dialogs for locked core system & driver nodes.

---

## 2. Directory Structure Strategy inside `E:\Project\NewOS\`

To keep all source files neatly separated from `E:\Project\NewOS\Reference\`, all project components will be created inside `E:\Project\NewOS\zenith_os\`:

```text
E:\Project\NewOS\
├── Reference/                      # User-provided concept UI & Boot images
└── zenith_os/                      # Main OS Project Root
    ├── IMPLEMENTATION_PLAN.md       # Master specification & roadmap document
    ├── README.md                    # Quickstart & documentation
    ├── xtask/                       # Custom Rust build runner & QEMU launcher
    ├── kernel/                      # Zenith Bare-Metal Kernel (Rust #![no_std])
    │   ├── Cargo.toml
    │   ├── .cargo/config.toml
    │   ├── x86_64-zenith_os.json    # Target specification
    │   └── src/
    │       ├── main.rs              # Kernel entrypoint (_start) & panic handler
    │       ├── vga_buffer.rs        # VGA text mode / Framebuffer print & println!
    │       ├── serial.rs            # UART 16550 serial port logging
    │       ├── gdt.rs               # GDT, TSS & IST interrupt stacks
    │       ├── interrupts.rs        # IDT & CPU exception handlers (#BP, #DF, #PF)
    │       ├── memory.rs            # Physical frame allocator & paging
    │       └── allocator.rs         # Kernel heap allocator (Box, Vec, BTreeMap)
    ├── userland/                    # Userspace programs & Zenith Shell
    └── brain_ui/                    # 3D Brain Compositor & Graphical Shell
```

---

## 3. Phased Roadmap Execution

### Phase 0: Kernel Bootstrap & Foundation (Current Active Focus)
* **Goal**: Boot bare-metal kernel in QEMU, print diagnostic logs to screen and serial, handle CPU interrupts safely.
* **Deliverables**:
  - `E:\Project\NewOS\zenith_os\kernel\` workspace initialized with `x86_64` bare-metal target.
  - Custom `#[panic_handler]` and `_start` entrypoint.
  - VGA text driver & UART 16550 serial driver for QEMU debugging.
  - GDT + TSS setup to prevent triple-fault crashes.
  - IDT with handlers for Breakpoint (`#BP`), Double Fault (`#DF`), and Page Fault (`#PF`).

### Phase 1: Memory Management
* **Goal**: Physical frame allocator + paging page-table mapping + kernel heap allocator (`Box`, `Vec`).

### Phase 2: Task Scheduling & Concurrency
* **Goal**: Preemptive round-robin scheduler, kernel threads, and async executor.

### Phase 3: Ring 3 User Mode & Syscall Boundary
* **Goal**: Ring 3 transition, `syscall`/`sysret` handlers, ELF64 executable loader.

### Phase 4: Drivers, VFS & Filesystem
* **Goal**: Keyboard/mouse drivers, virtio block storage, FAT32/ext2 filesystem, VFS layer.

### Phase 5: Zenith Shell & Userland
* **Goal**: Init process (PID 1), interactive `Zenith Shell`, system commands.

### Phase 6: 3D Brain Compositor & Visual Shell
* **Goal**: Custom Wayland/Framebuffer compositor rendering the 3D Neural Brain interface, Node Navigation, and Workspace Mode matching reference designs.

---

## 4. Verification Plan

1. **Compilation Check**:
   Build the kernel package inside `E:\Project\NewOS\zenith_os\kernel\` using `cargo build --target x86_64-unknown-none`.
2. **QEMU Execution Check**:
   Run the bootable kernel in QEMU to confirm:
   - VGA screen displays `[ZENITH OS Kernel Phase 0]` welcome text.
   - Serial output streams cleanly to terminal log.
   - CPU exception test (`int3`) triggers `#BP` breakpoint handler without crashing.
