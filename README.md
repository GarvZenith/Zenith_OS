# Zenith OS - The Intelligent Operating System

> **"Your Mind. Your Space. Your Control."**  
> Everything is Connected. Everything is Intelligent.

 Zenith OS is a standalone, bootable Operating System built in Rust. It features a bare-metal kernel foundation and a 3D Neural Brain visual shell.

---

## 📁 Repository Structure

```text
E:\Project\NewOS\
├── Reference/                      # Concept visual designs & boot sequence storyboards
└── zenith_os/                      # Main Operating System Project Root
    ├── IMPLEMENTATION_PLAN.md       # Master specification & roadmap document
    ├── README.md                    # Project documentation & usage guide
    ├── kernel/                      # Zenith Bare-Metal Kernel (Rust #![no_std])
    │   ├── Cargo.toml               # Kernel dependencies & bootimage config
    │   ├── .cargo/config.toml       # Bare-metal target runner configuration
    │   └── src/
    │       ├── main.rs              # Kernel entrypoint (_start) & panic handler
    │       ├── lib.rs               # Test harness & kernel core exports
    │       ├── vga_buffer.rs        # VGA text mode (0xb8000) & formatting macros
    │       ├── serial.rs            # UART 16550 serial port logging (COM1)
    │       ├── gdt.rs               # GDT, TSS & IST interrupt stacks
    │       └── interrupts.rs        # IDT & CPU exception handlers (#BP, #DF, #PF)
    ├── userland/                    # Userspace programs & Zenith Shell
    └── brain_ui/                    # 3D Brain Compositor & Graphical UI
```

---

## 🚀 Building & Running

### Prerequisites
1. **Rust Nightly Toolchain**:
   ```bash
   rustup default nightly
   rustup component add rust-src llvm-tools-preview
   ```
2. **QEMU Emulator** (for running the bootable disk image):
   Install QEMU and ensure `qemu-system-x86_64` is on system PATH.

### Build Kernel
```bash
cd E:\Project\NewOS\zenith_os\kernel
cargo build --target x86_64-unknown-none
```

### Run in QEMU
```bash
cargo run
```
