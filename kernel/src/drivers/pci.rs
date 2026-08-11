use crate::{println, serial_println};
use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
}

fn pci_read_u32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    let mut addr_port = Port::<u32>::new(0xCF8);
    let mut data_port = Port::<u32>::new(0xCFC);

    unsafe {
        addr_port.write(address);
        data_port.read()
    }
}

pub fn scan_pci_bus() {
    println!("[PCI BUS] Scanning PCI hardware devices...");
    serial_println!("[PCI BUS] Scanning PCI hardware devices...");

    let mut count = 0;
    for bus in 0..16 {
        for slot in 0..32 {
            let val = pci_read_u32(bus, slot, 0, 0);
            let vendor_id = (val & 0xFFFF) as u16;
            let device_id = ((val >> 16) & 0xFFFF) as u16;

            if vendor_id != 0xFFFF {
                let class_val = pci_read_u32(bus, slot, 0, 0x08);
                let class_code = ((class_val >> 24) & 0xFF) as u8;
                count += 1;

                println!(
                    "  -> PCI [{:02x}:{:02x}.0] Vendor: 0x{:04x}, Device: 0x{:04x}, Class: 0x{:02x}",
                    bus, slot, vendor_id, device_id, class_code
                );
                serial_println!(
                    "  -> PCI [{:02x}:{:02x}.0] Vendor: 0x{:04x}, Device: 0x{:04x}, Class: 0x{:02x}",
                    bus, slot, vendor_id, device_id, class_code
                );
            }
        }
    }
    println!("[PCI BUS] Discovered {} PCI hardware devices.", count);
    serial_println!("[PCI BUS] Discovered {} PCI hardware devices.", count);
}
