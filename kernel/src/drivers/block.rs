use alloc::vec;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub const SECTOR_SIZE: usize = 512;
pub const TOTAL_SECTORS: usize = 1024; // 512 KiB storage

pub struct RamBlockDevice {
    data: Vec<u8>,
}

impl RamBlockDevice {
    pub fn new() -> Self {
        RamBlockDevice {
            data: vec![0u8; SECTOR_SIZE * TOTAL_SECTORS],
        }
    }

    pub fn read_sector(&self, lba: usize, buf: &mut [u8; SECTOR_SIZE]) -> Result<(), &'static str> {
        if lba >= TOTAL_SECTORS {
            return Err("Sector index out of bounds");
        }
        let offset = lba * SECTOR_SIZE;
        buf.copy_from_slice(&self.data[offset..offset + SECTOR_SIZE]);
        Ok(())
    }

    pub fn write_sector(&mut self, lba: usize, buf: &[u8; SECTOR_SIZE]) -> Result<(), &'static str> {
        if lba >= TOTAL_SECTORS {
            return Err("Sector index out of bounds");
        }
        let offset = lba * SECTOR_SIZE;
        self.data[offset..offset + SECTOR_SIZE].copy_from_slice(buf);
        Ok(())
    }
}

lazy_static! {
    pub static ref BLOCK_DEVICE: Mutex<RamBlockDevice> = Mutex::new(RamBlockDevice::new());
}
