use crate::{println, serial_println};
use alloc::string::String;
use alloc::vec::Vec;

pub struct ZenithShell {
    pub buffer: String,
}

impl ZenithShell {
    pub fn new() -> Self {
        ZenithShell {
            buffer: String::new(),
        }
    }

    pub fn print_prompt(&self) {
        println!("\nzenith> ");
        serial_println!("\nzenith> ");
    }

    pub fn execute_command(&mut self, cmd_line: &str) {
        let trimmed = cmd_line.trim();
        if trimmed.is_empty() {
            return;
        }

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        serial_println!("[SHELL CMD] Executing: {}", trimmed);

        match command {
            "help" => self.cmd_help(),
            "sysinfo" => self.cmd_sysinfo(),
            "ls" => self.cmd_ls(),
            "cat" => self.cmd_cat(&args),
            "echo" => self.cmd_echo(&args),
            "ver" | "version" => self.cmd_ver(),
            "clear" => self.cmd_clear(),
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", command);
                serial_println!("Unknown command: '{}'. Type 'help' for available commands.", command);
            }
        }
    }

    fn cmd_help(&self) {
        println!("========================================================");
        println!("           ZENITH OS INTERACTIVE SHELL HELP            ");
        println!("========================================================");
        println!("  help            - Display this documentation menu");
        println!("  sysinfo         - Display CPU, Memory & Kernel Stats");
        println!("  ls              - List VFS Root Directory files");
        println!("  cat <filename>  - Display VFS File content");
        println!("  echo <text>     - Print text back to console");
        println!("  ver             - Display Zenith OS version info");
        println!("  clear           - Reset VGA text mode screen");
        println!("========================================================");
    }

    fn cmd_sysinfo(&self) {
        println!("========================================================");
        println!("              ZENITH OS SYSTEM INFORMATION              ");
        println!("========================================================");
        println!("  Target Arch     : x86_64-unknown-none (Bare-Metal)");
        println!("  Kernel Base     : 0x200000 (Relocatable static ELF)");
        println!("  GDT / TSS       : Ring 0 Kernel / Ring 3 User Descriptors");
        println!("  Interrupts      : 8259 PIC Remapped (IRQ 0x20/0x28)");
        println!("  Syscall MSR     : IA32_EFER, LStar, SFMask Configured");
        println!("  Heap Allocator  : 100 KiB Mapped at 0x444444440000");
        println!("  VFS Status      : Virtual File System Active");
        println!("========================================================");
    }

    fn cmd_ls(&self) {
        println!("[VFS LIST] Directory Listing for '/':");
        let items = crate::fs::vfs::VFS.lock().list_root();
        for item in items {
            println!("  {}", item);
        }
    }

    fn cmd_cat(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: cat <filename>");
            return;
        }

        let filename = args[0];
        if let Some(content) = crate::fs::vfs::VFS.lock().read_file(filename) {
            if let Ok(text) = core::str::from_utf8(&content) {
                println!("{}", text);
            } else {
                println!("[ERR] Binary content cannot be displayed as UTF-8");
            }
        } else {
            println!("File not found: '{}'", filename);
        }
    }

    fn cmd_echo(&self, args: &[&str]) {
        let output = args.join(" ");
        println!("{}", output);
    }

    fn cmd_ver(&self) {
        println!("Zenith OS v0.5.0 (Phase 5 Interactive Shell Active)");
        println!("Copyright (c) 2026 GarvZenith. Open Source MIT License.");
    }

    fn cmd_clear(&self) {
        // Clear VGA screen
        for _ in 0..25 {
            println!("");
        }
    }
}
